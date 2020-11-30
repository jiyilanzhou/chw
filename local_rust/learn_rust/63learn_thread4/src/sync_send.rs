
/*
1、内嵌于语言的两个并发概念:
    " std::marker "中的 sync 和 Send trait

2、通过 send 允许在线程间转移所有权
    a. Send 标记 trait 表明类型的所有权可以在线程间传递。几乎所有的 Rust 类型都是 Send 的
       但亦有例外例如" Rc<T> "是不能 Send 的
    b. 任何完全由 Send 类型组成的类型也会自动被标记为 Send
       如"由皆为 send 类型成员组成的结构体亦标记为 send "

3、sync 允许多线程访问
    a. sync 标记 trait 表明一个实现了 Sync 的类型可以安全的在多线程中拥有其值的引用，即对于
       任意类型 T，若 &T ("T"的引用）是 Send 则 T 就是 sync 的，这意味着其引用可安全的发送
       到另一个线程。
    b. 智能指针 Rc<T> 亦不是 sync 的(出于其不是 Send 相同的原因); Refcell<T> 和 CeLL<T>
       系列类型不是 sync 的。 Refcell<T> 在运行时所进行的借用检查也不是线程安全的
    c. Mutex<T> 是 Sync 的

4、手动实现 Send 和 sync 是不安全的
   通常并不需要手动实现 Send 和 Sync trait ，因为由 Send 和 Sync 类型组成的类型，自动就是
   Send 和 Sync 的。因其标记 trait，甚至不需要实现任何方法(仅用来加强并发相关的不可变性)

*/