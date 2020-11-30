/*
0. 函数与闭包的一些其它概念
    a. 逃逸闭包与非逃逸闭包
    b. 特殊情况：唯一不可变借用
    c. 闭包自身实现哪些 trait

1. 逃逸闭包与非逃逸闭包
    a. 很多语言都允许函数作为一个返回值
    b. 能被函数返回且不在调用过程中销毁的闭包即是逃逸闭包(否则即为非逃逸闭包)

 */

/* " impl Trait -> Type "为实现了 trait 的一种重载类型，属于静态分配，在" impl trait "语法之前返回的闭包
    往往是一个需要使用 Box_T 来包裹的类型(属于动态分发)

 */
#![feature(unboxed_closures, fn_traits)]
fn c_mut() -> impl FnMut(i32) -> [i32; 3] {
    let mut arr = [0, 1, 2];

    // 使用 move 且数据可 Copy 则操作的是副本
    let mut a = move|| { arr[0] = 3;println!("{:?}",arr);};
    println!("{:?}",arr);       // Console:" [0, 1, 2]
    // 闭包虽然捕获环境变量但未调用则不生效
    a();                    // Console:" [3, 1, 2] "
    println!("{:?}",arr);   // Console:" [0, 1, 2] "

    // 未使用 move 则操作的是原数据
    let mut a = || { arr[0] = 3;println!("{:?}",arr);};
    // 闭包虽然捕获环境变量但未调用则不生效
    a();                     // Console:" [3, 1, 2] "
    println!("{:?}",arr);   // Console:" [3, 1, 2] "

    /* move 关键字专用于为闭包移动所有权，没有 move 关键字则不允许返回闭包(因捕获的 arr 为局部变量[会随着
       函数的调用完毕而消亡，故必须使用 move 关键字将 arr 变量所有权转移到闭包内,如此闭包才可安全返回[逃逸
       闭包])。但又因为 arr 变量是一个基本数据类型(数组是基本数据类型，存储于栈空间[区别于动态数组 vec ])，
       为实现 Copy 的简单类型，所以 move 的动作实际上是将 arr 的拷贝份转移到闭包内。
       error[E0373]: closure may outlive the current function, but it borrows `arr`, which is owned
                                                                            by the current function
          |i| { arr[0] = i; arr }
          ^^^   --- `arr` is borrowed here
          |
          may outlive borrowed value `arr`
        note: closure is returned here
           | fn c_mut() -> impl FnMut(i32) -> [i32; 3] {
           |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^
        help: to force the closure to take ownership of `arr` (and any other referenced variables),
                                                                            use the `move` keyword
           |     move |i| { arr[0] = i; arr }

       // [自]注: 因为转移进闭包内的是 arr 拷贝份，故当 c_mut() 调用结束销毁" let mut arr = [0, 1, 2];"
                  后，闭包内保存的 arr 副本依然有效。 若未使用 move 关键字则闭包内操作是原 arr 数组，当
                  c_mut() 调用结束其销毁" let mut arr = [0, 1, 2];"而返回的闭包内还含有对已销毁的原 arr
                  数组的操作，Rust 编译器肯定不允许故报" 用于操作原数据的操作数(i)生命周期可能比借用的原
                  arr 数据的周期长：may outlive borrowed value `arr` "。故使用逃逸闭包时要确认其是否
                  引用了函数内部的局部变量及其绑定的数据是否实现 Copy trait，若未实现 Copy trait 则无法
                  返回一个局部变量的引用(无论是否使用 move )的闭包(防止出现悬垂指针)
      // 参阅文档"《深入浅出 Rust》"书的" 22.1 及 22.2 "闭包使用场景( p254 ~ 257 )
     */
    // |i| { arr[0] = i; arr }
    move |i| { arr[0] = i; arr }
}

/*
    fn main_0() {
        let i = 42;
        let mut arr_closure = c_mut();
        // arr_closure.call_once((i, ));  // 如何使得匿名闭包自定义实现 FnOnce [???]
        println!("{:?}", arr_closure(i));

    }
    // " for<'a> "为生命周期标注
    fn c_mut2() -> impl for<'a> FnMut(&'a str) -> String {
        let mut s = "hello".to_string();
        /* String 未实现 Copy 为动态可增长的字符串(存储于堆区)
           error[E0507]: cannot move out of `s`, a captured variable in an `FnMut` closure
               |     let mut s = "hello".to_string();
               |         ----- captured outer variable
               |     move |i| { s += i; s}    // 随着数据消亡则返回的、闭包捕获的 s 是一个悬垂指针(杜绝)
               |                        ^ move occurs because `s` has type `String`, which does not
                                                                        implement the `Copy` trait
          // 注：无论是否携带 move 关键字，当 c_mut2() 调用结构即销毁" let mut s = "hello".to_string();"
                 而返回的闭包即捕获的 s 仍持有" "hello".to_string(); "的引用，故编译器不允许(非逃逸闭包)
         //  注：根据 Rust 内存管理机制，从属函数内部创建的数据会随着函数调用结束也一并消亡，即在函数内部
                 创建的数据(如""hello".to_string();")会随着函数的调用结束而销毁。若数据可 Copy 则捕获是
                 数据的副本，函数调用结束虽原数据随即销毁但闭包捕获的变量及其绑定的数据副本仍有效。
         */
        move |i| { s += i; s}
    }
    fn main_() {
        let i = "world";
        let mut arr_closure = c_mut2();
    }
 */


#[derive(Copy,Clone)]
struct Sn {
    id: i32
}

// " for<'a> "为生命周期标注
fn c_mut3() -> impl for<'a> FnMut(&'a Sn) -> Sn {
    let mut s = Sn{id:0};
    move |i| { s.id += i.id; s}
}

fn main_0() {
    let i = Sn{id:3};
    let mut arr_closure = c_mut3();
}

/*
2. 唯一不可变引用
    Rust 不可变引用属于一种共享引用，即不可变引用可有多个(但被闭包捕获[即未使用 move 关键字或未实现 Copy ]
    的数据在闭包未执行前不可再共享引用[因闭包会捕获数据及其所有权])

 */
fn main() {
    let mut a = [1, 2, 3];
    let x = &mut a;
    // 花括号构建一个词法作用域
    {
        /* 编译错误：
        error[E0501]: cannot borrow `x` as immutable because previous closure requires unique access
         |   let mut c = || { (*x)[0] = 0; };       // 首次发生不可变借用(未释放前为"唯一不可变借引用")
         |               --     - first borrow occurs due to use of `x` in closure
         |               |
         |               closure construction occurs here
         |   let y = &x;                            // 再次发生不可变借用(首次借用未释放前杜绝再次借用)
         |           ^^ second borrow occurs here
         |   c();
         |   - first borrow later used here
        */
        // 调用"索引"是一个隐式借用，其会借用一个"不可变借用"
        let mut c = || { (*x)[0] = 0; };
        // let y = &x;                // Error              // 暂且注释以通过编译
        c();
        let y = &x;     // OK
    }       // 出了作用域，其内数据(包含闭包)的生命周期结束、销毁
    let z = &x;
}