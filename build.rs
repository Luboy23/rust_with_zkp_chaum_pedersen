fn main() {
    // 配置 tonic_build 来生成 gRPC 服务器代码
    tonic_build::configure()
        // 设置为生成服务器端代码。Tonic 可以生成客户端和服务器端代码，
        // 这里我们只需要服务器端的实现。
        .build_server(true)
        // 指定生成的 Rust 文件的输出目录
        .out_dir("src/")
        // 调用 compile 函数来编译指定的 .proto 文件，并生成相应的 Rust 代码
        // 第一个参数是需要编译的 .proto 文件的路径列表
        .compile(
            &["proto/zkp_auth.proto"], // 需要编译的 .proto 文件
            &["proto/"], // 该 .proto 文件所在的目录
        )
        // 使用 unwrap() 确保编译成功，如果编译失败则引发 panic
        .unwrap();
}