/*
0.(闭包实现原理)捕捉但修改环境变量：第二类场景
    捕捉但修改环境变量 -> 可变借用（&mut T）:(第二类场景)等价于如下自定义闭包实现

 */

#![feature(unboxed_closures, fn_traits)]
/* 捕获环境变量匿名结构体：
     因需要捕获环境变量故其匿名结构体属性指定为具体类型(未使用泛型：区别于未捕获环境变量的泛型结构体)
 */
struct Closure {
    env_var: [i32; 3],   // 具体类型(用于被捕获的环境变量操作)
}

// 参数类型源于" FnMut "(因其声明为" pub trait FnMut<Args>: FnOnce<Args> { ... } ")
impl FnOnce<(i32, )> for Closure {
    type Output = ();
    /* " mut self "消耗自身实例 : 为与子 Trait 即 FnMut 一致此处使用 mut 修饰（但为符合 FnOnce 定义实际
       应为" extern "rust-call" fn call_once(mut self, args: (i32, )) -> () { ... } ")
     */
    extern "rust-call" fn call_once(mut self, args: (i32, )) -> () {
        self.env_var[0] = args.0;
        println!("{:?}", self.env_var);
    }
}

/* // 标准库 FnMut trait 源码：
    #[lang = "fn_mut"]
    #[stable(feature = "rust1", since = "1.0.0")]
    #[rustc_paren_sugar]
    #[rustc_on_unimplemented(
        on(
            Args = "()",
            note = "wrap the `{Self}` in a closure with no arguments: `|| {{ /* code */ }}`"
        ),
        message = "expected a `{FnMut}<{Args}>` closure, found `{Self}`",
        label = "expected an `FnMut<{Args}>` closure, found `{Self}`"
    )]
    #[fundamental] // so that regex can rely that `&str: !FnMut`
    #[must_use = "closures are lazy and do nothing unless called"]
    pub trait FnMut<Args>: FnOnce<Args> {
        /// Performs the call operation.
        #[unstable(feature = "fn_traits", issue = "29625")]
        extern "rust-call" fn call_mut(&mut self, args: Args) -> Self::Output;
    }
    //  FnMut 继承 FnOnce 故 FnMut 内并未定义关联类型(因 FnOnce 已定义)。Rust 中 trait 的继承要求想要
        实现" 子 Trait "则必须先实现" 父 Trait "
 */
impl FnMut<(i32, )> for Closure {
    /* " &mut self "可变引用(因修改故使用" mut "修饰)：对应"所有权"三件套中的"可变借用"
       Compiling tao_video v0.1.0 (E:\chw\book\tao_video)
          error: internal compiler error: /rustc/8dae8cdcc8fa879ceae97be4c306\compiler\rustc_middle
                \src\ty\layout.rs:2587:21: argument to function with "rust-call" ABI is not a tuple
        // 说明：FnOnce 中的 call_once 及 FnMut 中的 call_mut 传入的第二个参数必须是元组类型(源于前面
                的 extern "rust-call" 定义，其代表 Rust API 约定。若为其它类型则报上述错误。)
     */
    extern "rust-call" fn call_mut(&mut self, args: (i32, )) -> () {
        self.env_var[0] = args.0;
        println!("{:?}", self.env_var);
    }
}

fn main() {
    let arr = [1, 2, 3];
    let mut c = Closure { env_var: arr };
    /* error[E0382]: borrow of moved value: `c`
          c.call_once((0, ));      // 会消耗 self 所以如果调用则其后闭包实例 c 不可再用
          ---------------- `c` moved due to this call
          c.call_mut((0, ));
          ^ value borrowed here after move
     */
    c.call_mut((0, ));
}