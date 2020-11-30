/*
 0. 使用 channel 发送结构体对象

 */
#[derive(Debug)]
struct User {
    id: i32,
    name: String,
}

impl User {
    fn new(id: i32, name: String) -> User {
        User {
            id,
            name,
        }
    }
}

use std::sync::{mpsc, Arc};
use std::thread;
use std::time::Duration;

fn main_0() {
    // 普通初始化对象
    let user = User::new(0, "chw".to_string());
    // 获取生产者及消费者
    let (tx, rx): (mpsc::Sender<User>, mpsc::Receiver<User>) = mpsc::channel();
    thread::spawn(move || {
        tx.send(user).unwrap();
    });
    // println!("{:?}",user); // 移动后不可再用
    println!("{:#?}", rx.recv().unwrap());

    // 使用 Arc
    let user = Arc::new(User::new(0, "chw".to_string()));
    // 获取生产者及消费者
    let (tx, rx) = mpsc::channel();
    let u = Arc::clone(&user);
    thread::spawn(move || {
        tx.send(u).unwrap();
    });
    println!("{:?}", user);
    println!("{:#?}", rx.recv().unwrap());
}

/*
练习：创建多线程

*/
fn curl(i: i32) {
    thread::sleep(Duration::from_secs(1));
    println!("第 {} 个网页抓取完成 ... ", i);
}

fn main_1() {
    let mut thread_pool = Vec::new();
    for i in 0..5 {
        thread_pool.push(thread::spawn(move || curl(i)));
    }
    // 循环等待
    for i in thread_pool {
        i.join().unwrap();
    }
}

/*
1. 使用 channel 进行多线程通信

*/
fn curl_tx(tx: mpsc::Sender<String>, i: i32) {
    thread::sleep(Duration::from_secs(1));
    tx.send(format!("第 {} 个网页抓取完成 ... ", i)).unwrap();
}

fn main() {
    let (tx, rx) = mpsc::channel();
    for i in 0..5 {
        // let clone_tx = mpsc::Sender::clone(&tx);
        let clone_tx = tx.clone();
        thread::spawn(move || curl_tx(clone_tx, i));
    }
    /* 若仅使用 tx 的副本而从未使用 tx 本身则" for i in rx {} "会持续等待。
       解决方案:使用tx即" thread::spawn(move||tx.send("".to_string())); "
     */
    thread::spawn(move || tx.send("".to_string()));
    for i in rx {
        println!("{}", i);
    }
}

