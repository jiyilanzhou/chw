
fn handle_client(mut stream: std::net::TcpStream) {
    // 读取请求行至 buffer
    let mut buf = [0; 512];
    std::io::Read::read(&mut stream, &mut buf).unwrap();
    // 判断请求
    let req_line = b"GET / HTTP/1.1\r\n";
    let (status_line, filename) = if buf.starts_with(req_line) {
        ("HTTP/1.1 200 OK\r\n\r\n", "main.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };
    // 读取文件
    let content = std::fs::read_to_string(filename).unwrap();
    let res = format!("{}{}", status_line, content);
    // 回写数据
    std::io::Write::write(&mut stream, res.as_bytes()).unwrap();
    std::io::Write::flush(&mut stream);

    /* 单线程存在的问题: 如"输出后睡眠"
       则后一个请求的处理需在前一个请求结束后方能开始着手
       原因分析:" listener.incoming() "阻塞监听
       解决方案：创建多线程处理
    */
    let tm = std::time::Duration::from_millis(10000);
    std::thread::sleep(tm);
}

fn main_0() -> std::io::Result<()> {
    // 绑定
    let listener = std::net::TcpListener::bind("127.0.0.1:8080")?;
    // 循环监听
    for stream in listener.incoming() {
        // 单线程处理客户端请求
        //handle_client(stream?);

        // 多线程(传入闭包)处理请求
        let stream = stream.unwrap();
        std::thread::spawn(move || {
            handle_client(stream)
        });
    }
    Ok(())
}

fn main() -> std::io::Result<()> {
    // 绑定
    let listener = std::net::TcpListener::bind("127.0.0.1:8080")?;
    // 创建线程容器
    let mut thread_vec: Vec<std::thread::JoinHandle<()>> = Vec::new();
    // 循环监听
    for stream in listener.incoming() {
        // 多线程(传入闭包)处理请求
        let stream = stream.unwrap();
        let thread_handle = std::thread::spawn(move || {
            handle_client(stream)
        });
        thread_vec.push(thread_handle)
    }
    // 等待线程结束 join
    for handle in thread_vec{
        handle.join().unwrap_err();
    }
    Ok(())
}