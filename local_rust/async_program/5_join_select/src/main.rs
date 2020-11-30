
use futures::{select, future::{self,FutureExt}, pin_mut};
use tokio::runtime::Runtime;
use std::io::Result;
/*
2. try_join ：针对 Future 返回 Result 应考虑使用 try_join!
   其功能类似 join ，但当添加到 try_join! 宏并发的 Future 中
   有任意子 Future 执行返回错误则 try_join! 立即返回(而 join!
   需等待所有子 Future 执行完毕才能返回)
   // 注：非返回 Result 不可使用 try_join! 宏

3. select ：有任一 Future 完成即可返回( select 可与 Fuse 搭配使用)
       a. join! 宏等待所有 Future 执行完毕方可返回
       b. try_join! 宏在子 Future 未出错情况下亦为等待所有 Future
          执行完毕方可返回
       c. 有任一 Future 完成即可返回

*/
async fn func1() -> Result<()> {
    // 其本质亦是 Future 故可使用" .await "
    tokio::time::delay_for(tokio::time::Duration::from_secs(1)).await;
    println!("func1 finished!");
    Ok(())
}
async fn func2() -> Result<()> {
    println!("func2 finished!");
    Ok(())
}

async fn async_main() {
    let f1 = func1();
    let f2 = func2();
    // 方式 1 ： 使用 join! 处理
    //futures::join!(f1,f2);

    // 方式 2 ： 使用 try_join! 处理 Futures
    /* 返回 Result 推荐使用 try_join! 宏处理：
           有任意子 Future 执行返回错误则 try_join! 立即返回
     */
    if let Err(_) = futures::try_join!(f1, f2) {
        println!("Err!");
    }

    /*方式 3 ：
        使用 select! (有任一子 Future 完成即可返回)处理 Futures
        ( select! 需与 Fuse 搭配使用)
     */
}
async fn async_main_select() {
    // 此例 select! 与 Fuse 搭配使用
    let f1 = func1().fuse();
    let f2 = func2().fuse();
    //方式 3 : 使用 select! (有任一 Future 完成即可返回)处理 Futures
    pin_mut!(f1, f2);
    select! {
		_ = f1 => println!("select func1 finished ... "),
		_ = f2 => println!("select func2 finished ... "),
	}
}

fn main() {
    //let mut runtime = Runtime::new().unwrap();
    //runtime.block_on(async_main());
    //runtime.block_on(async_main_select());

    // runtime.block_on(count());
    futures::executor::block_on(count());
    println!("Hello World!");
}

/*
使用 select! 宏：
    a. select 中使用的 Future 必须实现 Unpin trait 和 FusedFuture trait
    b. 必须实现 Unpin 的原因是 select 中使用的 Future 不是按值获取的而是
       按照可变引用获取的，通过不获取 Future 的所有权，所以在调用 select
       后，未完成的 Future 可以继续使用
    c. 必须实现 FusedFuture 的原因是 select 完成后不会再轮询 future,因此
       需要实现 FusedFuture 来跟踪 future 完成，同理对应到 stream 上会有
       一个 FusedStream trait 如：
            use futures::{
                stream::{Stream, StreamExt, FusedStream},
                select,
            };
            async fn add_two_streams(
                mut s1: impl Stream<Item = u8> + FusedStream + Unpin,
                mut s2: impl Stream<Item = u8> + FusedStream + Unpin,
            ) -> u8 {
                let mut total = 0;
                loop {
                    let item = select! {
                        x = s1.next() => x,
                        x = s2.next() => x,
                        complete => break,
                    };
                    if let Some(next_num) = item {
                        total += next_num;
                    }
                }
                total
            }
     d. Fuse::terminated() 允许构建一个已经终止的空的 Future (可简单理解为不可跳转的状态)
     e. 当需同时运行同一 Future 的多个副本时可用 FuturesUnordered 类型

 */
async fn count() {
    let mut a_fut = {
        std::thread::sleep(std::time::Duration::from_secs(3));
        future::ready(4)
    };
    let mut b_fut = future::ready(6);
    let mut total = 0;
    // 循环监听：此例 select！ 与 Ready 搭配
    loop {
        select! {   // select! 不会轮询故需要配合 loop
			a = a_fut => total += a,
			b = b_fut => total += b,
			// 表示所有分支都已完成且不会再取得进展的情况
			complete => break,
			//表示没有分支完成
			default => unreachable!(),
		}
    }
    assert_eq!(total, 10);
}
