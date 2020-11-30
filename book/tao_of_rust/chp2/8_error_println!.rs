
/*
2.11 错误处理
    Rust 中通过返回 Result<T,E> 类型的方式进行错误处理。" Result<T,E> "是" Option<T> "
    的升级版本(同样定义于标准库)，" Result<T,E> "源码实现：
        源码：  pub enum Result<T, E> {
                    /// Contains the success value
                    #[stable(feature = "rust1", since = "1.0.0")]
                    Ok(#[stable(feature = "rust1", since = "1.0.0")] T),
                    /// Contains the error value
                    #[stable(feature = "rust1", since = "1.0.0")]
                    Err(#[stable(feature = "rust1", since = "1.0.0")] E),
                }
    Option<T> 类型表示值存在的可能性而 Result<T,E> 类型表示错误的可能性("E"代表"Error")
    Interview : Option<T> 与 Result<T,E> 区别

*/
// (代码清单 2-55 ) Result<T,E> 源码
/*
enum Result<T, E> {
    Ok(T),
    Err(E),
}
*/
// (代码清单 2-56 ) Result<T,E> 使用示例
fn main_008_000() {
    /* 定义枚举值 Ok(-3) 后通过 is_ok 判断是否为 Ok(T) 枚举值：
       可将 Result<T,E> 作为函数返回值(类比 Option<T> )则开发者在调用时
       需处理"正常、错误"此两种情况从而为程序的健壮性提供了保证
    */
    let x: Result<i32, &str> = Ok(-3);
    assert_eq!(x.is_ok(), true);
    // 定义枚举值: Err("Some error message")
    let x: Result<i32, &str> = Err("Some error message");
    assert_eq!(x.is_ok(), false);
}

/*
Rust 2015 版本 ：
    main 函数并不能返回 Result<T,E> 但实际开发中二进制可执行库亦需返回错误
    如"读取文件发生错误"时需正常退出程序
Rust 2018 版本 ：
    允许 main 函数返回 Result<T,E>

*/
// (代码清单 2-57 ) main 函数中返回 Result<T,E> 示例
use std::fs::File;
// Rust 2018 版
fn main() -> Result<(), std::io::Error> {
    /* 调用 File::open 打开文件，其后跟随问号操作符" ? "是一个错误处理
       的语法糖(其会在出现错误情况下自动返回" std::io::Error ")，如此
       可在程序发生错误时自动返回错误码并于程序退出时打印相关的错误信息
       (便于调用[无需手动处理错误])
    */
    let f = File::open("bar.txt")?;
    Ok(())
}

/*
补注:
   日常开发中常会使用到" println! "宏语句来进行格式化打印(对于调用非常重要)
   其" println! "宏中的格式化列表如下：
         " nothing "代表 Display : 如 println!("{}",2)
         " ? "代表 Debug         : 如 println!("{:?}",2)
         " o "代表八进制          : 如 println!("{:o}",2)
         " x "代表十六进制小写    ：如 println!("{:x}",2)
         " X "代表十六进制大写    ：如 println!("{:X}",2)
         " p "代表指针           ：如 println!("{:p}",2)
         " b "代表二进制         ：如 println!("{:b}",2)
         " e "代表指数小写        ：如 println!("{:e}",2)
         " E "代表指数大写        ：如 println!("{:E}",2)

小结：
    a. Rust 是一门表达式语言，Rust 中一切皆表达式故掌握表达式求值机制至关重要
       如 if 流程控制亦是表达式故无需单独提供" ?: "条件表达式
    b. 当处理一些 Option 类型时可用 if let 或 while let 表达式简化代码

*/