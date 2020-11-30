
/* 通道:
1、Rust 中一个实现消息传递并发的主要工具是通道。通道由两部分组成，
  一个是发送端，一个是接收端，发送端用来发送消息，接收端用来接收消息。
  发送者或者接收者任一被丢弃时就可以认为通道被关闭了。

2、通道介绍
    （1）通过 mpsc::channel 创建通道，mpsc 是多个生产者单个消费者；
    （2）通过 spmc::channel 创建通道，spmc 是单个生产者多个消费者；
    （3）创建通道后返回的是发送者和接收者，示例：
         let (tx, rx) = mpsc::channel();
         let (tx, rx) = spmc::channel();

     问题:如何创建"单生产者单消费者"或"多生产者多消费者"[?]

*/
use std::thread;
use std::sync::mpsc;

fn main() {
    // 多生产者单消费者
    let (tx, rx) = mpsc::channel();
    // 创建子线程(使用" move "将 tx 移至子线程)
    let handle = thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    // 非阻塞等待
    /* " try_recv() "非阻塞等待：需配合"  handle.join() "使用 :
         handle.join().unwrap(); // 等待子线程结束
         let received = rx.try_recv().unwrap();
    */

    // 阻塞等待
    let received = rx.recv().unwrap();
    println!("Got: {}", received);

}
/* 知识点：
1、发送者的 send 方法返回的是一个 Result<T,E>
   如果接收端已经被丢弃了，将没有发送值的目标，此时发送会返回错误。
2、接受者的 recv 返回值也是一个 Result 类型，当通道发送端关闭时，返回一个错误值。
3、接收端使用的 recv 方法会阻塞直到有一个消息到来。亦可使用 try_recv() 但其不会
   阻塞会立即返回。

*/
