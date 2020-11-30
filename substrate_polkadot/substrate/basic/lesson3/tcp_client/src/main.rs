
/* 一般开发为使得代码更加简洁可使用" use "导入依赖包：
       显式将其引入作用域如" use std::net::{TcpServer,TceStream} "
    // 注：源于更加明确代码含义：此示例使用全路径表示法
 */
// " std::net::TcpStream "为全路径表示法(后续代码皆如此[不再赘述])
fn handle_client(mut stream: std::net::TcpStream) {
    // 读取请求至 buffer
    let mut buf = [0; 512]; // 声明 buffer 类型及大小
    // 读取结果并处理 Result (为简化代码：后续直接使用" unwrap "处理 Result )
    match std::io::Read::read(&mut stream, &mut buf) {
        Ok(_) => (),
        Err(err) => {
            println!("This was a problem reading the stream:{:?}", err);
        }
    };
    // 输出请求信息
    println!("Request: {}", String::from_utf8_lossy(&buf[..]));
    // 判断请求(此处仅处理 GET 请求)
    let req_line = b"GET / HTTP/1.1\r\n"; // b"xxx" 表示将&str转换为字节
    // 使用模式匹配解构元组 (status_line, filename)
    let (status_line, filename) = if buf.starts_with(req_line) {
        ("HTTP/1.1 200 OK\r\n\r\n", "index.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };
    // 读取文件内容(此处为简化示例直接使用" unwrap "：实际开发中务必对 Result 进行处理)
    let content = std::fs::read_to_string(filename).unwrap();
    // 拼接响应数据
    let res = format!("{}{}", status_line, content);

    // 方式 1 ：回写数据(同理为简化示例直接使用" unwrap "：实际开发中务必对 Result 进行处理)
    std::io::Write::write(&mut stream, res.as_bytes()).unwrap();
    // 刷出缓存(将回写的数据响应到请求客户端)
    std::io::Write::flush(&mut stream).unwrap();
    /* // 使用完全限定语法：
       <std::net::TcpStream as std::io::Write>::write(&mut stream,res.as_bytes()).unwrap();
       <std::net::TcpStream as std::io::Write>::flush(&mut stream).unwrap();
     */

    /* 方式 2 ：用全路径须导入" use std::io::Write; "否则出现" method not found in
               `std::net::TcpStream` "的错误，因为 write、flush 方法都是 TcpStream
               实现 Write 的方法。所以未导入 Write Trait 且 TcpStream 本身亦未实现
               相应的 write、flush 方法故肯定找不到。
       其 `std::net::TcpStream`  源码如下:
               // 实现 Write 的部分方法
               impl Write for TcpStream {
                    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
                        self.0.write(buf)
                    }
                    fn flush(&mut self) -> io::Result<()> {
                        Ok(())
                    }
                }
               // 而其 TcpStream 本身未曾实现相应的 write 及 flush 方法
               impl TcpListener {
                    // 其内未曾实现 write 及 flush 方法
               }
              // 通过源码分析可知必须导入" use std::io::Write; "的原因
    */
    // use std::io::Write; // 未导入则其下两行代码找不到相应的" write、flush "方法
    // stream.write(res.as_bytes()).unwrap();
    // stream.flush().unwrap();

    // 睡眠一段时间(防止浏览器重复发送请求，从而覆盖网页显示效果)
    std::thread::sleep(std::time::Duration::from_millis(10000));
}

// 主函数使用返回" std::io::Result "(用于匹配 ? 错误处理)
fn main() -> std::io::Result<()> {
    // 绑定 IP 及 端口
    let listener = std::net::TcpListener::bind("127.0.0.1:8080")?;
    // 创建线程容器
    let mut thread_vec: Vec<std::thread::JoinHandle<()>> = Vec::new();
    // 循环监听
    for stream in listener.incoming() {
        // 多线程(传入闭包)处理请求
        let stream = stream.unwrap();
        // 使用 move 关键字将 stream 移入闭包(即获取 stream 所有权)
        let thread_handle = std::thread::spawn(move || {
            // 业务处理
            handle_client(stream)
        });
        // 将操作句柄追加至线程容器
        thread_vec.push(thread_handle)
    }
    // 等待线程结束 join
    for handle in thread_vec {
        handle.join().unwrap_err();
    }
    Ok(())
}