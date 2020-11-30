
/*
4、过程宏
   a. 过程宏接收输入的 Rust 代码并于其上进行操作，然后产生输出代码。
      区别于声明宏(匹配对应模式后以另一部分代码替换当前代码)
   b. 定义过程宏的函数接受一个 Tokenstream 作为输入并产生一个 Tokenstream
      作为输出(亦是宏的核心:宏所处理的源代码组成了输入 Tokenstream 同时宏
      生成的代码是输出 Tokenstream )如:
          use proc_macro
          #[some_attribute]
          pub fn some_name（input: TokenStream）-> TokenStream {
              // ...
          }

示例 ：
    a. 配置依赖于"...\96learn_macro2\main\Cargo.toml":
       [dependencies]
       hello_macro = {path = "../hello_macro"}
       hello_macro_derive = {path = "../hello_macro/hello_macro_derive"}
    b. 按需引用后调用
       进入"... /96learn_macro2/main " 后执行" cargo run "

*/

use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

/* derive 宏：
   在结构体上标注则编译器自动为其实现相应 trait 如
        #[derive(Debug)]
        struct SN{}
    // 编译器自动为结构体生成实现" Debug trait "的代码
*/
#[derive(HelloMacro)]   // 过程宏(" derive 宏 ")
struct Main;

// 使用宏
fn main() {
    Main::hello_macro();
}
