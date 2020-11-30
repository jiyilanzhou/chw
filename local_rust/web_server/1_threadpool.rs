
// ----------------- ThreadPool_1 -----------------------------------
pub struct ThreadPool_1 {
    // 将线程句柄装入容器
    threads: Vec<std::thread::JoinHandle<()>>
}
impl ThreadPool_1 {
    // 指定线程数量
    pub fn new(size: usize) -> ThreadPool_1 {
        // 校验
        assert!(size > 0);
        // 指定线程容器容量
        let mut threads = Vec::with_capacity(size);
        // 遍历创建线程
        for _ in 0..size {
            let handle = std::thread::spawn(move || {
                // 创建线程时需传入闭包但此时未知具体闭包则如何处理
                // 解决方案:
                // 传入 receiver 且在" execute<F>(&self,f:F) "中传入闭包
            });
            threads.push(handle);
        }
        ThreadPool_1 {
            threads
        }
    }

    /* 参照: " spawn " 其源码：
        pub fn spawn<F, T>(f: F) -> JoinHandle<T>
        where
            F: FnOnce() -> T,
            F: Send + 'static,
            T: Send + 'static,
        {
            Builder::new().spawn(f).expect("failed to spawn thread")
        }
    */
    pub fn execute<F>(&self, f: F)
        where F: FnOnce() + Send + 'static {

    }
}
// ----------------- ThreadPool_2 -----------------------------------
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
// work 需要等待在 channel 的接收端
struct Worker {
    // worker 标号
    id: usize,
    // 线程句柄
    thread: std::thread::JoinHandle<()>,
}
impl Worker {
    /* // 方式 1：
    fn new(id: usize) -> Worker {
        let thread = thread::spawn(move || {
           // 同理存在:
           // 创建线程时需传入闭包但此时未知具体闭包则如何处理?
           // 解决方案:
           // 传入 receiver 且在" execute<F>(&self,f:F) "中传入闭包
        });
        Woker {
            id,
            thread,
        }
    }
    */

    // 方式2：
    // 将管道接收端 receiver 传入创建 worker ，这样 worker 才能在 channel 接收端等待
    // fn new(id:usize,receiver:mpsc::Receiver<Job>)->Worker{ //receiver非线程安全
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = std::thread::spawn(move || {
            // 循环等待 receiver 并处理
            loop {  // 使用 Loop 则 worker 创建后一直循环，永不会结束(待完善)
                // 接收端有多个线程等待故需要" 锁  "(此例设定为" Mutex ")
                // receiver 阻塞等待       recv() 为接收到的内容
                let job = receiver.lock().unwrap().recv().unwrap();
                // println!("Worker {} got a job!", id);
                job();
            }
        });

        Worker {
            id,
            thread,
        }
    }
}

// 线程池
pub struct ThreadPool_2 {
    // 将 worker 装入容器
    workers: Vec<Worker>,
    // 发送者：使用" channel "("Job"为发送的任务)
    sender: mpsc::Sender<Job>,
}

//  Job 为 trait 对象(因为需要在线程间传递)
type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool_2 {
    // 指定线程数量
    pub fn new(size: usize) -> ThreadPool_2 {
        // 校验
        assert!(size > 0);
        // 指定 worker 容器容量
        let mut workers = Vec::with_capacity(size);
        let (sender, receiver) = mpsc::channel();
        let receiver_safe = Arc::new(Mutex::new(receiver)); // 线程安全
        // 创建 worker 并装入容器
        for id in 0..size {
            // workers.push(Worker::new(id, receiver)); // 非线程安全
            workers.push(Worker::new(id, Arc::clone(&receiver_safe)));
        }
        ThreadPool_2 {
            workers,
            sender,
        }
    }

    /* 线程池工作流程:
       a. 主线程调用线程池的 excute 传入闭包
       b. 闭包使用 Box 装箱再通过 Send 将其发送至 " ThreadPool::new() "时
          创建的 channel 即"  mpsc::channel(); "
       c. 接着 worker 会从 channel 的接收端(即 receive 端)等待接收
    */
    pub fn execute<F>(&self, f: F)
        where F: FnOnce() + Send + 'static {
        let job = Box::new(f);
        // 发送
        self.sender.send(job).unwrap();
    }
}

// main 入口
fn handle_client(mut stream: std::net::TcpStream) {
    // 读取请求行至 buffer
    let mut buf = [0; 512];
    std::io::Read::read(&mut stream, &mut buf).unwrap();
    //
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

fn main() -> std::io::Result<()> {
    // 绑定
    let listener = std::net::TcpListener::bind("127.0.0.1:8080")? ;
    // 创建线程池
    let pool = ThreadPool_2::new(4);
    // 循环监听
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        // thread pool 传入闭包
        pool.execute(|| {
            handle_client(stream)
        });
    }
    Ok(())
}