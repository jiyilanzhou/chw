
/*
2.7.3 枚举体
    枚举体(Enum 亦称为枚举类型或枚举)，顾名思义枚举类型包含了全部可能的情况，可有效防止用户
    提供无效值。在 Rust 中枚举类型可用 enum 关键字来定义且有三种形式：无参枚举、类 C 枚举(
    Rust 亦可编写类似 C 语言中某种形式的枚举体)及有参枚举。
    // 枚举体是 Rust 中非常重要的类型之一，为编程提供诸多方便且避免出现空指针
*/
// (代码清单 2-39 )无参数枚举体示例
// 定义枚举体 Number ：其内包含"Zero、One、Two"三个"值"(而非"类型")
enum Number{
    Zero,
    One,
    Two,
}
fn main_005_000() {
    // 获取枚举值需使用枚举名" EnumName "前缀
    let a = Number::One;
    // 可用 match 匹配枚举所有的值以处理相应情况
    match a {
        Number::Zero=>println!("0"),
        Number::One=>println!("1"),
        Number::Two=>println!("2"),
    }
}

// (代码清单 2-40 )类 C 枚举体示例
// 定义 Color 枚举体(包含三个枚举值[Red、Green 和 Blue]且分别被赋予相应值)
/* 枚举体包含"枚举值"(枚举体中的成员是值[而非类型:区别于结构体])：
   问题：枚举值(Red、Green 和 Blue)应为值表达式则其为何还能被赋予相应值 ？
         [即值表达式处于位置表达式环境为何未报错 ? ])

*/
#[derive(Debug, PartialEq)]
enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000fff,
    RGB,        // 默认" 4096 = 2^12 "(与计算机存储相关)
    // 飘红报错:" mismatched types [E0308] expected `isize`, found `bool`＂
    // CHW = true,  // ([自]枚举值仅允许" isize "数值类型 ? )
    NUM = 3,
}
fn main_005_001() {
    // (同理)访问具体枚举值需使用" EnumName "前缀
    println!("roses are #{:06x}",Color::Red as i32);      // Console:" #ff0000 "
    println!("violets are #{:06x}",Color::Green as i32);  // Console:" #00ff00 "
    println!("violets are {:?}",Color::Green);            // Console:" Green "
    println!("rgb are {:?}",Color::RGB);                  // Console:" RGB "
    println!("rgb are {}",Color::RGB as i32);             // Console:" 4096 "
    println!("num are {:06x}",Color::NUM as i32);         // Console:" 000003 "
    println!("num are {}",Color::NUM as i32);             // Console:" 3 "
}

// (代码清单 2-41 )带参数枚举体示例
/* 定义枚举体 IpAddr:
   枚举值携带类型参数属于一种类型构造器，此类枚举值本质属于函数类型(Fn-Item Type)，可通过显式
   指定类型来转换为函数指针类型(Fn-Pointer Type)，但非必要请勿显式转换(默认函数项性能更佳)
*/
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}
fn main_005_002() {
    /* 显式指定类型来转换为函数指针类型：
           IpAddr::V4 是 fn(u8, u8, u8, u8) -> IpAddr 函数指针
           IpAddr::V6 是 fn(String) -> IpAddr 函数指针
       使用携带参数的枚举值类比有参函数调用(需传入实参)
    */
    let x: fn(u8, u8, u8, u8) -> IpAddr = IpAddr::V4;
    let y: fn(String) -> IpAddr = IpAddr::V6;
    let home = IpAddr::V4(127, 0, 0, 1);
}

// (代码清单 2-42 )枚举体应用示例
/* 定义 Option 枚举类型(表示有无值两种情况):
    a. Some(i32) 代表有 i32 类型的值而 None 代表无任何值
    b. Option 类型可作为某些函数返回值：若函数有合法的值返回则使用 Some(i32)
       枚举值若函数返回空则可用 None。如此函数返回值已确定为有无值两种(调用者
       可分别处理从而提升程序的健壮性)
    c. Option 类型可有效避免开发中出现 Null 值故 Rust 标准库亦内置了相应类型
       即泛型枚举体" Option<T> "以便开发中可直接调用。内置 Option<T> 源码：
          源码： pub enum Option<T> {
                    /// No value
                    #[stable(feature = "rust1", since = "1.0.0")]
                    None,
                    /// Some value `T`
                    #[stable(feature = "rust1", since = "1.0.0")]
                    Some(#[stable(feature = "rust1", since = "1.0.0")] T),
                 }
*/
/* // 自定义 Option 枚举体(标准库已内置" Option<T> "枚举)
enum Option {
    Some(i32),
    None,
}
*/
fn main_005_003() {
    /* 使用 let 绑定 s 值为 Some(42)：因值确定故可用 unwrap 方法将 Some(42)
       中的数字 42 取出来(若未确定情况下使用 unwrap 则可能导致运行时错误)
    */
    let s = Some(42);
    let num = s.unwrap();
    // 可用 match 匹配枚举值情况并分别处理
    match s {
        Some(n) => println!("num is {}", n),    // Console:" 42 "
        None => (),
    }
}

// (代码清单 2-43 ) Option<T> 示例
fn main() {
    let s: &Option<String> = &Some("hello".to_string());
    /*// Rust 2015 版本 ：match 匹配引用:
      a. 可直接使用" Some<T> "("T"泛型[此处具体类型为"&str"字符串])
      b. 在 match 匹配分支中使用 &Some(ref s) 匹配模式：其目的是为了
         解构 &Some("hello".to_string())。其中 ref 也是一种模式匹配，
         是为了解构 &Some(ref s) 中 s 的引用(避免其中的 s 所有权转移)
    */
    match s {
        &Some(ref s) => println!("s is {}", s),
        _ => (),
    }

    // Rust 2018 版本 ：match 匹配引用
    match s {
        // 新版本无需再使用引用操作符"&"和"ref"进行解构，match 会自动处理
        Some(s) => println!("num is {}", s),
        _ => (),
    }
}
