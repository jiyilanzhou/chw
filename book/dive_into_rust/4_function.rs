
// diverge[daɪˈvɜːdʒ]v.发散，分歧
/*
0. 函数

4.1 简介
    a. 函数返回可使用 return 语句亦可使用"表达式"，函数参数支持模式解构
    b. 函数未显式声明返回类型则编译器默认其返回类型为 unit 即空 tuple ()
    c. Rust 中每一个函数都是独立的类型，但可转换到 fn 类型
            // 函数
            fn func1(){}
            fn func2(){}
            // 可变绑定
            let mut func = func1;
            func = func2;
            func();
            / * 编译报错:
               error[E0308]: mismatched types
                    func = func2;
                          ^^^^ expected fn item, found a different fn item
               = note: expected fn item `fn() {main::func1}`
                       found fn item `fn() {main::func2}`
               // 解析: 除函数名外即使函数声明相同亦为不同函数(因 Rust 中的每一个函数
                        皆为独立类型)，但其可转换为通用 fn 类型(区别于 Golang )
            * /
     d. 修复方案：转换为通用 fn 类型(注:除函数名外声明必须相同的函数间才能转换)
           // 函数
           fn func1(){}
           fn func2(){}
           fn func3(x:i32){}
           // (可变绑定)修复方案一： 用 as 类型转换
           //let mut func = func1 as fn();
           // (可变绑定)修复方案一： 显式类型标记
           let mut func : fn() = func1;
           func = func2;
           func();
           // 飘红报错:" mismatched types [E0308] expected `fn()`, found `fn(i32)` "
           func = func3;  // 声明必须相同的函数间才能转换
    e. Rust 函数体内也允许定义其它 item 包括"静态变量、常量、函数、trait、类型、模型"等
       主要于" 仅函数内部使用而避免污染外部命名空间 "的场景

*/
fn main() {
    // 函数
    fn func1(){}
    fn func2(){}
    // 可变绑定
    let mut func = func1 as fn();
    func = func2;
    func();
    /* 编译报错:
       error[E0308]: mismatched types
            func = func2;
                  ^^^^ expected fn item, found a different fn item
       = note: expected fn item `fn() {main::func1}`
               found fn item `fn() {main::func2}`
       // 解析: 除函数名外即使函数声明相同亦为不同函数(因 Rust 中的每一个函数
                皆为独立类型)，但其可转换为通用 fn 类型(区别于 Golang )
    */
}

/*
4.2 发散函数(Diverging functions)
    a. 特殊的发散函数其返回类型为" ！ "。若函数不能正常返回则可用" ! "类型如
            fn diverges() -> ! {
                panic!("This function never returns");
            }
    b. 发散函数存在意义：可被转换为任意一种类型如
            if 3==6 {
               panic!("error")
            } else {
               100
            };
    c. 以下情况永远不会返回，故其类型即是" ! "
       (0). panic! 宏及基于其实现的函数或宏(如"unimplemented!、unreachable!")
       (1). 死循环 loop{}
       (2). 进程退出函数 std::process::exit 以及类 libc 中的 exec 一类函数

4.3 main 函数
    a. 在 Rust 中传递参数和返回状态码皆由单独的 API 完成如
       fn main() {
           // 传递参数
           for arg in std::env::args() {
               println!("{}", arg);
           }
           // 返回状态码
           std::process::exit(0);
       }
       // 控制台执行" cargo run arg0 arg1 arg2 "命令
    b. 读取环境变量可用" std::env::var() "及" std::env::vars() "

4.4 const fn
    函数可以用 const 关键字修饰，这样的函数可在编译阶段被编译执行，返回值亦被
    视为编译期常量如:
        const fn cube(num: usize) -> usize {
            num*num*num
        }
        fn main() {
            const DIM:usize = cube(2);
            const ARR:[i32;DIM] = [0;DIM];
            println!("{:?}",ARR);
        }

4.5. 递归函数

*/