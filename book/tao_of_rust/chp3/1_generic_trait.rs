
/*
3.3 泛型 (参阅" p60 ~ p62 ")  // T : Type
    a. 泛型(Generic)是一种参数化多态。使用泛型可编写更为抽象的代码、降低冗余从而提高工作效率。
       简言之泛型就是把泛化的类型作为参数。单个类型就可抽象化为一簇类型。" Box<T>、Option<T>
       和 Result<T> 等都是泛型类型。
    b. 泛型使用须先声明：" <T> "一般用作泛型声明(如"函数、结构体、枚举体"的泛型声明[若未事先
       声明" <T> "就使用泛型" T "则编译器会将其视为作用域范围内某个已存在的普通" T "类型])
    c. 为泛型结构体实现具体方法时亦需声明泛型类型

3.3.1 泛型函数
    除定义类型外泛型亦可用于"函数、结构体、枚举体"等
    a. 泛型函数
           fn foo<T>(x: T) -> T {
               return x;
           }
           fn main() {
               assert_eq!(foo(1),1);
               assert_eq!(foo("hello"),"hello");
           }
    b. 泛型结构体及为其实现方法
           #[derive(Debug, PartialEq)]
           struct Point<T> { x: T, y: T }
           impl<T> Point<T> { // 为泛型结构体实现具体方法时亦需声明泛型类型(泛型欲使用先声明)
               fn new(x: T, y: T) -> Self {
                   Point { x: x, y: y }
               }
           }
           fn main() {
               let point1 = Point::new(1, 2);
               let point2 = Point::new("1", "2");
               assert_eq!(point1, Point { x: 1, y: 2 });
               assert_eq!(point2, Point { x: "1", y: "2" });
           }
    c. 泛型枚举: 如 Result<T,E> 源码
           pub enum Result<T, E> {
               /// Contains the success value
               #[stable(feature = "rust1", since = "1.0.0")]
               Ok(#[stable(feature = "rust1", since = "1.0.0")] T),
               /// Contains the error value
               #[stable(feature = "rust1", since = "1.0.0")]
               Err(#[stable(feature = "rust1", since = "1.0.0")] E),
           }
    d. 标准库提供的各种容器类型大多是泛型类型。如向量 Vec<T> 即是泛型结构体其 Vec<T> 源码：
           pub struct Vec<T> {
               buf: RawVec<T>,
               len: usize,
           }
    e. 单态化(Monomorphization)  // mono[ˈmɒnəʊ]adj&n.单声道/单色/单一(的)
                                // monomorphism[,mɑnə'mɔrfɪzəm]n.单态性
       Rust 中的泛型属于静多态(即编译期多态)。在编译期无论是泛型枚举、泛型函数还是泛型结构体
       都会被单态化(Monomorphization)。单态化是编译器进行静态分发的一种策略。
       // 泛型及单态化是 Rust 的两个重要功能。单态化静态分发优势为无运行时开销从而提升性能但
          容易造成编译后生成的二进制文件膨胀(此缺陷不影响 Rust 编程[可按需重构代码来解决])

*/

/* //(代码清单 3-20 ) 编译期单态化的泛型函数:
        fn foo<T>(x: T) -> T {
            return x;
        }
    // 对于" foo(1) "及" foo("2") "的调用：单态化即是编译器将泛型
       函数生成具体类型对应的函数(如" foo_1 "及" foo_2 ")
    // 编译期泛型函数" fn foo<T>(x: T) -> T "单态化为具体类型对应的函数：
        fn foo_1(x: i32) -> i32 {
            return x;
        }
    // 编译期泛型函数" fn foo<T>(x: T) -> T "单态化为具体类型对应的函数：
        fn foo_2(x: &'static str) -> &'static str {
            return x;
        }
*/
fn foo<T>(x: T) -> T {
    return x;
}
fn main_01_00() {
    // 编译期泛型函数单态化为具体类型对应的函数
    foo(1); // 完全等价于" foo_1(1) "
    foo("2"); // 完全等价于" foo_2("2") "
}

/*
3.3.2 泛型返回值自动推导
    "标注类型"辅助 Rust 自动推导出应调用的代码

*/
// 定义元组结构体
#[derive(Debug, PartialEq)]
struct Foo(i32);
#[derive(Debug, PartialEq)]
struct Bar(i32, i32);
// 定义 Inst trait
trait Inst {
    fn new(i: i32) -> Self;
}
// 为结构体实现 trait
impl Inst for Foo {
    fn new(i: i32) -> Foo {
        Foo(i)
    }
}
impl Inst for Bar {
    fn new(i: i32) -> Self {
        Bar(i, i + 10)
    }
}
// 定义泛型函数
fn foobar<T: Inst>(i: i32) -> T {
    T::new(i)       // 调用泛型 T 的 new 关联函数
}
// "标注类型"指定 Rust 自动推导出应调用的代码
fn main_01_01() {
    // 位置表达式"标注类型"：Rust 可据"标注类型"自动推导出需调用的" Foo::new "
    let f: Foo = foobar(10);
    assert_eq!(f,Foo(10));
    // 值表达式"标注类型"：Rust 可据"标注类型"自动推导出需调用的" Bar::new "
    let b = foobar::<Bar>(20);
    assert_eq!(b,Bar(20,30));
    // "位置、值"表达式皆"标注类型"(无此必要且易导致类型不兼容错误)
    // let f:Bar = foobar::<Bar>(20);
}

/* // (此后相关知识点需参阅"书本"内容)
3.4 深入 trait (参阅" p62 ~ p6 ")
    a. 可以说 trait 是 Rust 的灵魂。Rust 中的所有抽象(如" 接口抽象、OOP范式抽象、
       函数式范式抽象"等)均基于 trait 完成。同时 trait 亦保证这些抽象运行时几乎
       都是零开销的。
    b. 概念: 从类型系统而言 trait 是 Rust 对 Ad-hoc 多态(特定多态)的支持；从语义
       而言 trait 是行为上对类型的约束，该约束可让 trati 有如下 4 种用法：
       (1). 接口抽象 : 接口是对类型行为的统一约束
       (2). 泛型约束 ：限定范围
       (3). 抽象类型 ：运行时动态分发具体类型
       (4). 标签 trait ：对类型的约束可直接作为"标签"使用

3.4.1 接口抽象(" p63 ~ p69 ") : "结构体/枚举体"皆可实现 trait
    // 为不同类型实现 trait 属于一种函数重载，亦可说函数重载就是一种 Ad-hoc 多态
    a. 关联类型: [自] trait 内部 type 定义 Output 类型如
        trait Add<RHS = Self> {
            type Output;
            fn add(self, rhs:RHS) -> Self::Output;
        }
        // Rust 中的很多操作符都是基于 trait 来实现(如"加法"操作符)
    b. trait 一致性        // orphan[ˈɔːfn]n.孤儿
       孤儿规则(Orphan Rule)限制:若要实现某个 trait 则该 trait 及其实现类型至少
       有一个要在当前 crate 中定义，否则标准库相应类型实现的 trait 会被破坏,导致
       难以预料后的 bug
    c. trait 继承
       Rust 不支持传统面向对象的继承但支持 trait 继承

*/
trait Page {
    fn set_page(&self, p: i32) {
        println!("Page Default: 1");
    }
}
trait PerPage {
    fn set_perpage(&self, num: i32) {
        println!("Per Page Default: 10");
    }
}
/* 定义 Paginage trait 并使用冒号表示继承父 trait:
      即 Paginage trait 继承" Page 及 Perpage "父 trait:
      (继承多个父 trait 则使用加号" + "连接 )
*/
trait Paginate: Page + PerPage {
    fn set_skip_page(&self, num: i32) {
        println!("Skip Page : {:?}", num);
    }
}
struct MyPaginate { page: i32 }
impl PerPage for MyPaginate {}
impl Page for MyPaginate {}
// 方式1 ：
// impl Paginate for MyPaginate {}
/* 方式 2 (推荐使用): 为泛型 T 实现 Paginate (包括"空 impl 块")
   (即为所有拥有 Page 及 PerPage 行为的类型自动实现 Paginage)
*/
impl<T: Page + PerPage> Paginate for T {}
fn main() {
    let my_paginate = MyPaginate { page: 1 };
    my_paginate.set_page(2);
    my_paginate.set_perpage(100);
    my_paginate.set_skip_page(12);
}

/*
3.4.3 泛型约束(" p69 ~ p71 ")
    a. trait 限定：使用 trait 对泛型进行约束即" trait 限定(trait bound) "
       格式:" fn generic<T : Trait0 + Trait1 + ... TraitN "即是要求类型 T 须
            实现" Trait0 + Trait1 + ... TraitN "内定义的所有方法方能使用
    b. 理解 trait 限定

3.4.3 抽象类型(" p71 ~ p77 "[待理解]):无法直接实例化
    a. trait 对象 ：
       将共同拥有相同行为的类型集合抽象为一个类型即" trait Object "
       (薛定谔类型 [" p73 " : 待理解])
    b. 装箱 与 拆箱
       装箱抽象类型代表动态分发；拆箱抽象类型代表静态分发
    c. impl Trait 静态分发
       将" impl Trait "语法用于参数位置等价于使用 trait 限定的泛型
       将" impl Trait "语法用于返回值位置等价于给返回类型增加" trait 限定范围 "
    d. dyn Trait 动态分发
       为在语义上与 impl Trait 静态分发相对应专为动态分发的 trait 对象新增 dyn Trait

3.4.4 标签 trait (" p77 ~ p83 ")
    a. Rust 共提供 5 个重要的标签 trait (皆被定义于标准库" std::marker "模块 )
        (1). Sized trait : 用来标识编译期可确定大小的类型
        (2). Unsized trait ：用于标识动态大小类型 DST (目前该 trait 为实验特性)
        (3). Copy trait : 用来标识可以安全地按位复制其值的类型
        (4). Send trait : 用来标识可以跨线程安全通信的类型
        (5). Sync trait : 用来标识可以在线程间安全共享引用的类型
        // Rust 标准库仍继续增加新的标签 trait 以满足变化需求
    b. Sized trait 源码:
          // #[lang = "sized"]      // "sized"语言项(Lang Item)
          pub trait Sized {
             // Empty.
          }
       // Rust 类型多数皆默认为 Sized 故定义泛型结构体时无需显式声明" Sized trait"限定
          如" struct Foo<T>(T); "完整为" struct Foo<T:?Sized>(T); "
    c. Copy trait 源码:
            #[lang = "copy"]
            pub trait Copy : Clone {        // 实现 Copy trait 则须实现 Clone trait
                // Empty.
            }
       // Clone trait 源码:
            #[lang = "clone"]
            pub trait Clone : Sized {
                fn clone(&self) -> Self;
                fn clone_from(&mut self, source: &Self) {
                    *self = source.clone()
                }
            }
       // 实现 Copy trait 则须同时实现 Clone trait (书写繁锁:可用 derive 属性简化)
       // Rust 为很多基础数据类型实现了 Copy trait ：
          (如" 数字类型、字符、布尔、单元值、不可变引用 "等)
        "Copy"的行为是一个隐式的行为，开发者不能重载 Copy 的行为，其永远是一个简单的位复制,
        "Clone trait"是一个显式行为，任何类型皆可实现 Clone trait (开发者可按需实现 Clone
        trait 如"String"并未实现 Copy trait 但却实现 Clone trait [按需调用 String 类型的
        "clone"方法即可])。如若一个类型是 Copy 的则其 clone 方法仅仅需要返回" *self "即可
    Interview ： " Copy "与" Clone "的区别
    d. Send trait 和 Sync trait
       数据竞争是线程安全最大的" 隐患 ": Golang 提供协程和 CSP 并发模型解决方案；Rust 则从
       正面解决问题即依赖"类型系统"及"所有权"机制( Rust 提供了 Send 及 Sync 两个标签 trait
       [其是 Rust 无数据竞争并发的基石])。" Send、Sync "标签 trait 内部无具体方法实现(类比
       "Copy、Sized")。在多线程间传递未实现 Send 及 Sync 的类型则编译报错
        源码:   pub unsafe auto trait Send {
                    // empty.
                }
        源码:   pub unsafe auto trait Sync {
                    // Empty
                }

*/