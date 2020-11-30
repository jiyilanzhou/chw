pub mod custom;

pub use custom::*;

macro_rules! echo {
    ()=>(
            println!(" echo! ");
    )
}

/*
0. 宏：不是函数

1. 宏参数分为两个部分:
    a. 参数前缀( $ 开头)
    b. 指示符(有好几种)
        (0). 较为简单常用的有 expr (用于表达式)
        (1). stmt(语句)
        (2). ident(常用于函数名和变量名)

2. 可变参数的基本定义：
   Rust 中函数不支持可变参数(但宏支持)

 */

macro_rules! common {
    /*
        () => (
             println!(" echo! ");
        );  // 使用分号隔开
        // " $ "参数前缀
        ($expression:expr) => (
             //println!("{}",$expression);
             // 将表达式换为字符串形式
             println!("{}",stringify!($expression));
        );
    */
    // 匹配不定参("+/*"分别代表" 1 个或多个、0 个或多个[即任意数])
    ($($x:expr),*) => (
        /* 此" $( "代表外部的" $(prm) "即对不定参的处理
           其括号内才是对单个参数即" $x:expr "的处理
         */
        $(
            println!("未传入参数则不运行{}",stringify!($x));
        )*
    )
}

macro_rules! func {
    ($fn_name:ident) => (
        fn $fn_name(){
            println!("{}","宏展开生成函数");
        }
    );
}