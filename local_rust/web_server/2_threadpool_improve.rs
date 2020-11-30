
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;

// work 需要等待在 channel 的接收端
struct Worker {
    // worker 标号
    id: usize,
    // 线程句柄
    thread: Option<std::thread::JoinHandle<()>>,
}

impl Worker {
    // 将管道接收端 receiver 传入创建 worker ，这样 worker 才能在 channel 接收端等待
    // fn new(id:usize,receiver:mpsc::Receiver<Job>)->Worker{ //receiver非线程安全
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = std::thread::spawn(move || {
            // 循环等待 receiver 并处理
            loop {  // 使用 Loop 则 worker 创建后一直循环，永不会结束(待完善)
                // 接收端有多个线程等待故需要" 锁  "(此例设定为" Mutex ")
                // receiver 阻塞等待       recv() 为接收到的内容
                //let job = receiver.lock().unwrap().recv().unwrap();
                let message = receiver.lock().unwrap().recv().unwrap();
                match message {
                    Message::NewJob(job)=>{
                        println!("worker {} receive a job",id);
                        job();
                    },
                    Message::Terminate=>{
                        println!("worker {} receive a terminate",id);
                        break;
                    }
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}

// 线程池
pub struct ThreadPool_3 {
    // 将 worker 装入容器
    workers: Vec<Worker>,
    // 发送者：使用" channel "("Job"为发送的任务)
    //sender: mpsc::Sender<Job>,
    // 发送 Job 或 结束消息
    sender: mpsc::Sender<Message>,
}

//  Job 为 trait 对象(因为需要在线程间传递)
type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool_3 {
    // 指定线程数量
    pub fn new(size: usize) -> ThreadPool_3 {
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
        ThreadPool_3 {
            workers,
            sender:sender,
        }
    }

    /* 线程池工作流程:
       a. 主线程调用线程池的 excute 传入闭包
       b. 闭包使用 Box 装箱再通过 Send 将其发送至 " ThreadPool_3::new() "时
          创建的 channel 即"  mpsc::channel(); "
       c. 接着 worker 会从 channel 的接收端(即 receive 端)等待接收
    */
    pub fn execute<F>(&self, f: F)
        where F: FnOnce() + Send + 'static {
        let job = Box::new(f);
        // 发送
        // self.sender.send(job).unwrap();
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

enum Message{
    NewJob(Job),    // 具体任务
    Terminate       // 结束消息
}

impl Drop for ThreadPool_3 {
    fn drop(&mut self) {
        // 发送 terminate 消息到所有 workers
        for _ in &mut self.workers{
            // 每一个 worker 具有一个线程故需遍历发送
            self.sender.send(Message::Terminate);
        }

        // 等待所有 workers 结束
        for worker in &mut self.workers {
            // 等待工作线程结束
            // take 即是将 Option 内包裹的对象拿出(移动语义:详情参阅标准库)
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
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
    let listener = std::net::TcpListener::bind("127.0.0.1:8080")?;
    // 创建线程池
    let pool = ThreadPool_3::new(4);
    //for stream in listener.incoming() {     // 循环监听
    // 循环监听且增加限制:接收 4 个消息后结束("迭代器"亦设计了" take "方法)
    for stream in listener.incoming().take(4) {
        let stream = stream.unwrap();
        // thread pool 传入闭包
        pool.execute(|| {
            handle_client(stream)
        });
    }
    Ok(())
}
