use std::io::stdin; // 引入标准库中的 stdin 模块，用于从终端读取用户输入
use num_bigint::BigUint; // 引入 num_bigint 库中的 BigUint 类型，用于处理大整数

// 引入生成的 gRPC 代码模块
pub mod zkp_auth {
    // 包含 gRPC 服务和消息类型的定义，定义是在 .proto 文件中生成并自动生成的代码
    include!("./zkp_auth.rs");
}

// 引入 gRPC 客户端和认证/注册请求消息类型
use zkp_auth::{auth_client::AuthClient, AuthenticationAnswerRequest, AuthenticationChallengeRequest, RegisterRequest}; 
use zkp_chaum_pedersen::ZKP; // 引入实现 Chaum-Pedersen 零知识证明协议的库 ZKP

#[tokio::main] // 使用 tokio 宏，用于定义异步主函数
async fn main() { // 定义异步主函数，程序的入口点

    let mut buf = String::new(); // 创建一个空的 String，用于存储用户输入
    let (alpha, beta, p, q) = ZKP::get_constants(); // 调用 ZKP 协议获取常量 alpha、beta、p 和 q
    let zkp = ZKP {alpha: alpha.clone(), beta: beta.clone(), p: p.clone(), q: q.clone()}; // 创建 ZKP 实例，使用上述常量初始化

    // 创建 gRPC 客户端并连接到服务器，连接失败时将抛出错误
    let mut client = AuthClient::connect("http://127.0.0.1:50051").await.expect("could not connect to server");
    println!("Connected to the server"); // 打印连接成功消息

    // 提示用户输入用户名
    println!("Please provide username: ");
    stdin().read_line(&mut buf).expect("Could not get the username from stdin"); // 从终端读取用户输入的用户名
    let username = buf.trim().to_string(); // 去除输入的多余空格并转换为 String
    buf.clear(); // 清空缓冲区，准备下一次输入

    // 提示用户输入密码
    println!("Please provide password: ");
    stdin().read_line(&mut buf).expect("Could not get the password from stdin"); // 从终端读取用户输入的密码
    let password = BigUint::from_bytes_be(buf.trim().as_bytes()); // 将输入的密码转为大整数 BigUint 类型
    buf.clear(); // 清空缓冲区

    println!("Please provide the password (to login):");
    stdin()
        .read_line(&mut buf)
        .expect("Could not get the username from stdin");
    let password = BigUint::from_bytes_be(buf.trim().as_bytes());
    buf.clear();

    // 计算 y1 和 y2，分别为 alpha 和 beta 的密码次方模 p 的结果，使用 Chaum-Pedersen 协议
    let y1 = ZKP::exponentiate(&alpha, &password, &p);
    let y2 = ZKP::exponentiate(&beta, &password, &p);

    // 构建一个注册请求 RegisterRequest，包含用户名和计算得到的 y1 和 y2
    let request = RegisterRequest {
        user: username.clone(), // 用户名
        y1: y1.to_bytes_be(), // 将 y1 转换为字节数组
        y2: y2.to_bytes_be(), // 将 y2 转换为字节数组
    };

    // 向 gRPC 服务器发送注册请求，等待服务器响应，失败时将抛出错误
    let _response = client.register(request).await.expect("could not register");
    println!("{:?}", _response); // 打印服务器的响应结果

    // 创建用于认证的随机数 k，并计算 r1 和 r2
    let k = ZKP::generate_random_number_below(&q); // 生成随机数 k
    let r1 = ZKP::exponentiate(&alpha, &k, &p); // 计算 r1 = alpha^k mod p
    let r2 = ZKP::exponentiate(&beta, &k, &p); // 计算 r2 = beta^k mod p

    // 构建认证挑战请求 AuthenticationChallengeRequest
    let request = AuthenticationChallengeRequest {
        user: username, // 用户名
        r1: r1.to_bytes_be(), // 将 r1 转换为字节数组
        r2: r2.to_bytes_be(), // 将 r2 转换为字节数组
    };

    // 向 gRPC 服务器发送认证挑战请求，等待服务器响应，失败时将抛出错误
    let response = client.create_authentication_challenge(request).await.expect("could not request challenge to user").into_inner();

    // 获取认证挑战的 auth_id 和挑战值 c
    let auth_id = response.auth_id; // 从服务器响应中获取 auth_id
    let c = BigUint::from_bytes_be(&response.c); // 将挑战值 c 从字节数组转换为大整数

    // 计算响应值 s，使用 k、c 和用户密码
    let s = zkp.solve(&k, &c, &password);

    // 构建认证应答请求 AuthenticationAnswerRequest
    let request = AuthenticationAnswerRequest {
        auth_id, // 传递 auth_id
        s: s.to_bytes_be() // 将 s 转换为字节数组
    };

    // 向 gRPC 服务器发送认证应答请求，等待服务器响应，失败时将抛出错误
    let response = client.verify_authentication(request).await.expect("could not verify authentication in server").into_inner();

    // 打印成功登录的消息，并显示 session_id
    println!("You logged in !!! session_id: {}", response.session_id);
}