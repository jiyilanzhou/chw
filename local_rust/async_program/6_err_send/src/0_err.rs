use futures;

/*
0. " ? "运算符：
    (类比同步编程)异步编程亦可使用 ? 运算符
    提前返回 Result 的 Err 类型

 */
async fn foo() -> Result<(), String> {
    Ok(())
}

async fn func() -> Result<(), String> {
    let fut = async {
        foo().await?;
        // Ok(())

        // 显式标注类型( turbofish 操作符)
        Ok::<(), String>(())
    };
    fut.await
}

fn main() {
    let _ = futures::executor::block_on(func());
    println!("Hello, world!");
}
