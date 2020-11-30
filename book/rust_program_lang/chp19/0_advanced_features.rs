
// thunk[θʌŋk]n. 形式转换
/*
0. 高级特性
    a. 不安全 Rust
        舍弃 Rust 的某些安全保障并负责手动维护相对规则
    b. 高级 trait
        关联类型、默认类型参数、完全限定语法(full qualified syntax)、
        超 trait（supertrait)以及与 trait 相关的 newtype 模式
    c. 高级类型
        更多关于 newtype 模式的内容、类型别名、never 类型和动态大小类型
    d. 宏：在编译期生成更多代码的方法

1. 不安全 Rust
    a. 不完全 Rust 之所以存在是因为静态分析从本质上讲是保守的。当编译器在判断
       一段代码是否拥有某种安全保障时，它总是宁可错杀一些合法的程序也不会接受
       可能非法的代码。尽管某些代码也许是安全的但目前 Rust 编译器却可能会做出
       相反的结论！在这种情况下可使用"不安全的代码"但需为自己的行为负责
    b. 另一个需要不安全 Rust 的原因在于底层计算机硬件固有的不安全性。若 Rust
       不允许进行不安全的操作，那么某些底层任务可能根本就完成不了。然而 Rust
       作为一门系统语言需要能够进行底层编程，应当允许直接与操作系统打交道甚至
       编写操作系统，这正是 Rust 语言的目标之一。

2. 不安全超能力
    a. 可以在代码块前使用关键字 unsafe 来切换到不安全模式，并在被标记后的代码块
       中使用不安全代码。不安全 Rust 允许执行"4 种在安全 Rust 中不被允许"的操作
       而它们也就是所谓的"不安全超能力(unsafe superpower)",这些能力包括：
       (0). 解引用裸指针
       (1). 调用不安全的函数或方法
       (2). 访问或修改可变的静态变量
       (3). 实现不安全 trait
    b. 需要注意的是，unsafe 关键字并不会关闭借用检查器或禁用任何其它 Rust 安全
       检查：如果在不安全代码中使用引用，那么该引用依然会被检查。unsafe 关键字
       仅仅是可允许访问这 4 种不会被编译器进行内存安全检查的特性。因此，即使是
       身处于不安全的代码块中也仍然获得一定程度的安全性。
    c. 另外 unsafe 并不意味着块中代码一定就是危险的或一定会导致内存安全问题，它
       仅仅是将责任转移到了程序员的肩上，即需手动确定 unsafe 块中的代码会以合法
       的方法访问内存。
    d. 人为的操作总难免出错，但通过将这 4 种不安全操作约束在拥有 unsafe 标记的
       代码块中，可以在出现内存相关的错误时快速将问题定位到 unsafe 代码块中(但
       应当尽可能避免使用 unsafe 代码块)
    e. 为了尽可能地隔离不安全代码，可以将不完全代码封装在一个安全的抽象中并提供
       一套安全的 API。实际上某些标准库同样使用了审查后的不安全代码并以此为基础
       提供了安全的抽象接口。这种技术可以有效地防止 unsafe 代码泄漏到任何调用它
       的地方，因为使用安全抽象总会是安全的。

3. 解引用裸指针
    裸指针(raw pointer)：分为可变或不可变，它们分别被写作  *const T 和 *mut T。
    在裸指针上下文中，"不可变"意味着不能直接对解引用后的指针赋值。裸指针与引用、
    智能指针的区别在于：
    (0). 允许忽略借用规则，可以同时拥有指向同一个内存地址的可变和不可变指针，或者
         拥有指向同一个地址的多个可变指针
    (1). 不能保证总是指向有效的内存地址
    (2). 允许为空
    (3). 没有实现任何自动清理机制
     // 通过舍弃 Rust 的强加保证，能够以"放弃安全保证"为代价换取更好的性能，或者
        换取与其它语言、硬件进行交互的能力( Rust 的保障在这些领域本就不起作用)
     // 裸指针存在主要用途之一便是与 C 代码接口进行交互，另外还可被用来构造一些
        借用检查器无法理解的安全抽象。

4. 高级 trait
    a. 在 trait 定义中使用关联类型指定占位类型
       关联类型(associated type)是 trait 中的类型占位符，它可以被用于 trait 的
       方法签名中。trait 的实现者需根据特定的场景来为关联类型指定具体的类型。通过
       这一技术可定义出包含某些类型的 trait，而无须在实现前确定它们的具体类型。如
           // 标准库 Iterator trait 定义：带有关联类型 Item
           pub trait Iterator {
               type Item;  // 此处 Item 类型是一个占位符(实现者需其指定具体类型)
               // next 方法表明会返回类型为" Option<Self::Item> "的值
               fn next(&mut self) -> Option<Self::Item>;
           }
           impl Iterator for Counter {
               type Item = u32; // Iterator trait 实现者需为 Item 指定具体类型
               // 在实现的 next 方法中返回一个包含 Item 类型值的 Option
               fn next(&mut self) -> Option<Self::Item> {
                  //-- snip --
               }
           }
           // 借助"关联类型"无需在使用该 trait 的方法时标注类型，因不能为单个类型
              多次实现 trait ，使用关联类型的 trait 对于单个具体类型其 Item 类型
              只能选择一次，即其只能单次实现" impl trait for type "
        // 关联类型类似泛型：但泛型可为同一类型多次实现拥有泛型定义的 trait，并在
           每次实现中改变具体的泛型参数，同时调用具体方法时也必须提供类型标注指明
           欲调用的方法实现。而关联类型针对同一类型只能单次实现拥有关联类型定义的
           trait，因单次具体实现时必须指定关联类型，故调用方法亦无需提供类型标注。
    b. 默认泛型参数和运算符重载
       可以在使用泛型参数时为泛型指定一个默认的具体类型。当使用默认类型就能工作时
       该 trait 的实现者可不再指定另外的具体类型。可在定义泛型时通过语法来为泛型
       指定默认类型(语法:" <PlaceholderType=ConcreteType> ")。这个技术常被运用
       在"运算符重载(operator overloading)"中如默认泛型类型位于 Add trait 定义：
           // 带有一个方法和一个关联类型的 trait
           pub trait Add<Rhs=Self> {
               /// The resulting type after applying the `+` operator.
               #[stable(feature = "rust1", since = "1.0.0")]
               type Output;
               /// Performs the `+` operation.
               #[must_use]
               #[stable(feature = "rust1", since = "1.0.0")]
               fn add(self, rhs: Rhs) -> Self::Output;
           }
          // " Rhs=Self "即是所谓的"默认类型参数(default type parameter)",
             泛型参数 Rhs(right-handle side 缩写)定义了 add 方法中 rhs 参数
             的类型。若在实现 Add trait 的过程中没有为 Rhs 指定一个具体的类型
             那么 Rhs 的类型就会默认为 Self，也就是实现 Add trait 的具体类型。
         // 默认类型参数主要被用于以下两种场景
            (0). 允许在大部分用户都不需要的特定场合进行自定义。如标准库中提供
                 的 Add trait 使用默认参数类型，意味着在大多数情况下都不需要
                 指定额外的参数，换言之即可避免一小部分重复的代码模块从而可以
                 更加轻松地使用 trait
            (1). 扩展一个类型而不是破坏现有代码。与第一种场景些许类似但却采用
                了相反的思路：当想要为现有的 trait 添加一个类型参数来扩展功能
                时，可以给它设置一个默认值来避免已经实现的代码
    c. 完全限定语法：用于消除歧义(调用相同名称的方法)
       " <Type as Trait>::function(receiver_if_method, next_arg, ...); "
       对于关联函数没有 receiver 而只保留剩下的参数列表。可以选择在任何调用函数
       或方法的地方使用完全限定语法。而 Rust 允许忽略能够从上下文中推导的"部分(
       当且仅当存在多个同名实现而 Rust 无法推断预期调用的实现时才需使用这种较为
       烦琐的显式语法)
    d. 超 trait : 用于在 trait 中附带另外一个 trait 功能
    e. 在外部类型上实现外部 trait : 使用 newtype 模式
       孤儿规则：只有当类型和对应 trait 中的任意一个定义在本地包内时才能够为该
       类型实现这一 trait。但实际上还可使用 newtype 模式来巧妙地绕过这个限制，
       它会利用元组结构体(单一字段)创建一个新的类型，即是想要实现 trait 类型的
       瘦封装(thin wrapper[包装类])，由于封装后的类型位于本地包内，故可为这个
       壳类型实现对应的 trait。newtype 是一个来自 Hashkell 编程语言的术语，其
       使用这一模式不会导致任何额外的运行时开销，因为封装后的类型会在编译过程中
       被优化掉。

5. 高级类型             // thunk[θʌŋk]n. 形式转换
    a. 使用 newtype 模式实现类型安全与抽象。比如为类型的某些细节提供抽象能力或
       隐藏内部实现
    b. 使用类型别名创建同义类型：类型别名(type alias)被视作原类型的同义词，其
       使用 type 关键字定义如" type kilometers = i32; "，主用于简化"长类型"
       如"type Thunk = Box<dyn Fn() + Send + 'static>;"，或者减少代码重复
       (如"Result<T,E>"常使用类型别名来减少代码重复，故标准库 std::io 模块中
       使用" type Result<T> = Result<T, std::io::Error>; "类型别名)
    c. 永不返回的 Never 类型       //如" continue、panic!、loop "等
       Rust 有一个名为"!"的特殊类型其在类型系统的术语为"空类型(empty type)"，
       因为它没有任何值即从不返回故更倾向于称之为" never 类型"。
    d. 动态大小类型和 Sized trait (p593[*])
       (0). 动态大小类型：(Dynamically Sized Type，DST)有时亦被称作不确定大小
            类型(即" unsized type ")，此种类型只有在运行时才能确定其大小如 str,
            尽管 &T 被视作存储 T 所在内存地址的单个值，但 &str 实际上是由两个值
            组成(胖指针)：str 的地址(数据的起始位置)与它的长度。可将 str 与所有
            种类的指针组合起来，例如 Box<str> 或 Rc<str> 等。
       (1). 另一种动态大小类型：trait。每一个 trait 都是一个可以通过其名称来进行
            引用的动态大小类型。在第 17 章的"使用 trait 对象来存储不同类型的值"
            一节曾提到过"为了将 trait 用作 trait 对象则必须将其置于某种指针之后"
            如" &dyn Trait、Box<dyn Trait> 或 Rc<dyn Trait> "
       (2). 为了处理动态大小类型，Rust 还提供了一个特殊的 Sized trait 来确定一个
            类型的大小在编译时是否可知。编译时可计算出大小的类型会自动实现此 trait,
            另外 Rust 还会为每一个泛型函数隐式地添加 Sized 约束。也就是说,下面定义
            的泛型函数：
                fn generic<T>(t:T) {//--snip--}
            实际上会被隐式地转换为：
                fn generic<T:Sized>(t:T) {//--snip--}
            在默认情况下，泛型函数只能被用于在编译时已知大小的类型，但可通过如下特殊
            语法来放宽限制：
                fn generic<T: ?Sized>(t: &T) {
                    // --snip--
                }
            " ?Sized trait 约束表达了与 Sized trait 约束相反的含义，即可将其理解为：
            " T 可能是也可能不是 Sized 的"(此语法仅用于 Sized 而不能用于其它 trait),
            另外需要的是将 t 参数类型由 T 修改为 &T (因为类型可能不是 Sized 故需将其
            置于某种指针后[本例选择引用])

6. 高级函数与闭包
    a. 函数指针
       函数会在传递过程中被强制转换为 fn (使用小写 f 从而避免与 Fn 闭包 trait 相混淆)
       类型。fn 类型即是所谓的函数指针(function pointer)。将参数声明为函数指针的语法
       与闭包类似(但与闭包不同，fn 是一个类型而不是一个 trait，因此可以直接指定 fn 为
       参数类型而不用声明一个以 Fn trait 为约束的泛型参数)。由于函数指针实现了全部 3
       种闭包的 trait (Fn、FnMut 及 FnOnce),所以总是可以把函数指针用作参数传递给一个
       接收闭包的函数，也正是出于此原因，更倾向于使用搭配闭包 trait 的泛型来编写函数(
       因其可同时处理闭包与普通函数)。
       另外还有一种十分有用的模式，它利用元组结构体和元组结构枚举变体的实现细节。这些
       类型的初始化语法" () "与调用函数有些相似。实际上它们的构造器也确实被实现为函数
       (该函数接收它们的参数并返回一个新的实例，因此可将构造器视作实现了闭包 trait 的
       函数指针，并在那些接收闭包的方法中使用它们)如：
          enum Status {
              Value(u32),
              Stop,
          }
          /* 使用 Status::Value 构造器调用 map 方法,从而为范围内的每一个 u32 值创建
             对应的 Status::Value 实例，实际编程中一些人倾向于使用这种风格而另外一些
             人则喜欢使用闭包。这两种形式最终都会编译出同样的代码，可按喜好选用即可。
          */
          let list_of_statuses: Vec<Status> = (0u32..20)
                                              .map(Status::Value)
                                              .collect();
         // 完全等价于
         let list_of_statuses: Vec<Status> = (0u32..20)
                                             .map(|i|Status::Value(i))
                                             .collect();
    b. 返回闭包
       由于闭包使用了 trait 来进行表达(意味着不能直接返回闭包)。在大多数预期返回 trait
       的情形下可将一个实现了该 trait 的具体类型作为函数返回值；但无法对闭包执行同样的
       操作，因为闭包没有一个可供返回的具体类型(如无法将函数指针 fn 用作返回类型，因为
       Rust 无法推断需要多大空间来存储返回的闭包)，其解决方案：使用 trait 对象如：
          fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
              Box::new(|x| x + 1)
          }

7. 宏(macro)
   宏是 Rust 中的某一组相关功能的集合称谓，其中包括使用 macro_rules! 构造的"声明宏(
   declarative macro)"及另外 3 种"过程宏(procedural macro)":
   (0). 用于结构体或枚举的自定义 #[derive] 宏，它可以指定随 derive 属性自动添加代码
   (1). 用于为任意条目添加自定义属性的属性宏
   (2). 表象类似函数的函数宏，它可以接收并处理一段标记(token)序列

8. 宏与函数之间的差别
    a. 宏本质是一种用于编写其它代码的代码编写方式，即" 元编程范式(metaprogramming) "
    b. 元编程可以极大程度地减少需要编写和维护的代码数量(元编程对比函数拥有更多功能)
    c. 函数在定义签名时须声明参数的个数与类型，而宏则能够处理可变数量的参数。另外由于
       编译器会在解释代码前展开宏，所以宏可以被用来执行某些较为特殊的任务，比如为类型
       实现 trait 等而函数无法做到(因 trait 需在编译时实现而函数在运行时才调用执行)

9. 用于通用元编程的 macro_rules! 声明宏(p600[*])
   a. Rust 中最常用的宏形式是声明宏(declarative macros)，亦称作 macros by example
      (示例宏)或 macro_rules! (规则宏)或直接简称 macro (宏)。从核心形式而言声明宏
      要求编写些许类似 match 表达式的代码 ：match 表达式是一种接收其它表达式的控制
      结构，它会将表达式的结果值与模式进行比较，并在匹配成功时执行对应分支中的代码。
      类似地，宏也会将输入的值与带有相关执行代码的模式进行比较(此处的值是传递给宏的
      字面 Rust 源代码，而此处的模式则是可以用来匹配这些源代码的结构)。当某个模式
      匹配成功时，该分支下的代码就会被用来替换传入宏的代码(这一切都发生在编译期)。
   b. 声明宏的定义方式 : " macro_rules! macro_name{ //... } "。比如可用 vec! 来
      创建出包含 2 个整数或包含 5 个字符串切片的动态数组，而函数则无法完成同样的事(
      因为无法提前确定值的数量与类型)
   c. 宏模式匹配的是代码而不是值(区别于一般的"模式语法")如：
        #[macro_export]         // 导出外部才可使用
        macro_rules! vec {
            ( $( $x:expr );* ) => { // 一般分隔符选用","(为演示此处选用";"分隔符)
                {
                    let mut temp_vec = Vec::new();
                    $(
                        temp_vec.push($x);
                    )*
                    temp_vec
                }
            };
        }
        fn main() {
            let v: Vec<u32> = vec![1;2;3];  // 元素分隔符对应声明宏中的分隔符
            /*  // 宏展开后的代码
                let mut temp_vec = Vec::new();
                temp_vec.push(1);
                temp_vec.push(2);
                temp_vec.push(3);
                temp_vec
              // 宏(可接收任意数量和类型的参数)生成创建包含指定元素的vector代码
            */
            println!("{:?}",v);     // Console:" [1 2 3] "
        }
    //注：由于 macro_rules! 中存在一些奇怪的技术细节，Rust 开发团队正在致力于推出
         使用 macro 关键字的声明宏，它与现有宏的工作方式类似但修复了某些可能的极端
         情况(更新后 macro_rules! 会被标记为弃用[deprecated])

10. 基于属性创建代码的过程宏
    a. 过程宏更类似函数(某种形式的过程)，其会接收并操作输入的 Rust 代码，并生成另外
       一些 Rust 代码作为结果(区别于声明宏[根据模式匹配来替换代码])
    b. 过程宏存在 3 种不同的类型(自定义派生宏、属性宏及函数宏)且工作机制类似。如:
          use proc_macro;
          #[some_attribute]    // some_attribute 是一个使用特定宏的占位符
          pub fn some_name(input: TokenStream) -> TokenStream { //... }
         // 定义过程宏的函数接收 TokenStream 作为输入,并产生 TokenStream 作为输出。
            TokenStream 类型在 proc_macro 包中定义，表示一段标记序列。这也是过程宏
            的核心所在：需要被宏处理的源代码组成了输入的 TokenStream 而宏生成的代码
            则组成了输出的 TokenStream。函数附带的属性决定创建哪一种过程宏(同一个包
            中可拥有多种不同类型的过程宏)
    // [自]函数附带的属性" #[proc_macro_derive(Trait)]、#[proc_macro_attribute]
       及 #[proc_macro] "分别标识创建"派生宏、属性宏 及 函数宏"

11. 如何编写一个自定义 derive 宏：" #[proc_macro_derive(Trait)] "标识 (p604[*])
    syn 包被用来将 Rust 代码从字符串转换为可供进一步操作的数据结构，而 quote 包能够
    将 syn 包产生的数据结构重新转换为 Rust 代码。
    // " #[proc_macro_derive] "属性宏定义源码[?]

12. 属性宏：" #[proc_macro_attribute] "标识 (p611[*])
    属性宏与自定义派生宏类似，其允许创建新的属性(而非为 derive 属性生成代码)。属性宏
    在某种程度上也更加灵活：derive 只能被用于结构体和枚举,而属性则可以同时被用于其它
    条目(比如函数等)。

13. 函数宏：" #[proc_macro] "标识 (p611[*])
    函数宏可以定义出类似函数调用的宏，但远比普通函数更为灵活。如与 macro_rules! 类似,
    函数宏也能接收未知数量的参数。但 macro_rules! 宏只能使用类似于 match 的语法进行
    定义，而函数宏则可以接收一个 TokenStream 作为参数，并与另外两种过程宏一样在定义
    中可使用 Rust 代码来操作 TokenSteam。

*/