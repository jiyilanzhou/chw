
// 借用后亦可将其返回：以便调用者能使用
fn takes_ownership(some_string: String) -> String{
    println!("{}", some_string);
    /* 借用后未返回则在函数作用域内部" } "释放回收；
       若返回其"释放回收"时机据调用者作用域而定
    */
    some_string
}

fn makes_copy(i: i32) {
    println!("i = {}", i);
}

fn main() {
    // 指向堆空间数据(大小不固定)
    let s = String::from("hello");
    let s1 = takes_ownership(s);
    /* 同理：将 s 所有权移至 takes_ownership 实参(皆为赋值传参)后使得
             变量 s 失效(因指向已移动)故欲再次使用则编译报错
    */
    // println!("{}",s); // error[E0382]: borrow of moved value: `s`
    //               ^ value borrowed here after move
    println!("{}", s1);

    // 栈上数据([自]基础数据类型[皆实现"copy trait"]大小固定)
    let x = 5;
    makes_copy(x);   // 栈上数据直接拷贝(因未指向堆空间数据故无所有权移动)
    println!("{}", x);  // 栈上数据直接拷贝故可再用(因其所有权不曾移动过)

}