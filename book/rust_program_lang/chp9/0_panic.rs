
/*
0. 错误处理
   a. Rust 中将错误分为两大类：可恢复错误 与 不可恢复错误。对于可恢复错误
      如文件未找到等，一般需要将其报告给用户并再次尝试进行操作；而不可恢复
      错误往往就是 bug 的另一种说法，比如尝试越界访问数组。
   b. Rust 提供用于可恢复错误的类型 Result<T,E> 及程序出现不可恢复错误时
      中止运行的 panic! 宏。

1. panic 中的栈展开与终止(p202[*])
   a. 当 panic 发生时，程序默认会开始栈展开(unwinding)，这意味着 Rust 会
      沿着调用栈的反向顺序遍历所有调用函数，并依次清理这些函数中的数据。但
      为支持这种遍历和清理操作，需要在二进制中存储许多额外信息。
   b. 当 panic 发生时除了栈展开，亦可选择立即终止(abort)程序，它会直接结束
      程序且不进行任何清理工作，程序所使用过的内存只能由操作系统来进行回收。
   c. 若项目需要使最终二进制文件尽可能小，那么可以通过在 Cargo.toml 文件的
      "[profile]"区域部分增加 panic = 'abort' 来将 panic 的默认行为从展开
      切换为终止。如在 release 模式中使用终止模式则可在Cargo.toml文件配置：
            [profile.release]
            panic = 'abort'

2. 使用 panic! 产生的回溯信息
   $ RUST_BACKTRACE=1 cargo run
   回溯列表：基于定位自编辑代码文件的那一行往上是"自编辑代码调用的代码",往下
            则是调用"自编辑代码的代码"(理解记忆：越上层的越接近错误发源地)

3. 可恢复错误 与 Result
   a. 大部分的错误其实都没有严重到需要整个程序完全停止的地步，函数有时会由于
      一些可简单解释并做出响应的原因而失败，比如尝试打开文件的操作会因为文件
      不存在而失败(此时可能考虑创建文件而不是终止进程）
   b. std::result::Result 源码
          pub enum Result<T, E> {
              /// Contains the success value
              #[stable(feature = "rust1", since = "1.0.0")]
              Ok(#[stable(feature = "rust1", since = "1.0.0")] T),
              /// Contains the error value
              #[stable(feature = "rust1", since = "1.0.0")]
              Err(#[stable(feature = "rust1", since = "1.0.0")] E),
          }

*/
// 匹配不同错误
use std::io::ErrorKind;
fn main_000_001() {
    let f = File::open("hello.txt");
    //方式1 ：多 match 实现
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Tried to create the file but there was a problem: {:?}", e),
            },
            other_error => panic!("There was a problem opening the file: {:?}", other_error),
        },
    };

    // 方式2 ：未包含 match 表达式且更为清晰易读(更有经验的实现)
    let f = File::open("hello.txt").map_err(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Tried to create the file but there was a problem: {:?}", error);
            })
        } else {
            panic!("There was a problem opening the file: {:?}", error);
        }
    });
}

/*
4. 失败时触发 panic 的快捷方式：unwrap 和 expect
    a. "unwrap"实现(类似 match 语句)
       (0). 若 Result 值是成员 Ok 则 unwrap 返回 Ok 中的值
       (1). 若 Result 是成员 Err 则 unwrap 调用 panic!
    b. "expect"实现(类比"unwrap"且可携带自定义提示信息[稍强大一些])
       (0). 允许指定 panic! 所附带的错误信息提示
       (1). 提供错误信息可表明意图并更易于追踪 panic 起源

5. 错误传播(p213[*])
   a. 将错误返回让调用者处理的过程即是错误传播
   b. 错误传播的模式在 Rust 编程中非常常见，所以 Rust 专门提供一个问号(?)
      运算符来简化它的语法

6. 传播错误的快捷方式：? 运算符
    a. 通过将 ? 放置于 Result 之后实现与使用 match 表达式来处理 Result 一样的
       功能。若 Result 的值为 OK 则包含 OK 的值会作为此表达式的值返回并继续执行
       程序；若值是 Err 则其会作为整个程序的结果提前返回(类比使用" return Err")
    b. ? 运算符与 match 区别
      (0).被 ? 运算符所接收的错误值会隐式地被 from 函数处理(这个函数定义于标准库
          的"From trait:用于错误类型之间进行转换")
      (1).当 ? 运算符调用 from 函数时，它就开始尝试将传入的错误类型转换为当前函数
          返回的错误类型。当一个函数拥有不同的失败原因，却使用了统一的错误返回类型
          来进行表达时，此功能会十分有用。只要每个错误类型都实现了转换为错误类型的
          from 函数，? 运算符会自动处理所有的转换过程)

    c. ? 运算符只能被用于返回 Result 的函数
       因 ? 运算符类比 match 表达式(携带" return Err(e) "故返回 Result)，故函数
       的返回类型亦必须是 Result 才能与 return 兼容

*/
use std::io::{self, Read};
use std::fs::{self, File};
// 一般写法：? 运算符
fn read_username_from_file_0() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("file")?.read_to_string(&mut s)?;
    Ok(s)
}
/* 更为符合项目实战的写法：
   因从文件中读取字符串是相当常见的操作，故 Rust 提供了一个函数
   "std::fs::read_to_string"：用于打开文件、创建一个新 String
   并将文件中的内容读入这个 String，接着返回调用者。
*/
fn read_username_from_file_1() -> Result<String, io::Error> {
    // "fs::read_to_string"返回"io::Result<String>"故可直接返回
    fs::read_to_string("file")
}

// 注意：使用 ? 运算符后仅在错误时提取 Err 提前返回但正常时往下执行
fn read_username_from_file_2() -> Result<String, io::Error> {
    fs::read_to_string("file")?;
    // 没有此表达式则返回空 tuple () 从而不符合函数返回值声明
    Ok("chw".to_string())   // 成功则自定义数据返回
}

/*
7. 要不要使用 panic! (p219 ~ 225[*])

*/