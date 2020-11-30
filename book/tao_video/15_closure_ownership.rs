/*
0. 闭包
    a. Rust 闭包的实现原理
    b. 闭包的具体分类

1. Rust 闭包的实现原理 : 与所有权语义匹配 (Rust 语言的一致性)
    a. 未捕捉环境变量          ->    所有权（Ownership）
            // 未捕获环境变量
            let c1 = || println!("hello");
            c1();
            c1();
            // 虽实现的是 FnOnce 但其结构体亦实现 Copy 故可多次调用(待深入理解[???])
    b. 捕捉但修改环境变量      ->    可变借用（&mut T）
            // 可修改环境变量
            let mut arr = [1, 2, 3];
            let mut c2 = |i| {
                arr[0] = i;
                println!("{:?}", arr);
            };
            c2(0);
    c. 捕捉但未修改环境变量    ->    不可变借用（&T）
            // 未修改环境变量
            let answer = 42;
            let c3 = || {
                println!("The answer to the Ultimate Question of life, The Universe, and Everything
                                                                                    is {}", answer)
            };
            c3();

2. (闭包实现原理)未捕捉环境变量 -> 所有权（Ownership）：(第一类场景)等价于如下自定义闭包实现

*/
#![feature(unboxed_closures, fn_traits)]
/* 自定义闭包实现：
   a. 实际上创建闭包时如"  let c1 = || println!("hello"); "，编译器去解析闭包，生成一个匿名的结构体(类比
      如下泛型结构体" Closure<T> ":其专门包含一个字段，主要用于存储捕获的自由变量)，由于第一类场景即闭包
      未捕获任何环境变量，故在 main 函数中调用时传入单元值" () "
   b. 调用闭包时即是调用匿名结构体实例实现的 FnOnce<Args> trait 下的 call_once 方法。

 */
// [自]因未捕获环境变量：故使用泛型(即针对所有类型的 Cloure 皆适用[区别于捕获变量的非泛型结构体])
struct Closure<T> { // 自定义闭包实现
    env_var: T,     // 使用泛型(因其未捕获环境变量)
}
/* 标准库 FnOnce trait 源码：
    #[lang = "fn_once"]
    #[stable(feature = "rust1", since = "1.0.0")]
    #[rustc_paren_sugar]
    #[rustc_on_unimplemented(
        on(
            Args = "()",
            note = "wrap the `{Self}` in a closure with no arguments: `|| {{ /* code */ }}`"
        ),
        message = "expected a `{FnOnce}<{Args}>` closure, found `{Self}`",
        label = "expected an `FnOnce<{Args}>` closure, found `{Self}`"
    )]
    #[fundamental] // so that regex can rely that `&str: !FnMut`
    #[must_use = "closures are lazy and do nothing unless called"]
    pub trait FnOnce<Args> {
        /// The returned type after the call operator is used.
        #[lang = "fn_once_output"]
        #[stable(feature = "fn_once_output", since = "1.12.0")]
        type Output;

        /// Performs the call operation.
        #[unstable(feature = "fn_traits", issue = "29625")]
        extern "rust-call" fn call_once(self, args: Args) -> Self::Output;
    }
 */
/* 为自定义闭包实现 FnOnce 泛型 trait (泛型参数即是闭包调用时传入的参数)，调用闭包时即是调用匿名结构体实例
   实现的 FnOnce<Args> trait 下的 call_once(self, args: Args) 方法。
   注：call_once 方法首参数为 self, 代表了它会消耗结构体的实例，self 实际上就是对应于所有权
 */
// 因" let c1 = || println!("hello"); "没有捕获环境谈判亦无参数及返回值故皆使用单元值" () "
impl<T> FnOnce<()> for Closure<T> {
    type Output = ();
    extern "rust-call" fn call_once(self, args: ()) -> Self::Output {
        println!("hello");
    }
}
fn main_0() {
    // 第一类场景即是闭包未捕获环境变量故传入参数" () "
    let c = Closure { env_var: () };
    c.call_once(());
    /* // FnOnce trait 的 call_once 会消耗实例
        error[E0382]: use of moved value: `c`
               let c = Closure { env_var: () };
                   - move occurs because `c` has type `Closure<()>`, which does not implement the `Copy` trait
               c.call_once(());
                 ------------- `c` moved due to this call
               c.call_once(());
               ^ value used here after move
        note: this value implements `FnOnce`, which causes it to be moved when called
        // 既然消耗实现那么" let c = || println!(); "中的 c() 为何却可以多次调用[???]
     */
    // c.call_once(());     // 暂且注释以通过编译

}

/*
3. (闭包实现原理)闭包未捕获环境变量(无论闭包是否有无参数及返回值[第一类场景]):
    a. 编译器操作实现 FnOnce 的闭包(即未捕获环境变量)类型时将其转为函数指针
    b. 不同参数及返回值的的闭包(本质即函数)肯定不是同一类型

 */
fn main() {
    // 携带参数的闭包
    let c0 = |i32| { "c0" };
    // 未显示声明 c0 类型由编译器自动推导：函数项(零成本大小优化)
    println!("{}", std::mem::size_of_val(&c0));     // Console:" 0 "
    let c1: fn(i32) -> &'static str = |i: i32| { "c1" };
    // 显示声明为指针类型：函数指针(占用一定空间)
    println!("{}", std::mem::size_of_val(&c1));     // Console:" 8 "
    let c2 = |i: i32| { "c2" };
    // 操作时将实现 FnOnce 的闭包转为函数指针
    let v = [c0, c1, c2];

    // 未携带参数的闭包
    let c1= || { println!("c1"); };
    let c2= || { println!("c2"); };
    let v = [c1, c2];               // 正常编译(说明为同一类型)
    let i = "c3";
    let c3 = || { i };
    /* 编译错误：
        error[E0308]: mismatched types
            let c3 = || { i };
            |              -------- the found closure
            |      let v = [c1, c2, c3];
            |                       ^^ expected fn pointer, found closure
            = note: expected fn pointer `fn()`
                          found closure `[closure@src\main.rs:97:14: 97:22]`
     // 解析：组装数组时 c1,c2 实例(实现 FnOnce:可被转换为函数指针类型)为函数指针而 c3 (实现 Fn )仍为闭包
     */
    // let v = [c1, c2, c3];    // 暂且注释以通过编译

}