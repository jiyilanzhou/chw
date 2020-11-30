/*
0. 枚举与模式匹配

1. 定义枚举
   枚举值只能是枚举变体(variant)中的一个成员

2. 枚举值
    a. 枚举体包含"枚举值"(枚举体中的成员即"变体"是值而非类型[区别于结构体])
         enum IpAddrKind {
             V4,             // [自]类似" 空结构体 V4 {} 实例 "
             V6,
         }
    b. 可将关联数据嵌入枚举变体内(避免将枚举集成至结构体)如
         enum IpAddrKind {
             V4(String),        // 可在枚举的变体中嵌入任意类型
             V6,
         }
         // 关联数据嵌入枚举变体
         let home = IpAddrKind::V4(String::from("127.0.0.1"));
    c. 使用枚举代替结构体的另一优势:每个变体可拥有不同类型和数量的关联数据如
         enum IpAddr{
             V4(u8, u8, u8, u8),  // 可在枚举的变体中嵌入任意数量的类型
             V6(String),
         }
         let home = IpAddr::V4(127, 0, 0, 1);
         let loopback = IpAddr::V6(String::from("::1"));
    d. 枚举 与 结构体
       (0). 虽然结构体可存储与枚举变体完全一样的数据，但是每个结构体都拥有自身的
            类型，如此无法轻易定义一个能够统一处理这些类型的函数(枚举可轻松应对)
       (1). 类比结构体，同理可用 impl 关键字定义枚举的方法(同样使用 self 来获得
            调用此方法的实例)

3. Option 枚举
    a. 由于 Option<T> 枚举十分常见且很有用故将其包含在" 预导入模块(prelude) "
       中，即不需要显式将其引用作用域，如此其变体亦可以在不加" Option:: "前缀的
       情况下直接使用 Some(T) 或 None
    b. 源码(位置"..\src\libcore\option.rs"):
            pub enum Option<T> {
                /// No value
                #[stable(feature = "rust1", since = "1.0.0")]
                None,
                /// Some value `T`
                #[stable(feature = "rust1", since = "1.0.0")]
                Some(#[stable(feature = "rust1", since = "1.0.0")] T),
            }

*/
fn main_0() {
    let some_number = Some(5);
    let some_string = Some("a string");
    /* 编译报错:
        error[E0282]: type annotations needed for `std::option::Option<T>`
           let absent_number = None;
               -------------   ^^^^ cannot infer type for type parameter `T` declared
                                                                  on the enum `Option`
               |
               consider giving `absent_number` the explicit type `std::option::Option<T>`,
                                                where the type parameter `T` is specified
       // 解析：使用 None (而非 Some<T>)变体进行赋值须指定 Option<T> 类型，因为单独的 None
                变体值与持有数据的 Some 不一样，编译器无法根据 None 变体值推导出"值的正确类型"
    */
    //let absent_number = None;
    let absent_number: Option<i32> = None;
}

/*
4. 控制流运算符 match (穷尽匹配 )
    a. match 与 if 表达式
       巨大区别： if 语句中表达式需要返回一个"布尔值",而 match 的表达式则可以返回任何类型。
    b. match 分支由模式及其所关联的代码组成，并使用" => "运算符用于将"模式"和"代码"分开，分支
       间使用逗号" , "分隔
    c. 每个分支所关联的代码同时也是一个表达式，其运行的结果值被作为整个 match 表达式返回值返回

5. 绑定值的模式
   匹配分支在于其可以绑定被匹配对象的部分值，而这正是" 从枚举变体中提取值的途径 "

*/
fn main_1() {
    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    // 飘红报错:" mismatched types [E0308] expected `i8`,found `Option<i8>` "
    /* 编译错误:
        error[E0277]: cannot add `std::option::Option<i8>` to `i8`
             let sum = x + y;
                         ^ no implementation for `i8 + std::option::Option<i8>`
           = help: the trait `std::ops::Add<std::option::Option<i8>>` is not
                                                           implemented for `i8`
        // 分析:" i8 与 Option<i8> "为不同类型
    */
    // let sum = x + y;

    /* 欲使用 Option<T> 中可能存在的 T ：
           需将其转换为 T (可用 match、if let )
           即" 从枚举变体中提取值的途径 "是匹配" 分支绑定对象的部分值 "
    */

    // 匹配守卫
    #[derive(Debug, PartialOrd, PartialEq)]
    enum UsState {
        Alabama,
        Alaska,
    }
    // 枚举
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }
    let coin = Coin::Quarter(UsState::Alabama);
    /* // match 表示法
        match coin {
            // 匹配绑定
            Coin::Quarter(state) => println!("State quarter from {:?}!", state),
            _ => (),
        }
    */

    // if let 等同 match 表示法(match表达繁锁时可用if let替换)
    /* if let Coin::Quarter(state) = coin {  // 匹配绑定
         println!("State quarter from {:?}!", state);
     } */

    /* // [自]仅为匹配判断(无绑定)
    if let Coin::Quarter(UsState::Alabama) = coin {
        println!("State quarter from {:?}!",UsState::Alaska);
    } */

    /* // 匹配绑定 后 进行普通判断
    if let Coin::Quarter(state) = coin {    // 匹配绑定
        // 普通判断(与"匹配守卫"有本质差别)
        if  state == UsState::Alabama {
            println!("State quarter from {:?}!", state);
        }
    }
    */

    /* // 匹配守卫是指定于 match 分支(而非 if 语句)模式后的额外 if 条件
    // 错误示例 ：飘红报错
    if let Coin::Quarter(state) = coin if state==UsState::Alabama {
        println!("State quarter from {:?}!",state);
    }
    */

    match coin {
        // 匹配绑定 后 匹配守卫
        Coin::Quarter(state)  if state == UsState::Alabama
        => println!("State quarter from {:?}!", state),
        _ => (),
    }
}

// 匹配绑定 / 匹配不绑定
fn main() {
    let some_u8_value = Some(0u8);
    match some_u8_value {
        // 匹配无绑定
        Some(3) => println!("three"),
        // 匹配绑定 后 匹配守卫
        Some(v) if v > 3 => println!("{}", v),
        // 匹配绑定
        Some(v) => println!("{}", v),
        _ => (),
    }

    // 匹配无绑定
    if let Some(3) = some_u8_value {
        println!("three");
    }
    // 匹配绑定 后 普通判断(本质区别于"匹配守卫")
    if let Some(v) = some_u8_value {
        if v == 0 {
            println!("{}", v);
        }
    }
    // 匹配绑定
    if let Some(v) = some_u8_value {
        println!("{}", v);
    }

}