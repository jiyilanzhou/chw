
/* 变量及常量:
    Rust 语言是静态类型语言，即编译时须知晓变量类型

*/
// 常量定义 const
const MAX_POINTS: u32 = 100000;
fn main() {
    //1、变量定义 let ：未使用 mut 修饰则默认为"不可变"
    // 自动推导类型
    let a = 1;
    println!("a = {}", a);

    // 显示声明类型( let name: type )
    let mut b: u32 = 1;
    println!("b = {}", b);

    // 定义变量用 let ：若变量未使用 mut 修饰则默认为"不可变"
    b = 2;
    println!("b = {}", b);

    //2、隐藏(后定义的同名变量隐藏前定义的同名变量)
    let b: f32 = 1.1;
    println!("b = {}", b);

    //3、常量 const
    println!("MAX_POINTS = {}", MAX_POINTS);

}
