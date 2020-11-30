
// tokio[ˈtəʊkjəʊ]n.东京
/*
0. Tokio 延迟执行(模拟 setTimeout )
   使用前添加依赖：
      [dependencies]
      futures = "0.3"
      tokio = {version = "0.2",features = ["full"]}

*/

use tokio::time::{delay_for, Duration, timeout};

async fn job() -> String {
    println!("延迟 2 秒后执行 ... ");
    String::from("chw")
}

#[tokio::main]
async fn main_0() {// 将 main 亦变为异步函数
    delay_for(Duration::from_secs(1)).await;
    println!("延迟 1 秒执行 ... ");
    let ret = timeout(Duration::from_secs(2), job()).await;
    // " .await "返回 Result
    println!("{}", ret.unwrap());
}

/*
1. 使用 tokio 完成简单的 webserver

*/
use tokio::net::TcpListener;
use tokio::prelude::*;

#[tokio::main]
async fn main_1() {
    let mut server = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    loop {
        let (mut stream, _) = server.accept().await.unwrap();
        let rsp = String::from("HTTP/1.1 200 OK\r\n\r\n Rust Server ...");
        stream.write_all(rsp.as_bytes()).await.unwrap();
    }
}

/*
2. Task
    a. 轻量级：Tokio 运行时调度(不由 OS 调度[类比 Go 协程])
    b. 基本用法： task::spawn  <-> (对标) thread::spawn

3. 预习两个框架
    a. Rocket ： web 框架
    b. hyper : http 底层

*/
use tokio::task;

#[tokio::main]
async fn main() {
    let mut server = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    loop {
        let (mut stream, _) = server.accept().await.unwrap();
        // 调度
        task::spawn(async move {
            let mut rsp = String::from("HTTP/1.1 200 OK\r\n\r\n Rust Server ... ");
            // 耗时程序
            let ret = task::spawn_blocking(|| {
                // ...
                "blocking"
            }).await;
            if let Ok(s) = ret{
                rsp.push_str(s);
            }
            // 回写数据
            stream.write_all(rsp.as_bytes()).await.unwrap();
        });
    }
}