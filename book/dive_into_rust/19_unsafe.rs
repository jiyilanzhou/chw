
/*
0. Unsafe

19.1 unsafe 关键字

19.2 裸指针(原生指针)：(p199[*])
    Rust 提供的两种裸指针:" *const T "和" *mut T "(在 unsafe 块中可相互转换)

19.3 内置函数(p201[*])
    标准库" std::intrinsics "模块包含一系列的编译器内置函数

19.3.1 transmute
    " fn transmute<T,U>(e:T) -> U "函数可执行强制类型转换

19.3.2 内存读写
    a. copy
    b. write
    c. read
    d. swap
    e. drop_in_place
    f. uninitialized

19.3.3 综合示例( unsafe 函数用途)

19.4 分割借用

19.5.1 什么是协变(p209)
    a. 协变
    b. 逆变
    c. 不变

19.5.2  PhantomData  (p211)

19.6 未定义行为(p214)


*/