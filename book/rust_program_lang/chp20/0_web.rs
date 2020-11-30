
/*
0. 最后的项目：构建多线程 Web 服务器
    a. 学习一些有关 TCP 和 HTTP 的知识
    b. 在套接字(socket)上监听 TCP 连接
    c. 解析少量的 HTTP 请求
    d. 创建一个合适的 HTTP 响应
    e. 使用线程池改进服务器的吞吐量

1. 构建单线程 Web 服务器
    a. 监听 TCP 连接
       TcpListener 上的 incoming 方法会返回一个产生流序列的迭代器(更准确
       说是 TcpStream 类型的流)。单个流(stream)代表了一个在客户端和服务器
       之间打开的连接，而连接(connection)则代表了客户端连接服务器、服务器
       生成响应以及服务器关闭连接的全部请求与响应的过程。
    b. 读取请求
       // 从 TcpStream 中读取
       let mut buffer = [0; 512];   // 声明缓冲区
       stream.read(&mut buffer).unwrap();// 读取数据并将其存储至缓冲区
       // 字节转为字符串 (from_utf8_lossy 接收 &[u8] 并产生对应的 String
       println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
    c. 响应 HTML
       // 读取文件内容
       let contents = std::fs::read_to_string("hello.html").unwrap();
       // 使用 format! 将文件内容加入到响应 body
       let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);
       // stream.write 方法只接收 &[u8] 故使用 as_bytes() 将字符转为字节
       stream.write(response.as_bytes()).unwrap();
       // flush 调用会等待并阻止程序继续运行直到所有字节被写入连接中(为减少
       // 对底层操作系统的调用，TcpStream 的实现中包含了一个内部缓冲区)
       stream.flush().unwrap();

2. 使用线程池改进吞吐量
    a. 线程池(thread pool)是一组预先分配出来的线程，它们被用于等待并随时处理
       可能的任务。当线程完成任务后使其变为空闲状态并放回线程池，以等待新任务。
    b. 创建用于存放线程的空间 (p638[?])
       " F: FnOnce() -> T + Send + 'static " ：因 T 为" () "故可省略? 即
       " F: FnOnce() + Send + 'static "中" FnOnce()->T "是函数 trait 特殊
        用法。 T 为闭包返回值类型那么" FnOnce()->T "为闭包 trait 类型声明(
        [自]因 Fn 系列系统也是 Trait 但其未能表示参数列表及返回值故使用特殊
        " FnXx(prm) -> T "语法表示 Fn 系列 Trait)
        // 由于线程池中的闭包仅被用来处理连接而无返回值故 T 为单元类型" () "
    c. loop 与 while let
       (0). loop
            loop {
               /* 调用 recv 可从通道中接收任务但会阻塞当前线程，当通道中不存在任务时，
                  当前线程就会一直处于等待状态。而 Mutex<T> 则保证一次只有一个 worker
                  线程尝试请求任务。
                  通过使用 loop 并且在循环代码块内部而不是外部请求锁和任务则 lock 方法
                  中返回的 MutexGuard 会在 let job 语句结束后被立即丢弃。确保了只会在
                  调用 recv 的过程中持有锁，并能够在调用 job() 之前将锁释放,因此服务器
                  才可同时响应多个请求。
               */
               let job = receiver.lock().unwrap().recv().unwrap();
               println!("Worker {} got a job; executing.", id);
               job();
            }
       (1). while let
            /* 此段使用 while let 的代码不会产生预期的行为：一个慢请求依旧会导致
               其它请求被阻塞等待。其原因有些微妙：Mutex 结构体不存在公开 unlock
               方法，因为锁的所有权依赖于 MutexGuard<T> 的生命周期，而代码只能在
               lock 方法返回的 LockResult<MutexGuard<T>> 中得到它。这使得编译器
               能够在编译过程中确保只有在持有锁时才能访问由 Mutex 守护的资源。若
               未妥当设计好 MutexGuard<T> 的生命周期则其实现可能意外地逾期持有锁。
               在本例中，由于 while 表达式内的值会把整个代码块视作自己的作用域,故
               在调用 job() 的过程中仍然持有锁，这也意味着其它工作线程无法正常地
               接收任务。
               而通过使用 loop 并且在循环代码块内部而不是外部请求锁和任务则 lock
               方法中返回的 MutexGuard 会在 let job 语句结束后被立即丢弃。确保了
               只会在调用 recv 的过程中持有锁，并能够在调用 job() 之前将锁释放，
               因此服务器才可同时响应多个请求。
            */
            while let Ok(job) = receiver.lock().unwrap().recv() {
                println!("Worker {} got a job; executing.", id);
                job();
            }

3. 优雅地停机与清理
    a. 为 ThreadPool 实现 Drop trait
       所有线程都应当在线程池被丢弃时调用 join 从而确保它们能够在结束前完成自己的
       工作(译者注：join 方法会阻塞当前的线程，并持续到它所属的线程执行完毕为止且
       " thread::JoinHandler<()> "的 join 方法要求获取参数的所有权故可用 Option
       来包裹，即" Option<thread::JoinHandler<()>> ",而后可用 take 将 Some 变体
       移出而在原位置留下 None 变体。
    b. 通知线程停止监听任务
       在 drop 方法中实现循环发送通知(send) 及 遍历阻塞等待(join)

===============================================================================
                                附  录

附录 A : 关键字
    关键字
    保留关键字
    原始标识符

附录 B : 运算符和符号
    运算符
    非运算符符号

附录 C : 可派生 trait (p673[?])
   a. Display 就是一个典型的不可派生 trait
   b. 面向程序员格式输出的 Debug
   c. 用于相等性比较的 PartialEq 及 Eq
   d. 使用 PartialOrd 和 Ord 进行次序比较
   e. 使用 Clone 和 Copy 复制值
   f. 用于将值映射到另外一个长度固定的值的 Hash
   g. 用于提供默认值的 Default
      Default trait 允许为某类型创建默认值
      ( Default::default 常组合用于结构体更新)

附录 D ：有用的开发工具
    a. 使用 rustfmt 自动格式代码
    b. 使用 rustfix 修复代码
    c. 使用 Clippy 完成更多的代码分析
    d. 使用 Rust 语言服务器来集成 IDE

附录 E ：版本
    Rust 语言与编译器每 6 周发布(可持续获取功能更新)

*/