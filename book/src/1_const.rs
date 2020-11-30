/*
0. (常量) Rust 两种常量：可在任意作用域(包括全局作用域)声明但都需要显式的类型声明
    const：不可改变的值
    static：具有 'static 生命周期(可变: 须使用 static mut 关键字）

 */
// 全局变量是在所有其它作用域之外声明
static LANGUAGE: &'static str = "Rust";
const  THRESHOLD: i32 = 10;

fn is_big(n: i32) -> bool {
    // 在一般函数中访问常量
    n > THRESHOLD
}

fn main_0() {
    let n = 16;
    // 在 main 函数（主函数）中访问常量
    println!("This is {}", LANGUAGE);
    println!("The threshold is {}", THRESHOLD);
    println!("{} is {}", n, if is_big(n) { "big" } else { "small" });
    // const 常量不能修改
    // THRESHOLD = 5;
}