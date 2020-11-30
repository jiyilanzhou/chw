/*
0. 闭包自身实现了哪些 trait (即 Rust 闭包会默认生成的匿名结构体默认实现的 trait )
    a. Sized            所有闭包默认实现
    b. Copy / Clone     取决于环境变量是否实现 Copy 以及它如何被闭包使用的
    c. Sync/ Send

1. 闭包实现 Copy / Clone 的两条简单规则( Copy/Clone 是伴随实现的两个 trait )
    a. 若环境变量实现 Copy 但闭包以可变借用方式捕获环境变量并对其进行修改，则闭包自身不会实现 Copy
    b. 若环境变量是 Move 语义则闭包内捕获环境变量的操作涉及修改或消耗环境变量，则闭包自身不会实现 Copy
    // 除上述特殊情况闭包一般会自动实现 Copy ,如逃逸闭包(环境变量实现 Copy 且使用 move [闭包实现 Copy])
    // 注：闭包实现源于"防止多个可变借用"(理解闭包实现原理,将所有权与闭包的一致性关联，推断其是否实现 Copy)

 */
// " Fn() + Copy "用于约束判断传入的 fn 是否实现 Copy
fn foo<F: Fn() + Copy>(f: F) {
    f()
}

fn main() {
    let s = "hello".to_string();
    // 捕获环境变量 s 但并未对其进行修改故此处为实现 Fn trait 的闭包(闭包除"1.a/1.b"外一般亦会实现 Copy )
    let f = || { println!("{}", s) };
    foo(f);     // Ok       // 正常编译

    let s = "world".to_string();
    /* 此处使用 move 关键字相当于闭包捕获并消耗环境变量(符合"1.b"规则故闭包自身不会实现 Copy ):
       error[E0277]: the trait bound `String: Copy` is not satisfied in `[closure@src\main.rs:21:13]`
         | fn foo<F: Fn() + Copy>(f: F) {
         |                  ---- required by this bound in `foo`
         |     let f = move || { println!("{}", s) };
         |             ----------------------------- within this `[closure@src\main.rs:21:13: 21:42]`
         |     foo(f);
         |     ^^^ within `[closure@src\main.rs]`, the trait `Copy` is not implemented for `String`
         = note: required because it appears within the type `[closure@src\main.rs:21:13: 21:42]`

        // 据报错信息可知闭包并未实现 Copy，因其捕获的环境变量未实现 Copy 即 Move 语义(符合"1.b"规则)

     */
    let f = move || { println!("{}", s) };
    // println!("{}", s);   // error[E0382]: borrow of moved value: `s`
    //foo(f);     // Error

    let s = "chw";
    // 此处是否使用 move 关键字皆可(静态区数据待深入理解 : [自]上述规则仅适用于堆栈数据[???])
    let f = move || { println!("{}", s) };
    println!("{}", s);      // Ok
    foo(f);     // Ok
}

/*
2. 闭包实现 Sync / Send 的三条简单规则(后续课程详解)
    a. 如果所有捕获变量均实现 Sync 则闭包实现 Sync
    b. 如果环境变量都不是"唯一不可变引用"方式捕获且都实现 Sync 则闭包实现 Send
    c. 如果环境变量是以"唯一不可变引用、可变引用、Copy 或 Move 所有权捕获的则闭包实现 Send

3. 小结
    a. 逃逸闭包与非逃逸闭包
    b. 共享引用可有多个(特例："唯一不可变引用情况"[闭包捕获的环境变量:未使用 move 关键字或未实现 Copy ])
    c. 闭包自身会实现的 trait

 */
