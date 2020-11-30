
// semaphone[ˈseməfɔː(r)]n.信号量   // barrier[ˈbæriə(r)]n.障碍,屏障
// spin[spɪn]v&n.旋转(自旋)； spinlock 自旋锁
// poison[ˈpɔɪzn]v&n. (中)毒,毒(害/药)   // rayon[ˈreɪɒn]n.人造纤维
// cross[krɒs]n.交叉 // beam[biːm]n.横梁 // resume[rɪˈzjuːm]v&n.恢复
// crossbeam['krɔːs,biːm]n.横梁        // yield[jiːld]v&n.产出,收益
// pend[pend]v.搁置,悬挂,待定    // pending[ˈpendɪŋ]v&n.挂起,搁置,待定
// semi[ˈsemi]n.半(导体设备)，常用" semi-* "表示如" semi-coroutine "
// poll[pəʊl]v&n.投票,轮询(顺序访问)
// pin[pɪn]v&n.钉(住),(插)针/标识码
// parallel[ˈpærəlel]n.平行线,并行 // parallelism[ˈpærəlelɪzəm]n.平行,并行
/*
0. 安全并发

11.1.1 多进程与多线程(p363[*])
    a. 进程是资源分配的最小单位，线程是程序执行的最小单元
    b. 从操作系统角度看进程代表操作系统分配的内存、CPU时间片等资源的基本单位。
       其为程序提供基本的运行环境。不同的应用程序可以按业务划分为不同的进程
    c. 从用户角度看进程代表运行中的应用程序，是动态条件下由操作系统维护的资源
       管理实体(非静态应用程序文件)。每个进程都享有自己独立的内存单元从而极大
       地提高程序的运行效率
    d. 线程是进程内的实体无法独立存在，必须依赖进程，线程的系统资源(包括内存)
       来自进程(但占用更小)。每个进程至少拥有一个线程(即主线程)

11.1.2 事件驱动、异步回调和协程(p364[*])
    a. 事件驱动编程
    b. 回调地狱(Call Hell)
    c. 协程(p365[*]):以线程为容器(内存占用及开销更小，无昂贵的系统内核调度
       但却无法使用"多核")

11.1.3 线程安全(p365[*])    // barrier[ˈbæriə(r)]n.障碍,屏障
    a. 竞态条件(Race Condition)与临界区
    b. 数据竞争(Data Race) : 并非所有的竞态条件都是数据竞争，也并非所有的
       数据竞争都是竞态条件
    c. 同步、互斥和原子类型   // semaphone[ˈseməfɔː(r)]n.信号量
       (0). 通常可使用" 锁、信号量(Semaphores)、屏障(Barrier)、条件变量
            (Condition Variable) "机制来实现同步
       (1). 锁根据不同并发场景可分为"互斥锁(Mutex)、读写锁(RWLock)、自旋锁
            (Spinlock)
    d. 原子类型与多线程内存模型

11.2 多线程并发编程
    a. 线程管理
    b. 线程同步

11.2.1 线程管理
    a. 定制线程
        thread::Builder(亦是"thread::spawn"的底层)结构体创建可配置的线程
        (直接使用"thread::spawn"生成的线程默认没有名称，栈大小默认为 2M )
    b. 线程本地存储(Thread Local Storage,TLS):每个线程独有的空间
    c. 底层同步原语
        std::thread::park 提供阻塞线程的基本能力
        std::thread::Thread::unpark 可将阻塞的线程重启
        std::thread::yield_now 主动出让当前线程时间片

11.2.2 Send 及 Sync
    a. 实现了 Send trait 的类型可以安全地在线程间传递所有权
    b. 实现了 Sync trait 的类型可以安全地在线程间传递不可变借用
    c. 与" Send/Sync "相反的标记是" !Send/!Sync "(表示不能在线程间安全传递的类型)
        (0).如" Cell / RefCell "都实现了 " !Sync "：表示无法跨线程共享
        (1).如" RC "实现了 " !Send "：表示无法跨线程移动
    c. 特殊语法
        " unsafe impl Send for .. " : 表示所有类型实现了" Send trait "
        " unsafe impl Sync for .. " : 表示所有类型实现了" Sync trait "

11.2.3 使用锁进行线程同步    // poison[ˈpɔɪzn]v&n. (中)毒,毒(害/药)
    a. 互斥锁(Mutex)
    b. 线程恐慌和错误处理
       在线程获取锁之后发生恐慌的情况称之为" 中毒(Poison) "
    c. 死锁
    d. 读写锁(RwLock)

11.2.4 屏障和条件变量
    屏障 : 一般用于实现线程同步
    条件变量：在运行中每个条件变量只能和一个互斥体一起使用

11.2.5 原子类型
    a. 实现无锁(Lock-Free)并发编程
       (0). Load : 表示从一个原子类型内部读取值
       (1). Store ：表示从一个原子类型内部写入值
       (2). 各种提供原子" 读取 - 修改 - 写入 "的操作
            CAS(Compare And Swap) : 表示比较交换
            Swap : 表示原子交换操作
            Compare-Exchange : 表示比较/交换操作
            Fetch-* : 表示 fetch_add、fetch_sub 等一系列原子加减或逻辑运算
    b. Rust 标准库提供的原子类型
       (std::sync::atomic 模块提供) AtomicBool,AtomicIsize,AtomicUsize,AtomicPtr
    c. 内存顺序

11.2.6 使用 Channel 进行线程间通信
    a. 典语:"不要通过共享内存来通信，而应该使用通信来共享内存 "
    b. 基于消息通信的并发模型主要有两种: Actor 模型和 CSP 模型
        Actor 模型 ：代表语言是 Erlang
        CSP 模型 ：代表语言是 Golang
        // 注: Actor 模型耦合度高于 CSP 模型(因 CSP 模型不关注消息的发送者和接收者)
    c. CSP 并发模型(Communicating Sequential Processes:通信顺序进行)
       Rust 语言中线程就是 CSP 进程而通道就是 Channel
    d. 生产者消费者模式与 Channel
       (0).一般使用一个 FIFO 队列充当中间层
       (1). 异步无界 Channel
       (2). 同步有界 Channel
    e. 死锁
    f. (应用典例)利用 Channel 模拟工作量证明(Proof of Work，POW)

11.2.7 内部可变性探究 : 内部可变性由" UnsafeCell<T> "提供
    Mutex、Cell<T>、RefCell<T>、RwLock<T> 源码

11.2.8 线程池(p399~407)
    参考第三方包" threadpool "( https://crates.io/crates/threadpool )实现

11.2.9 使用 Rayon 执行并行任务      // rayon[ˈreɪɒn]n.人造纤维
    rayon 并行能力基于一种"工作窃取(Work-Stealing)"技术
    参考第三方包" Rayon "( https://github.com/rayon-rs/rayon )

11.2.10 使用 Crossbeam (第三方并发库)
    // cross[krɒs]n.交叉 // beam[biːm]n.横梁  // crossbeam['krɔːs,biːm]n.横梁
    a. 扩展原子类型
    b. 使用 Scoped 线程
    c. 使用缓存行(Cache Line)填充提升并发性能
       伪共享(False Sharing)
    d. 使用 MPMC Channel
       Crossbeam 还提供 std::sync::mpsc 的替代品 MPMC Channel 即多生产者多消费者
       Crossbeam 的 MPMC Channel 亦提供"有界通道"及"无界通道"(类比标准库 Channel)

11.3 异步并发(p412[*])
    a. 异步编程发展的三个阶段：
       (0). 第一个阶段：直接使用回调函数(弊端：回调地狱)
       (1). 第二个阶段: 使用 Promise/Futrue 并发模型，解决"回调地狱"问题但"代码冗余"
       (2). 第三个阶段: 利用协程实现 async/await 解决方案(亦称"异步的终极解决方案")

11.3.1 生成器(Generator)：关键字 yield     // yield[jiːld]v&n.产出,收益
    a. 协程(Coroutine)一般分为两种：有栈协程(Stackful)及无栈协程(Stackless)
       (0). 有栈协程
       (1). 无栈协程：基于状态机(State Machine)
       // Rust 支持无线协程功能
    b. 生成器实现原理
    c. 生成器与迭代器
    d. 用生成器模拟 Future        // semi[ˈsemi]n.半(导体设备)
       半协程(Semi-Coroutine)：严格而言生成器属于一种半协程

11.3.2 Future 并发模式  // poll[pəʊl]v&n.投票,轮询(顺序访问)
    a. Future
       Poll<T> 源码
    b. Executor 和 Task

11.3.3 async/await      // pin[pɪn]v&n.钉(住),(插)针/标识码
    a. async/await 实现原理
    b. Pin 与 UnPin  // Pin<T> 是定义于 std::pin 模块中的智能指针
      (0). 自引用结构体(Self-referential Struct)
      (1). Pin 有"钉"之意，在 Rust 中使用 Pin<T> 则代表将数据内存位置牢牢地"钉"在
          原地不让其移动; 反之 UnPin<T> 则代表被"钉"住数据可安全地移动
    c. async/await 异步开发示例
       配合第三方库" futures-rs "

11.4 数据并行 // parallel[ˈpærəlel]n.平行线,并行 // parallelism[ˈpærəlelɪzəm]n.平行,并行
    a. 任务并行(Task Parallelism) 和 数据并行(Data Parallelism)
    b. 根据 Flynn 分类法，将计算机系统结构分为四类    // flynn[flin]n.弗林(姓氏)
       SISD、SIMD(Single Instruction Multiple Data：单指令多数据)、MISD、MIMD

11.4.1 什么是 SIMD
    术语介绍
    编写 SIMD 数据并行的代码称为"向量化(Vectorization)",因为向量(Vector)是一个指令指令数

11.4.2 在 Rust 中使用 SIMD
    a. SIMD 使用示例
    b. SIMD 命名说明

*/