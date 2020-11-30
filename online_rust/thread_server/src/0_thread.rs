
/*
0. 线程
    同步与异步

 */
use std::thread;
use std::time::Duration;
use std::sync::mpsc;

fn main() {
    /* channel：异步通道(即 channel 是有缓冲的通道[类比 Golang ])
       模式匹配错误示范(多个变量与类型不可分开声明，因其为元组整体
       [区别于参数列表])：
          let (tx:mpsc::Sender<i32>,rx:mpsc::Receiver<i32>)
          = mpsc::channel();
       应正确表示为：
          let (tx, rx):(mpsc::Sender<i32>,mpsc::Receiver<i32>)
          = mpsc::channel();
     */
    let (tx, rx):(mpsc::Sender<i32>,mpsc::Receiver<i32>) = mpsc::channel();
    //let (tx, rx) = mpsc::channel();

    /* 同步通道" mpsc::sync_channel(0) ": 接收发送均阻塞等待对端处理
           (类比 Go 无缓冲 channel)而" mpsc::sync_channel(非零) "即
           自定义缓冲容量的异步 channel (类比 Go 自定义缓冲 channel )
     */
    let (tx,rx) = mpsc::sync_channel(0);

    // 开启线程
    thread::spawn(move || {
        for i in 0..5 {
            /* channel 为异步通道(即" send "后通道未填满时可继续" send "[并非
               阻塞等待对端接收才能继续 send : 因 channel 是有缓冲的异步通道)
             */
            tx.send(i).unwrap();
            // 睡眠 500 毫秒
            thread::sleep(Duration::from_millis(500));
        }
    });
    // 接收
    /*
       loop{
           if let Ok(i)=rx.recv(){
               print!("{}\t",i);
           }else{
               println!("Done");
               break;
           }
       }
    */
    //thread::sleep(Duration::from_secs(3));
    for i in rx {
        print!("{}\t", i);
    }
}
