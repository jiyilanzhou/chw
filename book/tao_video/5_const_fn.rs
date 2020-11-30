
/*
0. 常量函数（const fn）/ 常量安全子类型系统
    a. 普通的 fn 关键字定义的函数，是 Safe Rust 主类型系统保证安全。
    b. const fn 定义的函数，是 Safe Rust 主类型系统下有一个专门用于常量计算的子类型系统来保证常量安全。
    c. 而 unsafe fn 定义的函数则是指函数进入了 unsafe Rust 范围，有些操作可以跳过 Safe Rust 主类型系统
       的安全检查，提醒开发者在调用的时候注意，在 const fn 这个安全子类型系统的保护下，不会将编译期计算的
       错误结果引入到运行时。

1. (在引入常量函数的概念后) Rust 目前" 常量上下文能接受的常量表达式 "就只限于以下三种：
    a. const fn 函数
       目前 const fn 还未支持所有表达式，正在逐步支持更多如发布的 Rust1.46 就新加入了 if、match、loop 等
    b. 元组结构体
    c. 元组的值
    d. 反例：

*/
/* 反例 (编译报错)：
    error[E0015]: calls in constant functions are limited to constant functions, tuple structs and
    tuple variants
      "Hello".to_string()
       ^^^^^^^^^^^^^^^^^^^
    原因分析：常量函数返回一个 String 类型，而 String 类型是一个在堆上分配内存的智能指针类型(无法在编译期
    进行计算)，故此处只能使用静态生命周期(即"字面量"字符串)
 */
/* 暂且注释：以通过编译
const fn hello() -> String {
    "Hello".to_string()
}
const S: String = hello();
 */

// 使用字面量
const fn hello() ->  &'static str{
    "hello"
}
const S: &str = hello();

// 常量上下文可接受的常量表达式除常量函数( const fn )外，还可接受"元组结构体及元组"
#[derive(Debug)]
struct Answer(u32);
const A:Answer = Answer(42);    // 常量上下文授受元组结构体

fn main_0() {
    println!("{}", S);     // Console: hello
    println!("{:?}", A);   // Console: Answer(42)
}

/*
2. 编译期计算：
    Rust 编译过程会产生 MIR (中级中间语言)，实际上中级中间语言是一个对控制流图进行编码的一组数据结构(如图
    "2_Rust 编译期计算.png")，控制流图是编译原理的一个经典概念，一个控制流图就是一个有向图，控制流图里的
    每个基本程序块就对应有向图的一个节点，每一个控制转移就对应一个有向图的一条边。然后 Rust 编译器中包含了
    一个 MIR 解释器(简称 Miri),它可以执行中级中间语言代码，编译期计算就是通过 Miri 执行中级中间语言代码来
    完成的。当然准确来说是执行 MIR 中 const 上下文相关的 const 代码。

3. while true 与 loop
    a. 当使用无限循环时，可能会想到 while true 但 Rust 编译器会建议使用 loop
    b. Rust 编译器为什么不识别 while true？
       (0). 要考虑：while (constexpr == true) 的情况
       (1). 使用 #[allow(while_true)] 属性在某些情况下允许使用 while true

 */
// denote[dɪˈnəʊt]v.标志，指示       // infinite[ˈɪnfɪnət]adj.无限(穷)的
fn main() {
    /* 编译警告：无限循环应该使用 loop
        warning: denote infinite loops with `loop { ... }`
                while true{}
                ^^^^^^^^^^ help: use `loop`
            note: `#[warn(while_true)]` on by default
       // 分析：编译器建议使用 loop 来代替 while true
     */
    while true{}
    /* 编译警告及错误：
       warning: denote infinite loops with `loop { ... }` // 无限循环应使用" loop "
       error[E0381]: borrow of possibly-uninitialized variable: `a`
           println!("{}",a);
                         ^ use of possibly-uninitialized `a`
     */
    let mut a;
    /*  // 暂且注释以通过编译：
        #[allow(while_true)]
        while true{
            a = 1;
            break;
        }
     */

    /* 当将 while true {} 换为 loop 时可正常编译：这是为什么呢
       a. 虽然在编译期可识别出字面量，但整体来看 while true 是属于一种特殊的情况：更为普遍的是 while 条件
          表达式(即是否等于 true 这种情况)，如果后面的条件表达式越复杂则越难判断到底是不是 true，为了保持
          语言的一致性，就不能给 while true 开小灶。但若执意要使用 while true 的话则最好加上
          #[allow(while_true)] 属性(这也算是 Rust 设计上的一种妥协，因为 while true 确实让人容易理解为
          无限循环，同时也为习惯于使用 while true 的人[极力推荐使用" loop "更为简单明了])。
       b. while true 也是有限制的，即便添加了 #[allow(while_true)] 上述示例也不会通过编译。不要在 while
          true 内部执行初始化或者返回之类的操作，因编译器不会识别。但是可以在 while true 内修改已经初始化
          的值，这是因为 Rust 要保证内存安全，因为 while true 是表达式则无法确定是否该正确初始化一个变量。
          像这样通过静态分析的方式来提示开发者是个很好的方式。
       c. 同理也不支持 if true 内使用初始化变量或返回

     */
     loop {
         a = 1;
         break;
     }
    println!("{}",a);
}