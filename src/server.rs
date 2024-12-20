use std::collections::HashMap; // 引入标准库中的 HashMap，用于存储用户信息
use std::sync::Mutex; // 引入 Mutex，用于在多线程环境下安全地共享数据
use num_bigint::BigUint; // 引入大整数类型 BigUint，处理超大数字
use tonic::{transport::Server, Code, Request, Response, Status}; // 引入 Tonic 的 gRPC 相关模块，处理 gRPC 请求和响应

use zkp_chaum_pedersen::ZKP; // 引入 ZKP 模块，用于实现 Chaum-Pedersen 零知识证明协议

// 引入生成的 gRPC 代码模块
pub mod zkp_auth {
    // 包含 gRPC 服务和消息类型的定义，定义是在 .proto 文件中生成的代码
    include!("./zkp_auth.rs");
}

// 使用生成的 gRPC 服务和消息结构体
use zkp_auth::{
    auth_server::{Auth, AuthServer}, // 引入 Auth 服务接口和 AuthServer 实现，用于 gRPC 服务器的创建
    AuthenticationAnswerRequest, AuthenticationAnswerResponse, // 验证认证时的请求和响应消息类型
    AuthenticationChallengeRequest, AuthenticationChallengeResponse, // 创建认证挑战的请求和响应消息类型
    RegisterRequest, RegisterResponse // 注册功能的请求和响应消息类型
};

// 定义一个结构体 AuthImpl，用于实现 gRPC 服务
#[derive(Debug, Default)] // 派生 Debug 和 Default 宏，生成结构体的调试输出和默认构造器
pub struct AuthImpl {
    user_info: Mutex<HashMap<String, UserInfo>>, // 使用 Mutex 保护 HashMap，存储用户信息以确保线程安全
    auth_id_to_user: Mutex<HashMap<String, String>>, // 保存认证 ID 到用户名的映射，方便后续认证流程
}

// 定义一个结构体 UserInfo，用于存储用户相关信息
#[derive(Debug, Default)] // 为 UserInfo 结构体实现 Debug 和 Default 特性
struct UserInfo {
    pub user_name: String, // 用户名
    pub y1: BigUint, // 大整数 y1，用户注册时传递的验证数据
    pub y2: BigUint, // 大整数 y2，用户注册时传递的验证数据
    pub r1: BigUint, // 认证时使用的随机数 r1
    pub r2: BigUint, // 认证时使用的随机数 r2
    pub c: BigUint, // 验证时的挑战值 c
    pub s: BigUint, // 验证时的响应值 s
    pub session_id: String, // 用户会话的 session_id
}

// 实现 gRPC 服务的接口，这里实现的是 Auth 服务接口
#[tonic::async_trait] // 使用 async_trait 宏将异步函数声明为 Tonic 异步 gRPC 服务
impl Auth for AuthImpl {
    // 实现注册功能，接收 RegisterRequest 并返回 RegisterResponse
    async fn register(&self, request: Request<RegisterRequest>) -> Result<Response<RegisterResponse>, Status> {
        println!("Processing Register: {:?}", request); // 打印收到的注册请求，方便调试

        let request = request.into_inner(); // 将 gRPC 请求解包，提取请求消息

        let user_name = request.user.clone(); // 从请求中获取用户名

        let mut user_info = UserInfo::default(); // 创建一个默认的 UserInfo 实例
        user_info.user_name = user_name.clone(); // 存储用户名
        user_info.y1 = BigUint::from_bytes_be(&request.y1); // 将请求中的 y1 字节数组转换为 BigUint 类型
        user_info.y2 = BigUint::from_bytes_be(&request.y2); // 将请求中的 y2 字节数组转换为 BigUint 类型

        // 获取 user_info 哈希表的锁，将用户信息插入其中
        let user_info_hashmap = &mut self.user_info.lock().unwrap();
        user_info_hashmap.insert(user_name, user_info); // 将用户信息存储在哈希表中

        // 返回一个空的 RegisterResponse，表示注册成功
        Ok(Response::new(RegisterResponse {}))
    }

    // 实现创建认证挑战的功能，接收 AuthenticationChallengeRequest 并返回 AuthenticationChallengeResponse
    async fn create_authentication_challenge(&self, request: Request<AuthenticationChallengeRequest>) -> Result<Response<AuthenticationChallengeResponse>, Status> {
        println!("Processing Challenge: {:?}", request); // 打印收到的认证挑战请求，便于调试

        let request = request.into_inner(); // 解包 gRPC 请求，获取请求消息
        let user_name = request.user; // 从请求中获取用户名

        let user_info_hashmap = &mut self.user_info.lock().unwrap(); // 获取用户信息哈希表的锁

        // 如果用户存在于哈希表中，则生成认证挑战
        if let Some(user_info) = user_info_hashmap.get_mut(&user_name) {
            let (_, _, _, q) = ZKP::get_constants(); // 获取 ZKP 常量

            let c = ZKP::generate_random_number_below(&q); // 生成小于 q 的随机数作为挑战值
            let auth_id = ZKP::generate_random_string(12); // 生成 12 位随机字符串作为认证 ID

            user_info.c = c.clone(); // 将挑战值 c 存储在用户信息中
            user_info.r1 = BigUint::from_bytes_be(&request.r1);
            user_info.r2 = BigUint::from_bytes_be(&request.r2);


            let auth_id_to_user = &mut self.auth_id_to_user.lock().unwrap(); // 获取认证 ID 到用户的映射表锁
            auth_id_to_user.insert(auth_id.clone(), user_name); // 将认证 ID 映射到对应的用户名

            // 返回认证挑战响应，包含生成的认证 ID 和挑战值 c
            Ok(Response::new(AuthenticationChallengeResponse { auth_id, c: c.to_bytes_be() }))
        } else {
            // 如果用户不存在，返回 NotFound 错误
            Err(Status::new(Code::NotFound, format!("User: {} not found in database", user_name)))
        }
    }

    // 实现认证验证功能，接收 AuthenticationAnswerRequest 并返回 AuthenticationAnswerResponse
    async fn verify_authentication(&self, request: Request<AuthenticationAnswerRequest>) -> Result<Response<AuthenticationAnswerResponse>, Status> {
        println!("Processing Verification: {:?}", request); // 打印收到的认证验证请求，便于调试

        let request = request.into_inner(); // 解包 gRPC 请求，获取请求消息
        let auth_id = request.auth_id; // 从请求中获取认证 ID

        let auth_id_to_user_hashmap = &mut self.auth_id_to_user.lock().unwrap(); // 获取认证 ID 到用户映射表的锁

        // 如果认证 ID 存在，进行认证验证
        if let Some(user_name) = auth_id_to_user_hashmap.get(&auth_id) {
            let user_info_hashmap = &mut self.user_info.lock().unwrap(); // 获取用户信息哈希表的锁
            let user_info = user_info_hashmap.get_mut(user_name).expect("AuthId not found on Hashmap");

            let s = BigUint::from_bytes_be(&request.s); // 将请求中的 s 字节数组转换为 BigUint 类型

            let (alpha, beta, p, q) = ZKP::get_constants(); // 获取 ZKP 常量
            let zkp = ZKP { alpha, beta, p, q }; // 创建 ZKP 实例

            // 验证用户提交的解答是否有效
            let verification = zkp.verify(&user_info.r1, &user_info.r2, &user_info.y1, &user_info.y2, &user_info.c, &s);

            if verification {
                // 如果验证通过，生成一个新的会话 ID
                let session_id = ZKP::generate_random_string(12);
                Ok(Response::new(AuthenticationAnswerResponse { session_id }))
            } else {
                // 验证失败，返回权限拒绝错误
                Err(Status::new(Code::PermissionDenied, format!("AuthId: {} bad solution to the challenge", auth_id)))
            }
        } else {
            // 如果认证 ID 不存在，返回 NotFound 错误
            Err(Status::new(Code::NotFound, format!("AuthId: {} not found in database", auth_id)))
        }
    }
}

// 主函数，运行 gRPC 服务器
#[tokio::main] // 使用 tokio 运行时来处理异步任务
async fn main() {
    // 定义服务器监听的地址和端口号
    let addr = "127.0.0.1:50051".to_string();
    println!("Running the server in {}", addr); // 打印服务器运行地址，方便调试

    // 创建 AuthImpl 实例，作为 gRPC 服务的实现
    let auth_impl = AuthImpl::default();

    // 构建并启动 gRPC 服务器
    Server::builder() // 创建一个 gRPC 服务器构建器
        .add_service(AuthServer::new(auth_impl)) // 将 Auth 服务添加到 gRPC 服务器中
        .serve(addr.parse().expect("could not convert address")) // 开始监听指定的地址和端口，并处理可能的错误
        .await.unwrap(); // 异步运行服务器，使用 unwrap 处理可能的错误
}