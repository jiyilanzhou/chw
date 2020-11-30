use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time;
use std::io::{self, Read, Write};
/* (mut stream:TcpStream) 与  (stream:&mut TcpStream) 的区别[???]
    为何没有 (stream : mut TcpStream) 的表示方法
    是否又有 (&mut stream : TcpStram) 的表示方法
 */
fn handle_client(mut stream: TcpStream) -> io::Result<()> {
    let mut buf = [0; 512];
    let mut count = 0;
    while count < 100 { // 服务器应永真循环(只有主动关闭或断开才出循环)
        let bytes_read = stream.read(&mut buf)?;
        if bytes_read == 0 {
            return Ok(());
        }
        stream.write(&buf[..bytes_read])?;
        thread::sleep(time::Duration::from_secs(1));
        count += 1;
    }
    Ok(())
}

fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    let mut thread_vec: Vec<thread::JoinHandle<()>> = Vec::new();

    for stream in listener.incoming() {
        let stream = stream.expect("failed");
        let handle = thread::spawn(move || {
            // " unwrap_or_else(|error|{})" : 对 error 的处理
            handle_client(stream).unwrap_or_else(|error| eprintln!("{:?}", error));
        });
        thread_vec.push(handle);
    }

    for handle in thread_vec {
        handle.join().unwrap();
    }
    Ok(())
}
