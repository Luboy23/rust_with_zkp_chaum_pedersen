use tonic::{transport::Server, Code, Request, Response, Status};

// 引入生成的 gRPC 代码模块
pub mod zkp_auth {
    // 包含 gRPC 服务和消息类型的定义
    include!("./zkp_auth.rs");
}

// 使用生成的 gRPC 服务和消息结构体
use zkp_auth::{
    auth_server::{Auth, AuthServer}, // 引入 Auth 服务接口和 AuthServer 实现
    AuthenticationAnswerRequest, AuthenticationAnswerResponse, // 认证答案请求和响应的消息类型
    AuthenticationChallengeRequest, AuthenticationChallengeResponse, // 认证挑战请求和响应的消息类型
    RegisterRequest, RegisterResponse // 注册请求和响应的消息类型
};

// 定义一个结构体用于实现 gRPC 服务
#[derive(Debug, Default)]
struct AuthImpl {
}

// 实现 gRPC 服务的接口，这里实现的是 Auth 服务接口
#[tonic::async_trait]
impl Auth for AuthImpl {
    // 实现注册功能。接收 RegisterRequest 并返回 RegisterResponse
    async fn register(&self, request: Request<RegisterRequest>) -> Result<Response<RegisterResponse>, Status> {
        todo!() // todo! 是一个占位符，表示该功能未实现
    }

    // 实现创建认证挑战功能。接收 AuthenticationChallengeRequest 并返回 AuthenticationChallengeResponse
    async fn create_authentication_challenge(&self, request: Request<AuthenticationChallengeRequest>) -> Result<Response<AuthenticationChallengeResponse>, Status> {
        todo!() // 未实现的功能
    }

    // 实现验证认证功能。接收 AuthenticationAnswerRequest 并返回 AuthenticationAnswerResponse
    async fn verify_authentication(&self, request: Request<AuthenticationAnswerRequest>) -> Result<Response<AuthenticationAnswerResponse>, Status> {
        todo!() // 未实现的功能
    }
}

// 主函数，运行 gRPC 服务器
#[tokio::main] // 使用 tokio 运行时执行异步代码
async fn main() {
    // 服务器监听的地址和端口号
    let addr = "127.0.0.1:50051".to_string();
    println!("Running the server in {}", addr); // 打印服务器运行地址

    // 创建服务的实现实例
    let auth_impl = AuthImpl::default();

    // 构建并启动 gRPC 服务器
    Server::builder() // 创建 gRPC 服务器构建器
        // 将 Auth 服务添加到服务器
        .add_service(AuthServer::new(auth_impl)) 
        // 开始监听指定的地址
        .serve(addr.parse().expect("could not convert address")) 
        // 运行服务器并处理可能发生的错误
        .await.unwrap(); // unwrap 用于处理运行时的错误，如果有错误则会 panic
}