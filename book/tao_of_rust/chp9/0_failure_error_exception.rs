
// unwind[ˌʌnˈwaɪnd]v. 展开   // wind[wɪnd]v&n. 风,缠绕
/*
0. 构建健壮的程序
    栈回退(Stack Unwind)
    栈回溯(Stack Backtrack)

9.1 通用概念
    失败(Failure)、错误(Error)、异常(Exception)

9.2 消除失败
    Rust 使用以下两种机制来消除失败
    a. 强大的类型系统
    b. 断言

9.3 分层处理错误
    Option<T>
    Result<T,E>
    Panic : 线程恐慌(在 Rust 中线程发生恐慌就是异常)
    Abort : 线程中止
    // 问号语法糖(try!宏) : (p313[?])
    // 问号语法糖相关 trait : " std::ops::Try "(p314[*])

9.4 恐慌
    栈回退(Stack Unwind)机制
    恐慌安全(Panic Safety)

9.5 第三方库
    failure

9.6 小结
    a. Option<T> 与 Result<T,E>
    b. 错误 与 异常
    c. catch_unwind 可有限制地捕获线程恐慌
    d. 问号语法糖简化基于 Result<T,E> 的错误处理机制

*/