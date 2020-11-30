
/*
1、闭包是可以保存进变量或者作为参数传递给其它函数的匿名函数。
   闭包和函数不同的是，闭包允许捕获调用者作用域中的值。
2、闭包的使用方式
3、使用带有泛型和 Fn trait 的闭包

*/

// 闭包对应的语法格式示例：
fn add_one_v1(x: u32) -> u32 {
    x + 1
}

fn main() {
    // 闭包([可捕获环境变量的]匿名函数)
    let use_closure = || {
        println!("This is a closure");
    };
    // 调用
    use_closure();

    // 闭包定义会为每个参数及返回值于首次调用时推导并绑定具体类型(即后续不可再次推导)
    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    let add_one_v3 = |x| { x + 1 };
    let add_one_v4 = |x| x + 1;

    // use function
    let a = add_one_v1(5);
    // use closure
    let b = add_one_v2(5);
    let c = add_one_v3(5);
    let d = add_one_v4(5);
    println!("a = {}, b = {}, c = {}, d = {}", a, b, c, d);

    // 闭包参数及返回值类型不能再次推导([自]因初次推断即绑定默认类型)
    let example_closure = |x| x;
    let s = example_closure(String::from("hello"));
    /* 编译报错: 错误类型
       error[E0308]: mismatched types
         let s = example_closure(5);
                                 ^
                                 |
                                 expected struct `std::string::String`, found integer
                                 help: try using a conversion method: `5.to_string()`
          = note: expected type `std::string::String` found type `{integer}`
       // 不能再次推断为非首次调用绑定的类型
    */
    // let s = example_closure(5);
    println!("s = {}", s);
    // 非首次调用须保持参数及返回值与初次调用时绑定的类型一致
    let n = example_closure(5.to_string());
    println!("n = {}", n);

    // 捕捉环境中的变量
    let i = 1;
    let exe = |x| { x + i };
    let r = exe(5);
    println!("r = {}", r);

}