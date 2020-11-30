
/*
0. 内存管理基础

10.1 堆和栈

10.2 段错误(segfault : segmentation fault)
    Rust 设计主要目标之一即是不使用自动垃圾回收机制的前提下避免产生 segfault

10.3 内存安全(Memory safety)
    a. Rust 设计的主要目标之一即是内存安全
    b. 内存不安全示例(p113[*])
       (0). 空指针
       (1). 野指针：指的是未初始化的指针
       (2). 悬空指针：指的是内存空间被释放之后继续使用的指针
       (3). 使用未初始化内存
       (4). 非法释放
       (5). 缓冲区溢出：指针访问越界
       (6). 执行非法函数指针
       (7). 数据竞争：并发场景非同步对同一块内存同时读写

*/