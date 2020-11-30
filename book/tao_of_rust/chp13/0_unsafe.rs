
// variance[ˈveəriəns]n.变化/异,方差
// covariant[koʊˈværiənt]n.协变 // invariant[ɪnˈveəriənt]n.不变
// contravariant[kɔntrə'væriənt]n.逆变 // contra[ˈkɒntrə]n.相反
/*
0. 超越安全的边界

13.1 Unsafe Rust
    a. Unsafe Rust 是 Safe Rust 的一个超集，即在 Unsafe Rust 中决不会禁用
       Safe Rust 中的任何检查
    b. Unsafe Rust 是指在进行以下五种操作时并不会提供任何安全检查
       (0). 解引用裸指针
       (1). 调用 unsafe 函数或方法
       (2). 访问或修改可变静态变量
       (3). 实现 unsafe trait
       (4). 读写 Union 联合体中的字段

13.1.1 Unsafe 语法
    a. unsafe 关键字
    b. unsafe 块

13.1.2 访问和修改可变静态变量
    若多线程同时访问同一可变静态变量则会引起"数据竞争"

13.1.3 Union 联合体
    a. Union 类似 Enum 且更节省内存空间但不安全
    b. Enum 属于 Tagged Union

13.1.4 解引用原生指针/裸指针(Raw Pointer) : p483[*]
    Rust 提供了 *const T (不变) 和 *mut T (可变)两种原生指针

13.2 基于 Unsafe 进行安全抽象

13.2.1 原生指针
    a. 创建空指针
    b. 使用 offset 方法
    c. 使用 read/write 方法
    d. 使用 replace/swap 方法
    e. 使用原生指针进行安全抽象

13.2.2 子类型与型变(p489[*])
    a. 子类型(subtype)在计算机科学中是相对另外一种有替代关系的数据类型(
       父类型)。一般而言可在使用父类型的地方可用子类型替代
    b. 在面向对象语言中，子类型亦称为子类型多态(subtype polymorphism),通过
       多态消除类型间的耦合性，实现统一接口。一般用" 里氏替换原则LSP(Liskov
       Substitution Principle)来描述这种关系：所有引用基类(父类)的地方必须
       能透明地使用其子类对象
    c. 型变(variance)的基本概念    // variance[ˈveəriəns]n.变化/异,方差
        // covariant[koʊˈværiənt]n.协变 // invariant[ɪnˈveəriənt]n.不变
        // contravariant[kɔntrə'væriənt]n.逆变 // contra[ˈkɒntrə]n.相反
       (0). 协变(covariant)
       (1). 逆变(contravariant)
       (2). 不变(invariant)
    d. 未合理使用型变将会引起未定义行为
    e. 使用 PhantomData<T> : 幻影类型     // phantom[ˈfæntəm]n.幽灵,幻影
    f. 协变、逆变与不变类型列表

13.2.3 未绑定生命周期
    Unsafe 代码很容易产生未绑定生命周期(Unbound Lifetime)

13.2.4 Drop 检查
    a. 在 Safe Rust 中由 dropck 引起的问题  // [自]dropck : drop check
    b. #[may_dangle]属性与 dropck
    c. 使用 PhantomData<T> 得到更严格的 drop 检查
    d. 来自标准库的用法
    e. 使用 std::mem::forget 阻止析构函数调用(p503[*])
    f. 在析构函数中手动指定机构顺序

13.2.5 NonNull<T> 指针
    a. NonNull<T> 的本质
    b. 空指针优化

13.2.6 Unsafe 与恐慌安全
    (代码清单) Unsafe Rust 中需要注意恐慌安全问题

13.2.7 堆内存分配(p509[*])
    标准库默认内存分配器: System 分配器

13.2.8 混合代码内存安全架构三大原则(p510[*])

13.3 与其它语言交互
    Common Lisp 语言规范中首次提出了使用术语" 外部函数接口( Foreign Function
    Interface,FFI) "用于规范语言间调用的语言特征。后来该术语逐渐被引用到大多数
    语言中(如"Haskell、Python、Rust、Golang"等)，亦有个别语言使用其它术语(如
    Ada 语言使用"语言绑定"[Language Bindings]、Java 语言则将 FFI 称为 JNI [
    Java Native Interface])

13.3.1 外部函数接口
    a. 应用程序二进制接口(ABI)
    b. 链接与 Crate Type
    c. 交叉编译
    d. extern 语法

13.3.2 与 C 交互
    a. 在 Rust 中调用 C 函数(p515[*])
    b. 在 C 中调用 Rust 函数(p519[*])
    c. 类型匹配与内存布局
    d. 第三方工具介绍

13.3.3 使用 Rust 提升动态语言性能
    为 Node.js 及其它语言写扩展
    // Rust 如何为 Golang 写扩展以致其 GC 自动调用达到内存及时回收

Rust 与 WebAssembly
    WebAssembly 要点
    使用 Rust 开发 WebAssembly
    打造 WebAssembly 生态

小结：
    只有彻底了解什么是"不安全"才能对安全有更深的认知。学习 Unsafe Rust 的过程
    才能对 Safe Rust 有更深的理解( Safe Rust 构建于 Unsafe Rust 之上 )

*/