
/*
0. 客户端：

 */
use std::io::{self, prelude::*, BufReader, Write};
use std::net::TcpStream;
use std::str;

fn main() -> io::Result<()> {
    // 发起请求连接
    let mut stream = TcpStream::connect("127.0.0.1:8080")?;
    for _ in 0..10 {
        // 发送请求
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read");
        stream.write(input.as_bytes()).expect("Failed to write");

        // 读取请求
        let mut reader = BufReader::new(&stream);
        let mut buffer: Vec<u8> = Vec::new();
        // 读取到换行
        reader.read_until(b'\n', &mut buffer).expect("Failed to read into buffer");
        println!("read  from server: {}", str::from_utf8(&buffer).unwrap());
        println!("");
    }
    println!("Hello, world!");
    Ok(())
}
