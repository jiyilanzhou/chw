
/*
0. Box 装箱
    作用于指向堆空间数据的栈空间变量(即指针亦称"装箱")
    如" let a = Box::new(5); "// a 指向堆空间数据 5

 */
fn main_0() {
    // 栈空间
    let a = 5;
    let b = &a;
    println!("{}", a == *b);
    // 堆空间("c"为指向堆空间 5 的指针)
    let c = Box::new(5);
    println!("{}", a == *c);
}

/*
1. Rc (Single-threaded reference-counting pointer)单线程引用计数指针
    a. 让一个值拥有多个所有者，调用 clone 可产生一个指向该值的指针
    b. 几个特性
        (0). 当 Rc 指针全部销毁则值也销毁
        (1). 不能通过 Rc 获得可变引用
        (2). Rc 仅用于单线程，多线程使用 Arc

 */
fn print_s0(s: String) {
    println!("{}", s);
}

fn print_rc(s: Rc<String>) {
    println!("{}", s);
}

use std::rc::Rc;

fn main() {
    let s = String::from("chw");
    print_s0(s);
    // println("{}", s);    // 移动后不可再用

    // 使用 Rc (引用计数类比 Go 语言中指向堆空间数据的变量赋值)
    let rc = Rc::new(String::from("683"));
    let rc1 = rc.clone();
    // 传入函数内部的引用计数在函数调用完毕即销毁
    //print_rc(rc1);
    println!("{}", rc);
    println!("{}", Rc::strong_count(&rc));
}
