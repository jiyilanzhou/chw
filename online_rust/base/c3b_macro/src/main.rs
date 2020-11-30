/*
0. 宏(macro)
    可进行元编程(meta program ：用代码生成代码、控制代码、扩展代码)

1. 声明宏

2. 过程宏

 */
/* 将宏引入作用域：
    " mod xx "之上的注解" #[macro_use] "表示将 xx "模块
    即" xx.rs "或" xx/mod.rs "文件内定义的宏引入作用域
    // 问题：如何将定义于其它文件内的宏引入作用域
       解决方案：宏上方标注" #[macro_extern] "使其对外部可见
 */
#[macro_use]
mod dmacro;

fn main() {
    // echo!();
    //common!(true && false);
    // 未传入参数则不进行宏展开
    common!();

    // 宏(元编程)展开：用于生成函数
    func!(function);
    // 调用
    function();

    // 引入定义于"...\dmacro\custom.rs "的宏
     cat!();

}