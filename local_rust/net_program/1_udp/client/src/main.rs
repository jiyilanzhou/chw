
/*
0. UDPClient:

 */
use std::net::UdpSocket;
use std::{io, str};

fn main() -> io::Result<()> {
    // 绑定一个客户端自己的端口
    let socket = UdpSocket::bind("127.0.0.1:8000")?;
    // 连接服务器( IP + Port )
    socket.connect("127.0.0.1:8080")?;

    loop{
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        socket.send(input.as_bytes())?;

        // 接收数据
        let mut buffer = [0u8; 1500];
        // 接收的长度及对端地址
        // let (_size, _src) = socket.recv_from(&mut buffer)?;
        socket.recv_from(&mut buffer)?;

        println!("recv: {}", 
            str::from_utf8(&buffer).expect("Could not write buffer as string"));
    }
}
