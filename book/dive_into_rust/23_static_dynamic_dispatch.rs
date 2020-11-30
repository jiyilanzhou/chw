
/*
0. 动态分派和静态分派
    a. Rust 可同时支持动态分派(dynamic dispatch)和静态分派(static dispatch)
    b. trait 与其它语言中的 interface 区别

23.1 trait object
    指向 trait 的指针就是 trait object

23.2 object safe
    a. 当 trait 有 Self::Sized 约束时
    b. 当函数中有 Self 类型作为参数或者返回类型时
    c. 当函数第一个参数不是 self 时
    d. 当函数有泛型参数时

23.3 impl trait
    目前 Rust 仅支持返回" 具体类型 "而不能返回一个 trait

*/