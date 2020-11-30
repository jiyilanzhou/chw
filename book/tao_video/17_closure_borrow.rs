/*
0.(闭包实现原理)捕捉但未修改环境变量：第三类场景
    捕捉但未修改环境变量 -> 不可变借用（&T）:(第三类场景)等价于如下自定义闭包实现

 */

#![feature(unboxed_closures, fn_traits)]
/* 捕获环境变量匿名结构体：
     (同理)捕获环境变量其匿名结构体属性指定为具体类型(未使用泛型：区别于未捕获环境变量的泛型结构体)
 */
struct Closure {
    env_var: i32,   // 具体类型(用于被捕获的环境变量操作)
}

/* 参数类型源于" FnMut "(因其声明为" pub trait FnMut<Args>: FnOnce<Args> { ... } ")
   又因为" pub trait Fn<Args>: FnMut<Args> { ... } "故 FnOnce 的参数类型取决于子 Trait 即" Fn trait"
 */
impl FnOnce<()> for Closure {
    type Output = ();
    // " self "消耗自身实例(因不修改故无需使用 mut 修饰)
    extern "rust-call" fn call_once(self, args: ()) -> () {
        println!("The answer to the Ultimate Question of life, The Universe, and Everything is {}",
                 self.env_var)
    }
}

// 参数类型源于" Fn "(因其声明为" pub trait Fn<Args>: FnMut<Args> { ... } ")
impl FnMut<()> for Closure {
    extern "rust-call" fn call_mut(&mut self, args: ()) -> () {
        println!("The answer to the Ultimate Question of life, The Universe, and Everything is {}",
                 self.env_var)
    }
}

/* // 标准库 FnMut trait 源码：
    #[lang = "fn"]
    #[stable(feature = "rust1", since = "1.0.0")]
    #[rustc_paren_sugar]
    #[rustc_on_unimplemented(
        on(
            Args = "()",
            note = "wrap the `{Self}` in a closure with no arguments: `|| {{ /* code */ }}`"
        ),
        message = "expected a `{Fn}<{Args}>` closure, found `{Self}`",
        label = "expected an `Fn<{Args}>` closure, found `{Self}`"
    )]
    #[fundamental] // so that regex can rely that `&str: !FnMut`
    #[must_use = "closures are lazy and do nothing unless called"]
    pub trait Fn<Args>: FnMut<Args> {
        /// Performs the call operation.
        #[unstable(feature = "fn_traits", issue = "29625")]
        extern "rust-call" fn call(&self, args: Args) -> Self::Output;
    }
 */
impl Fn<()> for Closure {
    extern "rust-call" fn call(&self, args: ()) -> Self::Output {
        println!("The answer to the Ultimate Question of life, The Universe, and Everything is {}",
                 self.env_var)
    }
}

/*
a. 继承关系：" Fn (不可变借用)继承 FnMut(可变借用) "、" FnMut(可变借用) 继承 FnOnce(所有权) "即实现 Fn(
   不可变借用)须先实现 FnMut(可变借用)，而实现 FnMut 须先实现 FnOce(所有权)。
b. Rust 闭包的实现原理 / 闭包的类型
    (0). 未捕获任何变量则实现 FnOnce、捕获且修改则实现 FnMut、捕获并未修改则实现 Fn
         注：实现是指编译器默认生成的闭包"匿名结构体"实现相应 Trait
    (1). 特殊情况 : 编译器会根据操作 FnOnce 实例情况将其转换为 fn(T) 函数指针；Fn/FnMut/FnOnce 三者 trait
         依次继承(对应于"所有权"语义三件套[所有权、可变借用、不可变借用])

 */
fn main() {
    let answer = 42;
    let mut c = Closure { env_var: answer };
    // c.call_once(()); // 同理调用会消耗 c 实例(其后 c 实例不同再用)
    c.call_mut(());
    c.call(());
}

/*
1. 小结 ：闭包实现原理
    a. 闭包实际上是编译器的语法糖，其会生成匿名结构体，且根据是否捕获环境变量及其捕获变量的类型来选择实现
       相应 Trait
    b. 理解 Fn、FnMut、FnOnce 继承关系，在所有权语义上与"不可变借用、可变借用、所有权"相对应( Rust 语言
       一致性的体现)

 */