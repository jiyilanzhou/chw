use futures::Future;
use futures::executor;

/* 异步编程：使用单线程达到多线程效果(降低开销)
 async 关键字：
     a. 在函数前添加 async 关键字后即为实现了 Futrue trait 的状态机
        表象类似:"  fn hello() -> impl Future<Output=()> {
                        async {
                            println!("hello");
                        }
                    }
                "
     b. " core::future::Future / std::future::Future "源码：
            pub trait Future {
                type Output;
                /* Pin 可创建不可移动的 trait object (可在其字段间存储指针)
                   表象类似: struct Fut {
                                   a : i32,
                                  prt : *const i32    // 指向首元素(即结构体起始位置)
                             }
                */
                fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
            }
            pub enum Poll<T> {
                Ready(#[stable(feature = "futures_api", since = "1.36.0")] T),
                Pending,
            }
            #[derive(Copy, Clone)]
            pub struct Pin<P> {
                pointer: P,
            }
    c. " async "关键字可修饰函数或代码块
                fn func() -> impl Future<Output=()> {       // 返回 Future 状态机
                    async {
                        println!("async_func");
                    }
                }
            等价于:
                async fn func(){
                    println!("async_func");
                }

*/

// 通过 async 定义异步函数
async fn hello() {
    std::thread::sleep(std::time::Duration::from_secs(1));
    println!("hello");
}

// 使用 block_on 阻塞调用
fn main_0() {
    /* 直接调用异步函数(此时已非普通函数而是 Future )不会执行：
          a. 可通过执行器"  executor::block_on(async_fn) "阻塞式(即未让出线程控制权)调用执行
          b. 亦可在异步函数或代码块内通过 await 阻塞调用(注：前提是必须置于异步函数或代码块内)
          // [自]" executor::block_on(async_fn); "一般用于主函数内阻塞调用异步函数;
                 " await "一般用于异步函数或代码块内阻塞调用有先后逻辑顺序要求的异步函数
     */
    let async_fn = hello();

    // error[E0728]: `await` is only allowed inside `async` functions and blocks
    //async_fn.await;

    // 阻塞式调用(顺序执行)：即等待" executor::block_on(async_fn); "执行完毕
    //executor::block_on(async_fn);
    executor::block_on(hello());

    // (顺序执行)即等待"  executor::block_on(async_fn); "执行完毕方能执行
    println!("Hello Rust");
}

// asyn 函数或代码块内使用 await 异步调用
use std::time::Duration;
use async_std::task;

async fn learn_song() {
    // std::thread::sleep(Duration::from_secs(1));
    task::sleep(Duration::from_secs(1)).await;
    println!("learn_song");
}

// 逻辑顺序" learn_song " -> " sing_song "(学习后才能唱)
async fn sing_song() {
    println!("sing_song");
}

async fn dance() {
    println!("dance");
}

async fn sing() {
    // 阻塞等待(有先后顺序)
    learn_song().await;
    sing_song().await;
}

async fn async_fn() {
    let f1 = sing();
    let f2 = dance();
    // " f1 或 f2 "阻塞则会让出线程控制权(类比并发执行)
    futures::join!(f1, f2);
}

fn main() {
    /* (直接使用"std::thread::sleep")阻塞式调用:执行效果未符合预期
        Console:"
                  learn_song
                  sing_song
                  dance             // 未符合预期
                "
     */
    executor::block_on(async_fn());

    // 顺序执行
    println!("Hello Rust");

    /* 未达到预期效果原因分析：
            直接使用标准库" std::thread::sleep "睡眠
            并没有唤醒机制。
       解决方案：使用" async-std "提供的" task::sleep "
     */
}
