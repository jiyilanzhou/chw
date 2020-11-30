use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;
use std::time;

fn handle_client(mut stream: TcpStream, wait_time: u64) -> std::io::Result<()> {
    let mut buf = [0; 512];
    loop {
        // 读取流中的数据并将其存储至缓冲区
        let bytes_read = stream.read(&mut buf)?;
        // 此为模仿下载过程(若为聊天室，则此处退出条件应为超时或主动断开链接[???])
        if bytes_read == 0 {
            return Ok(());
        }
        thread::sleep(time::Duration::from_secs(wait_time));
        stream.write(&buf[..bytes_read])?;
        stream.write(&("\n".as_bytes()))?;
    }    
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;

    for stream in listener.incoming() {
        handle_client(stream?, 3)?;
    }

    Ok(())
}
