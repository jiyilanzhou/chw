
// disambiguation ['dɪsæm,bɪɡjʊ'eɪʃən]n.消歧(消除歧义)    // affine[ə'faɪn]n.仿射
// chalk[tʃɔːk]n.粉笔
/*
0. Rust 系统分为两大概念 : 类型和行为
    a. Rust 类型系统承载的目标主要包括保证内存安全、保证一致性、表达明确的语义及零成本抽象表达能力，为达到
       这四大目标，Rust 类型系统使用了两个相辅相成的概念" 类型 及 行为 "
    b. Rust 一切皆类型，针对不同场景精致划分不同的类型，方便编译期对其静态分析
    c. Rust 又通过 trait 来规范这些类型的行为，力求做到类型安全

1. Rust 中的类型行为
    a. Rust 语言一切皆类型，类型系统如何保证类型的安全交互
       // 编译器如何知道" 4 "能安全转换为 4
        let four: u32 = "4".parse().unwrap();
        assert_eq!(4, four);
        // 定义元组结构体
        struct Point(i32, i32);
        /* 同理：编译器如何知晓"(1, 2)"能安全转换为 Point
              假如你来设计编译器则 parse 该如何设计才能保证转换安全
         */
        let p = "(1, 2)".parse::<Point>();
        assert_eq!(p.unwrap(), Point(1, 2));
    b. 为编译器实现 parse 的一个好思路如
        // 为 u32 实现 from_str 方法
        impl u32 {
            fn from_str(s: &str) -> u32 {
                // 如判断字符串切片内的字符是否为有效的 ASCII 数字
                // do something
            }
        }
        // 同理为 Point 实现 from_str 方法
        impl Point {
            fn from_str(s: &str) -> Point {
                // 将元组字符串用逗号分割提取后组装到 Point 实例返回
                // do something
            }
        }
        // 为 str 实现 parse 且在其内调用各类型实现的 from_str 方法
        impl str {
            pub fn parse<F>(&self) -> Result<F, F::Err> {
                F::from_str(self);
            }
        }
    c. 实现效果(图"15_类型的行为.png")
           "4".parse::<u32>();  // 等价于  u32::from_str("4");
           "(1, 2)".parse::<Point>();   // 等价于 Point::from_str("(1, 2)");
    // 注: 在使用 parse 时程序可正确转换到相应类型 : 编译器在编译时会去查询相应的具体类型( u32 或 Point )
           是否已实现 from_str 方法。但是此解决思路存在很大的一个弊端，若想要使用 parse 方法转换字符串到
           另一种类型则必须看到 parse 方法的内部实现即" F::from_str(&self); "，才能知晓转换的目标类型需
           自定义实现 from_str 方法才能使用 parse 方法，且不同类型间实现的 from_str 方法可复用部分代码，
           如此会造成代码冗余。
    // 注：为解决上述问题，发现一些规律，无论是 u32 还是 Point 类型都需要去调用" from_str "方法，进一步
           抽象而言，它们都遵循相同的行为，如果编译器能识别这种相同的行为并且开发者可以在函数签名就可显式
           指定，则可以极大地增强开发体验，故 Rust 引入了 Trait

2. Trait ：行为抽象
    a. trait 起源及在不同语言中的名称
             haskell -->  类型类(TypeClass)
             Go / Java --> 接口(Interface)
             Rust / Scala --> 特质(Trait)
    b. Rust 的实现更类似 Haskell 类型语言，因为 Rust 借鉴了 Haskell 的类型系统
        // 使用 Rust 的 Trait 对" 1.b "进行类型改造
        // 定义 FromStr trait
        pub trait FromStr {
            // 定义关联类型
            type Err;
            // 定义关联方法( trait [支持仅有方法签名亦支持默认实现])
            fn from_str(s: &str) -> Result<Self, Self::Err>;
        }
        // 为 u32 实现 FromStr
        impl FromStr for u32 {
            type Err = ();
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                // do something
                unimplemented!()
            }
        }
        // 为 Point 实现 FromStr
        struct Point(i32, i32);
        impl FromStr for Point {
            type Err = ();
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                // do something
                unimplemented!()
            }
        }
        // 为 str 实现 parse 方法
        impl str {
            /* 使用 FromStr 对泛型 F 进行限定(且在 parse 方法签名上使用" F::FromStr "形式表示)：
                   parse 方法仅接受实现了 FromStr trait 的类型，换言之" F::FromStr "形式表示拥有 from_str 行为
                   的类型才能被安全地转换，这就是 trait 的基本内涵，它代表了一种约束，是对类型行为的抽象。
             */
            pub fn parse<F: FromStr>(&self) -> Result<F, <F as FromStr>::Err> {
                F::from_str(self);
            }
        }

*/

/*
3. trait 是一种特设多态
    a.  Ad-hoc 多态： 一个接口多个实现。这种多态和动态语言中的鸭子类型相似，如同样是加法操作，数字的加法
        与字符串的加法是不同的行为
    b. Rust 中的加法是用 Trait 来实现
          // 标准库定义的 Add trait ：对应"加号"
          pub trait Add<Rhs = Self> {       // Self 代表实现了 trait 的具体类型
               type Output;
               fn add(self, rhs: Rhs) -> Self::Output;
          }
          // 底数类型实现 add
          impl Add for u32 {
               type Output = u32;
               fn add(self, other: u32) -> u32 {
                    self + other
               }
          }
          // Add trait 引入是为了扩展加法操作，让其对自定义类型亦能实现加法操作[而不仅仅限于标准库的已有
             实现如数字、字符串等]
          // 动态字符串 String 的实现
          impl Add<&str> for String {       // other 参数设置为字符串切片" &str "
              type Output = String;
              fn add(mut self, other: &str) -> String {
                  /*将 other 字符串切片用 push_str 方法将其和 String 字符串相连。这就是两个字符串相加
                    结果是两个字符串连在一起的原因
                  */
                  self.push_str(other);
                  self
              }
          }
          fn main() {
             // Add Trait
             let num = 1 + 1;
             let s = "1".to_string() + "1";
             assert_eq!(num, 2);
             assert_eq!(s, "11");
         }
    c. 是否可修改数字或者字符串的加法行为(孤儿规则：trait 或类型必须有一个在本地定义)
        Trait 及类型至少有一个是在本地定义的方能实现(不能为外部定义的 Trait 及 外部定义的类型重写实现)。
        这就是 Trait 一致性规则(亦称为孤儿规则)。在孤儿规则允许情况下可用 Trait 来对类型进行函数重载。

 */
// 函数重载
struct A;
impl A {
    fn hello(&self) {
        println!(" im A");
    }
}
trait Hello {
    fn hello(&self);
}
impl Hello for A {
    fn hello(&self) {
        println!("from Hello Trait")
    }
}
// disambiguation ['dɪsæm,bɪɡjʊ'eɪʃən]n.消歧(消除歧义)
fn main_0() {
    let a = A;
    // 直接调用会事先查找结构体自身实现的方法,没有再查找为 Trait 实现的同名方法
    a.hello();      // Console:" in A "
    /* 完全无歧义限定语法(Fully Qualified Syntax for Disambiguation)：
           顾名思义就是为了消除同名方法调用的歧义，故其直调用的是结构体为 Trait 实现的相应方法。
           ("完全无歧义限定语法"形似"turbofish"：注意区分)
     */
    <A as Hello>::hello(&a);    // Console:" from Hello Trait "
}

/*
4. trait 掌控类型行为的逻辑( trait 本质上对类型的行为逻辑进行操控)
    a. trait 除了是一种特设多态的约束外，其本质上还对类型的行为逻辑进行操控
    b. Rust 类型系统实质是遵循一种 仿射类型(Affine Type) 的逻辑进行推理。仿射类型是类型系统中的一种，一般
       用于类型系统中标识内存等资源，最多只能被使用一次。而 Copy 等 trait 在此类型系统的逻辑推理中起到至关
       重要的作用，这就是 Rust 语言的类型安全和内存安全的秘密武器。
    c. Rust 内置 trait 分类(图"16_Rust 内置 trait.png")
    d. 与"所有权"相关
       (0). Drop trait : 定义资源如何被析构
       (1). Unpin trait ：代表了使用 Pin 固定后还能安全移动的类型
    e. 与"线程安全"相关
       (0). Sync trait : 代表了类型可在线程间安全共享
       (1). Send trait ：代表类型可在线程间安全转移
            Sync、Send 共同作用下，Rust 在编译期就可以排除一些线程不安全的问题
    f. 与"大小"相关
        Sized trait : 标识可在编译期确定大小的类型
    g. 与"默认值"相关
        Default trait : 标识实现了默认值的类型
    h. 与"智能指针"相关
        Deref trait : 实现 Deref trait (智能指针标识)的类型可进行解引用行为。Rust 某些场合支持自动解引用
                      来提升开发体验([自]如在结构体指针上实现的方法可接收结构体实例[自动解引用])
    i. 与"类型转换"相关 : 类型间转换行为
        (0). From trait :
        (1). Into trait :
        (2). AsRef trait :
    // 以及其它各种 Trait 分别用来标识"比较、闭包捕获变量、错误处理、异步、IO 及网络等"行为。Trait 是 Rust
       的灵魂所在。Rust 语言编译期能保证内存安全、并发安全以及拥有良好的开发体验，Trait 功不可没，因其掌控
       了类型的行为逻辑。实质上在 Rust 编译期内使用了一个叫做 chalk 的 trait 系统(类比逻辑推理引擎)

 */
// trait 掌控类型行为的逻辑
fn main() {
    let a = "Hello".to_string();
    /* 当位置表达式置身于值上下文时，未实现 Copy 的变量绑定会默认发生转移行为.
       转移即是将原来 a 绑定的数据指针及管理权转移给了 b 以致 a 不可再用(所有权).即所有权被转移了，但并非
       所有情况都会发生 Move （如存储于栈空间的基本数据类型实现的是 Copy trait ）. 其 Copy 标识的数字类型
       可安全在栈内存上进行赋值。
     */
    let b = a;
    // a;   // 飘红报错:"Use of moved value ",变量绑定(a)已被 Move 无法再用

    // Copy trait 改变 Move(默认) 的行为逻辑
    let a = 42;
    let b = a;
    a;      // 变量绑定 a 未被 Move ,用于赋值的是拷贝的副本(即所有权未被转移，可再用)
}

/*
5. 小结
    a. trait 除了作为一种行为约束还是一种 Ad-hoc 特设多态(可用 trait 进行函数重载[须遵循孤儿规则])。除此
       之外 trait 在 Rust 中有更加重要的意义即"掌控类型的行为逻辑"从而达到通过类型系统来保证内存安全以及
       并发安全目的
    b. Rust 中内置多种 trait 分类

6. 作业
    参考标准库文档对 Rust 中的内置 trait 做一个详细的梳理和分类

 */