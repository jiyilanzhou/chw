
// 闭包  closure
/* 飘红报错：因为闭包的大小在编译期未知
  the trait bound `Fn<(i32), Output=i32>: std::marker::Sized` is not
  satisfied [E0277] `Fn<(i32), Output=i32>` does not have a constant
  size known at compile-time
  解决方案：" Convert to Box "或使用" impl Trait 即 impl Fn(i32)->i32 "
*/
//fn counter_closure(i: i32) -> Fn(i32) -> i32 { //... }

/* 此处闭包类型为" Fn(i32) -> i32 "：
    a. 以大写字母 F 开头的 Fn 是 trait 而并非函数指针类型" fn(i32)->i32 "
    b. 闭包实现可通过函数指针和捕获变量指针组合实现，指针放栈上，捕获变量
       放到堆上。实际上，早期的 Rust 版本实现闭包就采用了类似的方式，因为
       要把闭包捕获变量放到堆上，所以称其为"装箱(Boxed)"闭包,这种方式带来
       的问题就是影响性能。Rust 是基于 LLVM 的语言，这种闭包实现方式使用
       LLVM 难以对其进行内联和优化。故 Rust 团队又对闭包实现做了重大改进，
       也就是当前版本中的闭包实现方式，改进方案称为"非装箱(Unboxed)"闭包，
       此方案是 Rust 语言一致性的再一次体现。非装箱闭包有三个目标：
       (0). 可以让用户更好地控制优化
       (1). 支持闭包按值和按引用绑定环境变量
       (2). 支持三种不同的闭包访问(对应 self、&self 和 &mut self 三种方法)
       实现这三个目标的核心思想：通过增加 trait 将函数调用变为可重载的操作符
       如" 将 a(b,c,d) 这种函数调用变为以下形式：
            Fn::call(&a, (b,c,d))
            FnMut::call_mut(&mut a, (b,c,d))
            FnOnce::call_once(a, (b,c,d)
        Rust 增加这三个 trait 分别就是 Fn、FnMut 和 FnOnce (tao_p174[*])
    c. 标准库提供一系列 Fn trait，而所有的闭包都至少实现了 Fn、FnMut 以及
       FnOnce 中的一个 trait。使用 Fn trait 约束时会在 Fn 的 trait 约束中
       添加代表了闭包参数及闭包返回值的类型(因为每个闭包实例都有自己的匿名
       类型[即便两个闭包拥有完全相同的签名，其编译器分派后的结构体类型也不
       相同])，比如指定的 Fn trait 约束就是" Fn(u32) -> u32 "
    d. 闭包依靠 trait 来实现，跟普通 trait 一样，不能直接使用 Fn、FnMut 或
       FnOnce 作为变量类型、函数参数、函数返回值。与其它 trait 相比闭包相关
       的 trait 约束语法些许特殊，比如欲让闭包作为一个参数传递到函数中可指定
       泛型参数的约束条件为" Fn(i32) -> i32 "(针对闭包设计的专门语法而不是
       像普通 trait 那样使用形似 Fn<i32,i32> 之类的写法)，如此设计让其看似
       跟普通函数类型" fn(i32) -> i32 "(" f "为小写)更相似。向函数传递闭包
       有两种方式：
       (0). 通过泛型：这种方式会为不同的闭包参数类型生成不同版本的函数，实现
            静态分派
       (1). 通过 trait object 方式。这种方式会将闭包装箱进入堆内存中,向函数
            传递一个胖指针，实现运行期的动态分派
       示例：闭包作为参数
            // 泛型参数：对于每个不同的类型参数，编译器将会生成不同版本的函数
            fn static_dispatch<F>(closure: &F)
                where F: Fn(i32) -> i32
            {
                println!("static dispatch {}", closure(42));
            }
            // trait 对象 :" Box< Fn(i32) -> i32 > "
            fn dynamic_dispatch(closure: &Fn(i32) -> i32) {
                println!("dynamic dispatch {}", closure(42));
            }
            // 函数指针
            fn function_ptr(x: i32) -> i32 {
                x * 4
            }
            fn main() {
                let closure = |x| x * 3;
                // 静态分派
                static_dispatch(&closure);
                // 普通函数 fn (不可捕获外部环境变量)亦实现了 Fn trait
                static_dispatch(&function_ptr);
                // 动态分派
                dynamic_dispatch(&closure);
                dynamic_dispatch(&function_ptr);
            }
    e. 闭包作为返回值的两种方案：
       (0). 静态分派 ： impl Trait
       (1). 动态分派 ： trait 对象 (如" Box<dyn Fn(i32)->i32 > ")

*/
// 返回闭包
fn counter_closure(i: i32) -> impl Fn(i32) -> i32 {
    /* 编译报错：因为变量绑定 i 会随着栈帧的释放而释放
       error[E0373]: closure may outlive the current function, but it
                   borrows `i`, which is owned by the current function
          |n| n + i
          ^^^     - `i` is borrowed here
          |
          may outlive borrowed value `i`
        note: closure is returned here
           fn counter_closure(i: i32) -> impl Fn(i32) -> i32 {
                                         ^^^^^^^^^^^^^^^^^^^
        help: to force the closure to take ownership of `i` (and any
                 other referenced variables), use the `move` keyword
           |
           |     move |n| n + i
       [自]解决方案：若为"引用"可标注生命周期，但此例为直接捕获的自由变量为基础
                     数据类型故使用" move "

    */
    // |n| n + i
    move |n| n + i
}
// 返回函数
fn counter_fn(i: i32) -> fn(i32) -> i32 {
    fn inc(n: i32) -> i32 {
        n + 3
        /* 编译报错:
           Rust 不允许 fn 定义的函数捕获动态环境( counter_fn 函数)
           中绑定的变量 i，因为变量绑定 i 会随着栈帧的释放而释放。
            (若需这么做可使用"闭包"[可捕获外部自由变量])
        */
        // n + i
    }
    inc
}
fn main_1_0() {
    // 闭包
    let x = counter_closure(3);
    println!("{}", x(1));
    // 函数
    let y = counter_fn(0);
    println!("{}", y(3))
}

/*
闭包作为函数返回值：tao_p188[*]
    a. 因为闭包是 trait 的语法糖，所以无法直接作为函数返回值，如果要把闭包
       作为返回值必须使用 trait 对象如" Box< Fn(i32) -> i32 > "。如:
            fn square() -> Box<Fn(i32) -> i32> {
                Box::new(|i| i * i)
            }
            // 返回的闭包为 trait 对象 Box<Fn(i32) -> i32>
            let square = square();
            assert_eq!(4, square(2));
            assert_eq!(9, square(3));   // 可多次调用
    b. 同理若预期仅调用一次则新版本可直接指定闭包为 FnOnce (旧版本不支持)
           fn square() -> Box<FnOnce(i32) -> i32> {
               Box::new(|i| i * i)
           }
           let square = square();
           assert_eq!(3, square(2))
        // 注："旧版本"编译报错,
           (旧版本)解决方案：" FnOnce -> 使用 FnBox 代替 FnOnce (曲线救国[
                             有性能消耗:未来版本会被弃用]) -> impl Trait "

*/
fn main() {
    // 新版本可直接指定为" FnOnce "
    fn square() -> Box<FnOnce(i32) -> i32> {
        Box::new(|i| i * i)
    }
    let square = square();
    assert_eq!(4, square(2));
}