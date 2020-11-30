
// plain[pleɪn]adj&n.平原,普通的     // slot[slɒt]n.(插)槽
/*
0. 所有权系统
    栈内存是短暂的，会随着栈展开(常见的是函数调用)的过程而被自动清理。而堆内存是
    动态的，其分配和重新分配并不遵循某个固定的模式故需要使用指针来对其进行追踪

5.1 通用概念    // semantic[sɪˈmæntɪk]adj&n.语义(的)
    a. 值语义(Value Semantic)、引用语义(Reference Semantic)
    b. 基本的原生类型(亦称作 POD [Plain Old Data])都是值语义
    c. 智能指针 Box<T> 封装了原生指针，是典型的引用类型。Box<T> 无法实现 Copy
       trait 即意味着其被标记为"引用语义",禁止按位复制[*]

5.2 所有权机制( OwnerShip[p122~p126])
    a. Rust 中可通过能否实现 Copy trait 来区分数据类型的值语义(对应"复制[Copy]"
       语义)和引用语义(对应 Rust 中的"移动"[Move]语义)
    b. 深度复制
    c. 所有权机制 ：Rust 中每个值都有一个所有者，更进一步说 Rust 中分配的每一块
       内存皆有其所有者，所有者负责该内存的释放和读写权限
    d. 所有权的类型系统理论(p124[*])   // affine[ə'faɪn]n.仿射
       Rust 的所有权在系统理论中称为" 仿射类型(affine type) ",它属于类型理论中
       "子结构类型系统(Substructural Type System) : 子结构逻辑(Substructural
       Logic)在类型系统中的应用，子结构逻辑推理规则如下：
       (1). 线性逻辑(Linear Logic)：若变量符合某种特定结构则其内含" 必须且只能
            使用一次 "的规则
       (2). 仿射逻辑(Affine Logic): 类似线性逻辑但其规则为" 最多使用一次 "
    f. 所有权特点
       (1) 如 struct A{
                 a:i32,
                 b:i32
             }
           // A 结构体成员虽皆为复制主义类型但 Rust 也不会默认为 A 实现 Copy，欲
              实现还需手动在 A 结构体上方添加" #[derive(Copy)] "属性
           // 若结构体还有移动语义的成员则无法实现 Copy (因按位复制可能引用内存安全)
       (2) 枚举体和结构体类似，当成员皆为复制语义类型时亦不会自动实现 Copy
       (3) 对于元组类型而言其本身实现了 Copy : 若其元素均为复制语义类型则默认按位
           复制，否则会执行移动语义(p125[*])。注:" 数组、Option 类型亦是如此 "
           // 注：字符串字面量支持"按位复制"如
                  let a = "hello";  // 字符串字面量
                  let b = a;        // 按位复制
                  println!("{}",a); // a 仍可使用

5.3 绑定、作用域和生命周期
    let 即 let binding 之意(即"绑定"语义)

5.3.1 不可变与可变
    声明的绑定默认为不可变

5.3.2 绑定的时间属性--生命周期(p127[*])
    a. 变量绑定具有"时空"双重属性
       (1) 空间属性是指标识符与内存空间进行了绑定
       (2) 时间属性是指绑定的时效性，也就是指它的生命周期
    b. 花括号" {} "：可用于创建词法作用域。另外一些 match 匹配、流程控制、函数或
       闭包等所有使用到花括号的地方都会产生" 词法作用域 "
    c. 循环语句： for、loop 及 while
    d. while let、if let
    e. 函数、闭包

5.4 所有权借用(p131[*])
    a. 函数签名也支持模式匹配
    b. 引用(Reference)与借用(Borrow)
       (1) 引用：是 Rust 提供的一种指针语义，基于指针实现
       (2) 借用：引用" &x "也可称为借用，通过 & 操作符来完成所有权的租借
    c. 借用规则(p133[*])
       为了保证内存安全，借用必须遵循以下三个规则
       (1) 借用的生命周期不能长于出借方(拥有所有权的对象)的生命周期
       (2) 可变借用(引用)不能有别名(Alias)，因为可变借用具有独占性
       (3) 不可变借用(引用)不能再次出借为可变借用
       // 规则(1)是为了防止悬垂指针。规则(2)规则(3)为"共享不可变、可变不共享"
       // 规则(1)很好理解，若出借方已经被析构了但借用依然存在则会产生悬垂指针
          规则(2)规则(3)描述的不可变借用和可变借用相当于内存的读写锁：同一时刻只能
          拥有一个写锁或者多个读锁，不能同时拥有
       // 注：关于解引用值得注意的是"解引用操作会获得所有权"

5.5 生命周期参数
    借用的生命周期不能长于出借方(拥有所有权的对象)的生命周期

5.5.1 显式生命周期参数(p136[*])
    a. 生命周期参数须先声明后使用(类比泛型参数)
    b. 函数或方法返回值的生命周期需与输入参数相匹配，否则标的生命周期将毫无意义
       // 注：禁止没有任何输入参数的情况下返回引用(因其明显会造成悬垂引用)
    c. 指定生命周期参数之间的大小关系
           fn method<'a, 'b: 'a>(s1: &'a str, s2: &'b str) -> &'a str {
               if s1.len() > s2.len() {
                   s1
               } else {
                   s2
               }
           }
        // 其中" 'b:'a "表示 'b 的存活时长大于 'a (因 'a 为返回值的生命周期[
           借用方的生命周期不能长于出借方的生命周期])
        // 生命周期参数目的：帮助借用检查器验证合法引用，消除悬垂指针
    d. 结构体定义中的生命周期参数
       除函数签名，结构体在含有引用类型成员时亦需标生命周期参数(否则编译报错)
    e. 方法定义中的生命周期参数
       为包含引用类型成员(需标注生命周期)的结构体实现方法亦需在 impl 关键字后
       声明生命周期参数(约束输入引用的生命周期长于结构体[成员]的生命周期)
    f. 静态生命周期 'static
       'static 生命周期存活于整个程序运行期间。所有的字符串字面量都有 'static
       生命周期，类型为" &'static str " 如:
            let x = "hello Rust";
            let y = x;  // 按位复制的仅仅是存储于栈上的地址(因数据段可读故安全)
            assert_eq!(x,y);
        // 字符串字面量 x 执行 y=x 赋值操作后 x 继续可用，说明此处赋值执行的是
           按位复制，而非移动语义。
        // 字符串字面量是全局静态类型，它的数据和程序代码一起存储于可执行文件的
           "数据段"，其地址在编译期已知且只读(无法更改)
        // 在 Rust 2018 版本中使用 const 和 static 定义字符串字面量时都可以省掉
           'static 静态生命周期

5.5.2 省略生命周期参数
         生命周期省略规则

5.5.3 生命周期限定

5.5.4 trait 生命周期
    trait 对象及生命周期默认遵循规则(p146[*])

5.6 智能指针与所有权
    a. 智能指针与普通引用的区别之一即是所有权不同：智能指针拥有资源的所有权而普通
       引用只是对所有权的借用
    b. 解引用移动(p147[*])：理论上应使用 trait DerefMove 定义此行为，这也是 Rust
       官方未来预期的打算。目前支持此行为的智能指针只有 Box<T>
    c. "box"关键字(仅在 Rust 源码内部使用):调用内部堆分配方法 exchange_malloc 及
       释放方法 box_free 进行堆内存管理

5.6.1 共享所有权 Rc<T> 和 Weak<T>
    Rc<T> : 主用于单线程(即非线程安全)共享堆上数据(即可供程序多个部分读取)的场景

5.6.2 内部可变性 Cell<T> 和 RefCell<T>
    a. 内部可变性( Interior Mutability )实际上是 Rust 中的一种"设计模式"。内部
       可变性容器是对 struct 的一种封装，表面不可变但内部可通过某种方法来改变
    b. Cell<T> : 内部提供 get/set 方法(类比 OOP 中的 getter/setter )即通过暴露
                 的 get/get 方法实现对内部值的操作。使用 Cell<T> 虽然没有运行时
                 开销但尽量不要使用其包裹大的结构体(应选择某个字段):因为 Cell<T>
                 内部每 get/set 都会执行一次"按位复制"
    c. RefCell<T> : 对于没有实现 Copy 的类型，使用 Cell<T> 有诸多不便。Rust 提供
                    的 RefCell<T> 使用范围更广，对类型 T 并没有 Copy 限制。其内
                    提供的 borrow/borrow_mut 对应 Cell<T> 提供的 get/set 。
                    RefCell<T> 虽未分配空间但其运行时有开销，因其维护着运行时借用
                    检查器(检测是否违反借用规则)。
    d. Cell<T> 及 RefCell<T>  多数应用场景即是配合只读引用使用。如 &T 或 Rc<T>
    f. Cell<T> 和 RefCell<T> 间的区别(p152~153[*])

5.6.3 写时复制 Cow<T>
    a. 写时复制(Copy on Write)技术是一种程序中的优化策略，应用于多种场景。
    b. Cow<T> 是枚举体的智能指针，包括两个可选值( Borrowed[用于包裹引用]、Owned[用于
       包裹所有者])，Cow<T> 表示所有权的"借用"和"拥有"(类似 Option<T> 表示值的"有无")
    c. Cow<T> 提供的功能是"以不变的方式访问借用内容以及需要可变借用或所有权时再克隆一份
       数据"。Cow<T> 实现了 Deref，意味着可直接调用其包含数据的不可变方法。Cow<T> 旨在
       减少复制操作、提高性能，一般用于读多写少的场景
    d. 示例[?]
    e. Cow<T> 另一用处是"统一实现规范"

5.7 并发安全与所有权
    涉及" Send trait 及 Sync trait"、数据竞争

5.8 非词法作用域生命周期(Non-Lexical Lifetime,NLL)    // slot[slɒt]n.(插)槽
    a. 基于 MIR 的借用检查[?]
    b. 栈槽(Stack Slot)
    c. 非词法作用域工作原理可概述为以下两个阶段
       (1). 借用检查第一阶段：计算作用域范围内的借用
            如('a,shared|uniq|mut,lvalue)分别表示(生命周期、共享|独占|可变、左值)
       (2). 借用检查第二阶段：报告错误
    d. NLL 目前可以改善的问题

5.9 小结
    生命周期标注的唯一原则是输出(借用方)的生命周期不能长于输入(出借方)的生命周期

*/