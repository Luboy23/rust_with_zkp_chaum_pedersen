# 使用 Rust 官方镜像
FROM rust:1.77.0

# 设置工作目录
WORKDIR /zkp-server

# 复制项目的所有文件到工作目录中
COPY . .

# 安装 protobuf 编译器和其他依赖
RUN apt update 
RUN apt install -y protobuf-compiler 

# 构建项目，指定 server 和 client 两个二进制文件
RUN cargo build --release --bin server --bin client && ls target/release/
# 设置环境变量（例如用户名）
ENV USER=lllu_23
