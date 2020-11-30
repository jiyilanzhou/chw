use futures;
use tokio::runtime::Runtime;

/*
0. await 及 block_on
    a. " .await "皆为阻塞等待执行
        async fn get_book_and_music() -> (Book, Music) {
            /* " book、music "为顺序阻塞等待执行：
               (只是 book、music 阻塞时" fn get_book_and_music(){} "让出当前线程)
            */
            let book = get_book().await;
            let music = get_music().await;
            (book, music)
        }
    b. " block_on "直接阻塞直到 Futrue 执行完毕
            fn main() {
                futures::executor::block_on(async_fn());
            }

1. join : 异步函数间并发执行
    use futures::join;
    async fn get_book_and_music() -> (Book, Music) {
        let book_fut = get_book();
        let music_fut = get_music();
        // book_fut、music_fut 两 Future 并发执行
        join!(book_fut, music_fut)
    }

2. try_join ：针对 Future 返回 Result 应考虑使用 try_join!
   其功能类似 join ，但当添加到 try_join! 宏并发的 Future 中
   有任意子 Future 执行返回错误则 try_join! 立即返回(而 join!
   需等待所有子 Future 执行完毕才能返回)
   // 注：非返回 Result 不可使用 try_join! 宏

*/
async fn func1() {
    /* 同理不可使用" std::thread::sleep "即:
          std::thread::sleep(Duration::from_secs(1));
       解决方案：使用 async_std::task::sleep 或 tokio::time::delay_for "
     */
    // async_std::task::sleep(Duration::from_secs(1)).await;
    tokio::time::delay_for(tokio::time::Duration::from_secs(1)).await;
    println!("func1 finished!");
}

async fn func2() {
    println!("func2 finished!");
}

async fn async_main() {
    let f1 = func1();
    let f2 = func2();
    // 相当于 " f1、f2 "并发(对比" block_on、await "皆为阻塞等待执行)
    futures::join!(f1, f2);
    /* // 非返回 Result 不可使用 try_join! 宏
      error[E0599]: no method named `is_err` found for mutable reference `&mut ()`
                                                             in the current scope
         futures::try_join!(f1, f2);
         ^^^^^^^^^^^^^^^^^^^^^^^^^^^ method not found in `&mut ()`
     */
    // futures::try_join!(f1, f2);
}

fn main() {
    /* 若使用" async_std::task::sleep "：
          则对应使用 futures 库的 block_on 阻塞执行 Future
     */
    // futures::executor::block_on(async_main());

    // 使用 tokio 库的 runtime.block_on (对应" tokio::time::delay_for ")
    let mut runtime = Runtime::new().unwrap();
    // tokio::runtime::Runtime 内含 Executor (执行器)
    runtime.block_on(async_main());
    println!("Hello World!");

}

