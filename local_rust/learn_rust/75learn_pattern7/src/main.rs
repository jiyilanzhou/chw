
/* (函数/方法)忽略(使用"_")模式中的值
    应用场景：主要用于 trait 的实现
*/
fn foo(_: i32, y: i32) {
    println!("y = {}", y);
}

// trait 及其实现
trait A {
    fn bar(x: i32, y: i32);
}
struct B {}
impl A for B {
    /* 使用"_"忽略模式值典型场景：
           实现 trait ：可按需绑定形参
    */
    fn bar(_: i32, y: i32) {
        println!("y = {}", y);
    }
}

fn main_0() {
    // (函数)忽略模式中的值
    foo(1, 2);
    // (方法)忽略模式中的值
    B::bar(3,3);    // Console:" y = 3 "

    // (元组)忽略模式中的值
    let numbers = (1, 2, 3, 4);
    match numbers {
        // 使用" _ "忽略部分值
        (one, _, three, _) => {
            println!("one: {}, three: {}", one, three);
        },
    }

}

fn main() {
    /* 变量名前使用" _ "忽略未使用的变量
        a. 创建未使用的变量则 Rust 会给出警告(因其可能为 bug )，但有时
           创建未使用变量是有用的(如设计原型项目之初希望告知 Rust 无需
           警告未使用的变量，可用下划线" _ "作为变量名开头来解决)
        b. " _ "及以下划线开头的名称不同：
           如" _x "仍会将值绑定到变量而" _ "则完全不会绑定(即丢弃)
    */
    let _x = 5;     // 忽略警告(即忽略未使用的"变量")
    let _y = 5;

    let s = Some(String::from("hello"));
    // 匹配则转移所有权
    //if let Some(c) = s {
    if let Some(_c) = s {     // 忽略未使用变量的警告
        println!("found a string");
    }
    // println!("s: {:?}", s); // 若匹配其 s 所有权转换则其后不可再用

    let s = Some(String::from("hello"));
    // 使用" _ "则不会绑定(即"丢弃")
    if let Some(_) = s {
        println!("found a string");
    }
    println!("s: {:?}", s);

}
