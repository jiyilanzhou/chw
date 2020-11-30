use std::net::TcpStream;
use std::io::{ prelude::*, BufReader, Write };
use std::str;
use std::thread;

use futures;

fn use_server(server: &str, port: u16, content: &str) -> std::io::Result<()> {
    let mut stream = TcpStream::connect((server, port))?;
    let _ = stream.write(content.as_bytes())?;
    // 等待回写内容
    let mut reader = BufReader::new(&stream);
    let mut buffer: Vec<u8> = Vec::new();
    // 读取到缓冲区(模拟下载)
    reader.read_until(b'\n', &mut buffer)?;

    println!("recv from server: {} ", str::from_utf8(&buffer).unwrap());
    Ok(())
}

// 迭代一：同步
fn main_0() -> std::io::Result<()> {
    use_server("127.0.0.1", 8080, "use server1 download 127.0.0.1:8080")?;
    use_server("127.0.0.1", 8081, "use server2 download 127.0.0.1:8081")?;
    Ok(())
}

// 迭代二：多线程(异步[弊端：线程数量无限制且开启线程代码重复度高、开销大])
fn main_1() -> std::io::Result<()> {
    let mut handles: Vec<thread::JoinHandle<()>> = Vec::new();
    // 启动线程 1
    let handle = thread::spawn(move || {
        use_server("127.0.0.1", 8080, "use server1 download 127.0.0.1:8080").unwrap();
    });
    handles.push(handle);
    // 启动线程 2
    let handle = thread::spawn(move || {
        use_server("127.0.0.1", 8081, "use server2 download 127.0.0.1:8081").unwrap();
    });
    handles.push(handle);
    // 等待处理完毕
    for handle in handles {
        handle.join().unwrap();
    }
    Ok(())
}

// 使用异步编程(使用单线程达到多线程效果：减少开销)
async fn async_use_server(server: &str, port: u16, content: &str) {
    use_server(server, port, content).unwrap();
}
async fn use_all_server() {
    let f1 = async_use_server("127.0.0.1", 8080, "use server1 download 127.0.0.1:8080");
    let f2 = async_use_server("127.0.0.1", 8081, "use server2 download 127.0.0.1:8081");
    futures::join!(f1, f2);
}

// 执行效果未达到异步预期：(执行效果与迭代 1 同步效果相同：原因待理解[???])
fn main() -> std::io::Result<()> {
    let f = use_all_server();
    futures::executor::block_on(f);
    Ok(())
}