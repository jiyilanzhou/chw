
/*
0. 异步框架 Tokio (参阅" https://github.com/tokio-rs/tokio ")特性：
    a. 一个非常流行、爆强的、事件驱动的非阻塞 io 异步编程框架。
       (很知名 hyper 和 actix 都是基于 tokio 实现)
    b、多线程调度、利用多核
    c、非阻塞 io 模型。 Linux 使用 epoll 模型，bsd 使用 kqueue 模型，
       win 里面是 IOCP 模型
    d. 开箱即用。提供异步语法

1. 添加依赖
    [dependencies]
    futures = "0.3"
    tokio = {version = "0.2",features = ["full"]}

2. 传统 IO
    a. 例如有 TCP/UDP 连接和传输，读取和写入文件
    b. RUST 标准库提供的都是阻塞的，而 Tokio 提供了异步版本
    c. 最为经典的就是做个异步的 webserver

3. 使用标准库实现 webserver

 */
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

fn main() {
    // 绑定任意地址
    let server = TcpListener::bind("0.0.0.0:8080").unwrap();
    /*
        // 仅接受单次请求
        let stream = server.accept().unwrap();
        handler(stream.0);
     */
    for stream in server.incoming(){
        handler(stream.unwrap());
    }
}

fn handler(mut stream: TcpStream) {
    let resp = String::from("http/1.1 200 OK\r\n\r\n Rust Server...");
    stream.write(resp.as_bytes()).unwrap();
    // stream.flush();
}