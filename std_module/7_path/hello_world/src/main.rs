
/*
0. 用于在模块树中指明条目的路径
    (Base[p156~)

1. 用 use 关键字将路径导入作用域
   (Base[p165~168])

2. 使用外部包(Base[p170)
    标准库亦被视作外部包

3. 模块系统(tao[p339])两个版本的不同之处
    a. 2015 版本
       需使用 extern crate 语法导入第三方包
    b. 2018 版本
        (0). 无需在"根模块"中使用 extern crate 语法导入第三方包
        (1). 在模块导入的路径中使用 crate 关键字表示当前 crate
        (2). 按照特定规则，mod.rs 可省略
        (3). use 语句可使用嵌套风格导入模块
    // 具体使用参见" tao[p339] 的 模块系统 "

4. 可见性和私有性(tao[p359])

*/
/*
0. extern 的使用
    a. 同级目录下分别创建 bin 及 lib 项目
       Administrator@CHW MINGW64 /e/project/std_module/path (master)
           $ cargo new hello_world --bin
           $ cargo new good_bye --lib
     b. 在 lib.rs 中添加以下代码
            pub fn say(){
                println!("good bye");
            }
     c. 在" hello_world / Cargo.toml "配置
        # 引用本地路径中的库
        [dependencies]
        good_bye = { path = "../good_bye" }
        # 引用官方库
        #[dependencies]
        #lazy_static = "1.0.0"
        [dependencies]
        regex = "1.0.5"
    d. 使用(如在" main "函数中)
        //extern crate regex; // 先使用 extern 声明引入 regex 包
        use regex::Regex; // 新版本无需使用 extern 声明引用即可使用

1. extern 、use 功用
    使用 extern 声明外部包: " extern "用于连接外部包、函数或变量
    a. 针对 2015 版本
       若使用 workspace 囊括" hello_world "及" good_bye " crate
       则可直接使用" use good_bye; "替代" extern crate good_bye; "
    b. 针对 2018 版本(" use good_bye; / extern crate good_bye; "皆可)
       即使未使用 workspace 亦可直接使用" use good "(即两种方式皆可)
    c. extern 与 use 区别
       " extern "用于连接外部包、函数或变量
       " use "把符号引入作用域
    d. 通过 extern 和 use 关键字引入包和模块为声明语句
            // extern crate std;
            // use std::prelude::v1::*;
        注释掉" extern crate std; 及 use std::prelude::v1::*; "的原因
        是 Rust 会为每个 crate 自动引入标准库模块(除非使用" #[no_std] "
        或" #![no_std] "指明不需要标准库)

*/
//extern crate good_bye;

// " ::path "表示从根模块开始的相对路径(全局路径)
// use ::good_bye;
// use good_bye;

fn main() {
    good_bye::say();
}

/*
1. 相对路径
    use self::A::B::func;
    use A::B::func;

2. 绝对路径
    use crate::A::B::func;

3. 配置及 workspace 的用途 (Dive[p355])
    a. 配置
        " $HOME/.cargo/config " 全局配置
    b. workspace
       主要用于解决多个 crate 间互相协调问题而存在，

*/

mod A {
    pub mod B {
        pub fn func() {
            println!(" A::B::func(静心道) ")
        }
    }
}
// use self::A::B::func;
// use crate::A::B::func;
/* " ::path "表示从根模块开始的相对路径(全局路径)
    a. 2015 版本
           " use ::A::B::func; / use A::B::func "皆可
    b. 2018 版本
           " use ::A::B::func; " 未生效,待解决[?]

 */
// use A::B::func;
// use hello_world::A::B::func; // 未实现待解决[?]
fn main_1() {
    // func();
    A::B::func();

}