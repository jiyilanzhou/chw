
/*
1、rust语言将错误分为两个类别：可恢复错误和不可恢复错误
  （1）可恢复错误通常代表向用户报告错误和重试操作是合理的情况，例如未找到文件。
       rust 中使用 Result<T,E> 来实现。
  （2）不可恢复错误是 bug 的同义词，如尝试访问超过数组结尾的位置。rust 中通过 panic！来实现

2、panic！

3、使用 BACKTRACE=1

4、Result<T, E> 源码：
   enum Result<T, E> {
      Ok(T),
      Err(E),
   }

5、简写

注：调试小技巧" RUST_BACKTRACE = 非0 "
    如命令" RUST_BACKTRACE=1  cargo run " 可查看栈追踪信息

*/

use std::fs::File;
fn main() {
    let f = File::open("hello.txt");
    let r = match f {
        Ok(file) => file,

        /* 编译错误:" error[E0308]: match arms have incompatible types "
           问题:空 tuple () 不是可转换为所有类型么？为何不兼容[?]
           解析:[自]可转换为所有类型的是" never type "即" ! "而非空 tuple
        */
        // _=>(),

        Err(error) => panic!("error: {:?}", error),
    };

    // 简写方式 1 : unwrap
    //let f = File::open("hello.txt").unwrap();
    // 简写方式 2 : expect("预期展示的错误提示信息")
    let f = File::open("hello.txt").expect("Failed to open hello.txt");

    //panic!("crash here");

}