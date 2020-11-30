
// implicit [ɪmˈplɪsɪt]adj.含蓄的,隐式  // explicit[ɪkˈsplɪsɪt]adj.明确的,显式
// coercion[kəʊˈɜːʃn]n.强制       // blanket [ˈblæŋkɪt]v&n.总体的,覆盖
// ambiguity[ˌæmbɪˈɡjuːəti]n.歧义,含糊  // disambiguate[ˌdɪsæmˈbɪɡjueɪt]v.消除(歧义)
/*
3.5 类型转换(" P83 ~ p ")
    隐式类型转换(Implicit Type Conversion) : 强制类型转换(Type Coercion)
    显式类型转换(Explicit Type Conversion) : 一般意义上的类型转换(Type Cast)

3.5.1 解引用
    a. 自动解引用(简化编程)
       若一个类型 T 实现了 Deref<Target=U> 则该类型 T 的引用(或智能指针)在应用的时候
       会被自动转换为类型 U
    b. 手动解引用

*/
fn main_02_00() {
    let x = "hello".to_string();
    // 自定义解引用:将 &String 类型转为 &str 类型
    match &*x {
        "hello" => println!("hello"),
        _=>()
    }
}

/*
3.5.2 as 操作符( " P86 ")      // disambiguation ['dɪsæm,bɪɡjʊ'eɪʃən]n.消歧(消除歧义)
    a. as 操作符最常用于转换 Rust 中基本数据类型(原生类型)的场景
    b. 无歧义完全限定语法(Fully Qualified Synax for Disambiguation),曾经亦称为" 通用函数
       调用语法(UFCS) "
    Interview: [自]"无歧义完全限定语法" 与 "标记类型" 的区别与联系
    c. 类型和子类型相互转换
       Rust 没有标准定义中的(如"结构体继承")子类型但生命周期标记可看作子类型。如" &'static str "
       类型是" &'a str "的子类型，因二者生命周期标记不同，'a 和 'static 都是生命周期标记，其中
       'a 是泛型标记是 &str 的通用形式而 'static 则是特指静态生命周期的 &str 字符串。故通过 as
       操作符可将 &'static str 类型转为 &'a str

3.5.3 From 和 Into
    a. From 内部实现
        pub trait From<T>: Sized {
            /// Performs the conversion.
            #[stable(feature = "rust1", since = "1.0.0")]
            fn from(_: T) -> Self;
        }
    b. Into 内部实现
        pub trait Into<T>: Sized {
            /// Performs the conversion.
            #[stable(feature = "rust1", since = "1.0.0")]
            fn into(self) -> T;
        }
    c. (Rust 标准库默认实现)为所有实现了 From<T> 的类型 T 实现 Into<U>
        impl<T, U> Into<U> for T where U:From<T>        // 待理解[???]

3.6 当前 trait 系统的不足
     a. 孤儿规则的局限性：
            #[fundamental]      // "#[fundamental]"属性表示脱离孤儿规则限制
            #[stable(feature = "rust1", since = "1.0.0")]
            pub struct Box<T: ?Sized>(Unique<T>);
     b. 代码复用的效率不高        // blanket [ˈblæŋkɪt]v&n.总体的,覆盖
        除孤儿规则 Rust 还遵循"重叠(Overlap)规则":不能为重叠的类型实现同一个 trait。
        覆盖式实现(Blanket Impl)、impl 特化(Specialization[仅在 Nightly 版本使用])
     c. 抽象表达能力有待改进
        泛型关联类型( Generic Associated Tyep, GAT )。
        GAT 亦被称作关联类型构造器( Associated type constructor, ACT )
     // 注： Rust 中隐式类型转换是安全的(区别于其它语言)
            完善类型系统心智模型( Mental Model )  // mental[ˈmentl]adj&n.精神的,心智

*/