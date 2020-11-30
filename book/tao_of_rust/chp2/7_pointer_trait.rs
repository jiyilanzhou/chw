
/*  // partial[ˈpɑːʃl]adj.部分(的)
2.9 智能指针
    a. 智能指针(Smart Pointer)是 Rust 中极其重要的一种数据结构
    b. Rust 中的值被默认分配到栈内存，可通过 Box<T> 将值装箱(在堆内存中分配)。Box<T> 是指向
       类型为 T 堆内存分配值的智能指针，当 Box<T> 超出作用域范围时将调用其析构函数，销毁内部
       对象并自动释放堆中内存，可通过解引用操作符来获取 Box<T> 中的 T ( Box<T> 表象类似引用
       且可自动释放内存故称其为智能指针[ Rust 提供很多智能指针])
    c. 通过 Box<T> 可方便使用堆内存(确保内存安全)且无须自定义释放
    Interview : 智能指针的作用

*/
// (代码清单 2-50 ) Box<T> 在堆内存中分配值的示例
fn main_007_000() {
    // 定义结构体 Point
    #[derive(PartialEq, Debug)]
    struct Point {
        x: f64,
        y: f64,
    }
    // 使用 Box::new 将" Point { x: 0.0, y: 0.0 } "实例直接装箱(如此其就被分配给了堆内存)
    let box_point = Box::new(Point { x: 0.0, y: 0.0 });
    // 使用解引用操作符解引用可获取内部 Point 实例
    let unboxed_point: Point = *box_point;
    assert_eq!(unboxed_point, Point { x: 0.0, y: 0.0 })
}

/*
2.10 泛型和 trait : Rust 类型系统中至关重要的两个概念
     a. 泛型并非 Rust 特有(很多强类型语言亦支持泛型)，泛型允许编写使用时才指定类型的代码。
        泛型顾名思义即是"泛指的类型"(泛型解决代码复用[否则为所需的每个类型都实现一遍则会
        使得工作量成倍增加])
     b. trait (亦并非 Rust 特有)是 Rust 实现零成本抽象的基石，其有如下机制：
        (1). trait 是 Rust 唯一的接口抽象方式
        (2). 可以静态分发亦可动态分发
        (3). 可当作标记类型拥有某些特定行为的" 标签 "来使用
     c. 简言之:" trait 是对类型行为的抽象 "

2.10.1 泛型
     a. " Rust "标准库中定义了很多泛型类型，包括" Option<T>、Vec<T>、HashMap<K,V> "以及
        " Box<T> "等。其中 Option<T> 就是一种典型使用泛型的类型
     b. 在泛型类型签名中，通常使用字母 T 来代表一个泛型，即是说此 Option<T> 枚举类型对于
        任何类型都适用(如此无需为每个所需实现类型定义一遍 Option 枚举[如" Option<u32> 或
        Option<String> "等])。
     c. 标准库提供的 Option<T> 类型已通过" use std::prelude::v1::* "自动引入每个 Rust 包
        中故可直接使用 Some(T) 或 None 来表示一个 Option<T> 类型而无需写 Option::Some(T)
        或 Option<None>

*/
// (代码清单 2-51 ) Option<T> 定义示例：
enum Option<T> { // 重新声明 Option<T> (因标准库已内置)
    Some(T),
    None,
}
// (代码清单 2-52 ) Option<T> 应用示例：
use std::fmt::Debug;
/* 定义泛型函数：
    " <T: Debug> "为增加 trait 限定的泛型即仅适用于实现了" Debug trait "
    的类型(意味着可用" {:?} "格式化打印[如若去掉 Debug 限定则编译器报错 :
    " error[E0412]: cannot find type `T` in this scope "，亦充分体现了
    Rust 的类型安全保证])
*/
fn match_option<T: Debug>(o: Option<T>) {
    match o {
        Some(i) => println!("{:?}", i),
        None => println!("nothing"),
    }
}
fn main_007_001() {
    // 声明绑定不同具体类型(编译期会生成具体类型实现[零成本抽象])
    let a: Option<i32> = Some(3);
    let b: Option<&str> = Some("hello");
    let c: Option<char> = Some('A');
    let d: Option<u32> = None;
    // 调用
    match_option(a);    // Console:   3
    match_option(b);    // Console: "hello"
    match_option(c);    // Console: 'A'
    match_option(d);    // Console: nothing
}

/*
2.10.2 trait
    trait 与类型行为相关

*/
// (代码清单 2-53 ) trait 示例：
struct Duck;
struct Pig;
/* 定义 Fly trait :
    a. 在 Rust 中 trait 是唯一的接口抽象方式，使用 trait 可让不同类型实现同一行为
       亦可为类型添加新行为。
    b. Fly trait 中 fly 函数签名:包含参数及参数类型、返回值类型但无函数体(亦可为其
       定义函数实现)，函数签名基本反映函数意图，在其返回值类型中甚至还可包含错误处理
       的相关信息(这即是"类型系统"的优势之一：提升可读性)
*/
pub trait Fly {
    fn fly(&self) -> bool;
}
/* 实现 trait :
   a. 形如" impl Trait for Type "(写法语义非常直观)可表达"为 Type 实现 Trait 接口"
   b. 接口抽象：Duck 和 Pig 根据自身类型为同一接口同一函数实现不同行为。Rust 中并无
      传统面向对象语言中的"继承"概念,其通过 trait 将类型和行为明确进行区分，充分贯彻
      了"组合优于继承"及"面向接口编程"的编程思想
*/
impl Fly for Duck {
    // 对 fly 函数增加 Duck 类型的具体实现
    fn fly(&self) -> bool {
        return true;  // duck 能执行 fly 动作故返回 true
    }
}
impl Fly for Pig {
    // 对 fly 函数增加 Pig 类型的具体实现
    fn fly(&self) -> bool {
        return false;   // pig 未能执行 fly 动作故返回 false
    }
}
/* 实现 fly_static 泛型函数：泛型参数声明为 T 代表任意类型：
   " T:Fly "语法形式使用 Fly trait 对泛型 T 进行" 行为上 "的限制，代表实现了 Fly trait
   的类型,这种限制在 Rust 中称为 trait 限定(trait bound),其限定了泛型函数参数的类型范围
   (非"限定范围的类型"传入则编译器会自动襄并报错)
*/
fn fly_static<T: Fly>(s: T) -> bool {
    s.fly()
}
// [自]若将" fn<T: Fly>(s: T) "简化为" fn(s: Fly) "则飘红报错
/* 飘红报错：因编译时无法确定其占用空间大小
       the trait bound `Fly: std::marker::Sized` is not satisfied
       [E0277]`Fly` does not have a constant size known at compile-time
   编译报错：
       error[E0277]: the size for values of type `(dyn Fly + 'static)`
                                  cannot be known at compilation time
          fn fly_s(s: Fly) -> bool { true }
                   ^ doesn't have a size known at compile-time
        = help: the trait `std::marker::Sized` is not implemented for
                                                `(dyn Fly + 'static)`
        = note: all local variables must have a statically known size
        = help: unsized locals are gated as an unstable feature
   // [自]因 Fly trait 内可定义多个函数签名的默认实现故无法确定其占用空间
*/
//fn fly_s(s: Fly) -> bool { true }

/* 实现 fly_dyn 函数：其参数为 &Fly 动态类型，代表所有实现 Fly trait 的类型。
   " fly_static "与" fly_dyn "区别是其函数实现内 fly 方法的调用机制不同
*/
fn fly_dyn(s: &Fly) -> bool {
    s.fly()
}

// [自]反之若将" fn(s: &Fly) "写为" fn<T: &Fly>(s: T)"亦飘红报错
/*(多处)飘红报错：
      a. Function `fly_d` must have a body
      b.  #', <lifetime parameter>, <type parameter> or identifier expected,
          got 'T' '!' or '::' expected, got ':
      c. '!' or '::' expected, got '>'
  编译报错：
      error: expected one of `!`, `(`, `,`, `=`, `>`, `?`, `for`, lifetime,
                                                         or path, found `&`
        fn fly_d<T: &Fly>(s: T) -> bool { false }
                    ^ expected one of 9 possible tokens here
*/
//fn fly_d<T: &Fly>(s: T) -> bool { false }  // 多处飘红报错

fn main_007_002() {
    // 使用 let 声明并绑定
    let pig = Pig;
    // 使用" ::<Type> "语法形式为泛型参数指定具体类型([自]亦可使用自动推导[即不指定])
    assert_eq!(fly_static::<Pig>(pig),false);//多态(其内调用 Pig 实现的 fly 方法)
    let duck = Duck;
    // [自]未使用" ::<Type> "语法形式指定即自动推导具体类型
    assert_eq!(fly_static(duck),true); // 多态(其内调用 Duck 实现的 fly 方法)
    /* 静态分发：
           形如" fly_static::<Pig>(pig) / fly_static(duck) "调用方式即是静态分发。Rust
           编译器会为" fly_static::<Pig>(pig) / fly_static(duck) "此两种具体类型的调用
           生成特殊化代码,即对编译器而言抽象并不存在(因在编译阶段泛型已被展开为具体类型代码)
       动态分发：
           形如" fly_dyn(&Pig) / fly_dyn(&Duck) "即是动态分发，其会在运行时查找相应类型
           的方法(会带来很小的运行时开销)
       零开销原则：
           若不使用某个抽象则无需为之付出开销(静态分发);若是确定需要使用该抽象可保证以最小
           的开销方式使用(动态分发)
    */
    assert_eq!(fly_dyn(&Pig),false);
    assert_eq!(fly_dyn(&Duck),true);

    // 未实现 Fly trait 的类型
    struct Cat;
    impl Cat{
        fn fly(&self) -> bool {
            return false;   // cat 未能执行 fly 动作故返回 false
        }
    }
    let cat = Cat;
    // 未实现 Fly trait 的类型调用即出错
    fly_static(cat);
    // 未实现 Fly trait 的类型调用即出错
    assert_eq!(fly_dyn(&Cat),false);
}

/* Rust 内置很多 trait : 可通过实现内置 trait 来扩展自定义类型行为 ：
   如实现最常用的" Debug trait "即可拥有在 println! 宏语句中使用" {:?} "格式
*/
// (代码清单 2-54 ) 实现 Debug trait

/* (实现方式1)实现 Debug trait 内定义的 fmt 函数：
             以便可直接用 println! 宏语句来打印相应实例值
   (实现方式2)亦可使用在结构体上使用" #[derive(Debug)] "属性自动实现
              (这类属性本质上属于一种" 宏 ")
*/
/* // 实现方式 1：
impl Debug for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        // 使用编译器内置的 write! 宏来实现
        write!(f, "Point {{ x: {}, y: {} }}", self.x, self.y)
    }
}
*/

use std::fmt::{Debug, Formatter, Error}; // 引入所需模块
// 实现方式2：使用属性" #[derive(Debug)] "自动实现 Debug trait
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let origin = Point { x: 0, y: 0 };
    println!("The origin is: {:?}",origin);
}
