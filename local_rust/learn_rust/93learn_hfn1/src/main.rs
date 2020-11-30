
/*
1、函数指针
    a. 函数指针允许使用函数作为另一个函数的参数。
    b. 函数类型 fn (即函数指针：作为参数[类比闭包])
    c. 函数指针亦实现了闭包的三个特征("Fn、FnMut、FnOnce")
          Fn : 引用
          FnMut : 可变引用
          FnOnce : 消费环境中的值
       // 函数指针实现了闭包的三个 trait 故可使用闭包作为形参
          的地方亦可替换为使用"函数(指针)"作为参数

*/

// 普通函数
fn add_one(x: i32) -> i32 {
    x + 1
}
// 函数指针作为参数的函数
fn do_twice(f: fn(i32) -> i32, val: i32) -> i32 {
    f(val) + f(val)
}
fn wapper_func<T>(t: T, v: i32) -> i32 
    where T: Fn(i32) -> i32 {
    t(v)
}

fn func(v: i32) -> i32 {
    v + 1
}

fn main() {
    let r = do_twice(add_one, 5);
    println!("r = {}", r);

    // 闭包作为参数
    let a = wapper_func(|x| x+1, 1);
    println!("a = {}", a);
    // 函数(指针)作为参数
    let b = wapper_func(func, 1);
    println!("b = {}", b);

}