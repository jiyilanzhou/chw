
/*
4、Super trait(父 trait ):
  用于在 trait 内使用另外 trait 的功能
  注： 相关依赖的 trait 亦须实现

*/

use std::fmt;
// trait 定义 ([":"后为"Super trait":其功能类类比"继承"]):
trait OutPrint: fmt::Display { // 要求实现 Display trait
    fn out_print(&self) {
        // out_print 默认实现
        let output = self.to_string();
        println!("output: {}", output);
    }
}

struct Point{
    x: i32,
    y: i32,
}
// 实现 trait
impl OutPrint for Point {}
/* 未实现 Super trait 则编译报错：
    error[E0277]: `Point` doesn't implement `std::fmt::Display`
         impl OutPrint for Point {}
            ^^^ `Point` cannot be formatted with the default formatter
    = help:the trait `std::fmt::Display` is not implemented for `Point`
    = note:in format strings you may be able to use `{:?}` (or {:#?}
                                             for pretty-print) instead
   // 注： 实现 trait 则其 Super trait 亦需实现
*/

/* // 实现 trait 的 Super trait
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
*/

fn main() {
    println!("Hello, world!");
}
