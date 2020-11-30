
/*
1、建立 TcpListener
2、读取请求内容
3、编写响应
    (1) 返回一个响应行
    (2) 返回一个真正的网页
    (3) 有件的返回网页

4. HTTP 简单介绍
（1）http 请求报文包含三部分内容:请求行、请求头、请求体
     Method Request-URI Http-Version CRLF  // 请求行:请求方式、协议版本等
     headers CRLF    // 请求头:包含若干个属性，格式为"属性名:属性值"，服务端据此获取客户端的信息
     message-body    // 请求体:客户端真正要传送给服务端的内容
（2）http 响应报文也有三部分内容:响应行、响应头、响应体
    HTTP-Version Status-Code Reason-Phrase CRLF  //响应行:报文协议及版本，状态码及状态描述;
    headers CRLF        // 响应头:由多个属性组成
    message-body        // 响应体:真正响应的内容

*/
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::fs;

fn handle_client(mut stream: TcpStream) {
    // 读取请求行至 buffer
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    // 将字节转为字符串
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
    // 判断开始标识
    let get = b"GET / HTTP/1.1\r\n";
    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "main.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };
    // 回写
    let contents = fs::read_to_string(filename).unwrap();
    let reponse = format!("{}{}", status_line, contents);
    stream.write(reponse.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    for stream in listener.incoming() {
        handle_client(stream?);
    }
    Ok(())
}