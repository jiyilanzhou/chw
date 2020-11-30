/*
0. 异步编程

1. future
    参阅" https://github.com/rust-lang/futures-rs "
    future 是由异步计算产生的单个最终值(类比 JS 中的 promise )

*/
use std::time::Duration;
use std::thread;
use futures::executor::block_on;

// 添加 async 后为异步函数
async fn get_username() -> String {
    thread::sleep(Duration::from_secs(1));
    "chw".to_string()
}

async fn get_score() -> i32 {
    100
}

async fn get_info()->String{
    thread::sleep(Duration::from_secs(1));
    // 异步方法内调用异步方法(使用 await 等待)
    format!("username is {}, score is {}","chw",get_score().await)
}

fn main() {
    /*
        // 调用异步函数(程序继续往下执行[无须等待函数调用结束])
        get_username();
        println!("Hello Rust");
    */
    // 使用 block_on 实现异步方法调度
    let ret = block_on(get_username());
    println!("{}", ret);

    // 异步方法内使用 await 等待调用异步方法的结果
    let ret = block_on(get_info());
    println!("{}",ret);

}