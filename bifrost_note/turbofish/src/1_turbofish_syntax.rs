/*
0. turbofish 语法

1. 多数时候涉及泛型时，编译器可自动推断出泛型参数

 */
fn main_0() {
    // v must be a Vec<T> but we don't know what T is yet
    let mut v = Vec::new();
    // v just got a bool value, so T must be bool!
    v.push(true);
    // Debug-print v
    println!("{:?}", v);
}

/*
2. 编译器不能推断类型时需要显式指定类型

 */
fn main_1() {
    /* 编译错误：
     error[E0282]: type annotations needed for `Vec<T>`
         let v = Vec::new();
             -   ^^^^^^^^ cannot infer type for type parameter `T`
             |
             consider giving `v` the explicit type `Vec<T>`, where the type parameter `T` is specified
     */
    // let v = Vec::new();

    // 解决方案 1 ： 可使用类型注解
    // let v : Vec<bool> = Vec::new();

    /* 解决方案 2 ： 使用 turbofish 语法即" ::<> "来绑定泛型参数 T
         turbofish 方法在不想将结果绑定到变量时很有用

     */
    let v = Vec::<bool>::new();
    println!("{:?}", v);

    // turbofish 亦可用来在函数和方法中绑定泛型参数
    /* 编译错误：
        error[E0282]: type annotations needed
             let a = (0..255).sum(); //error, cannot infer type
                 ^ consider giving `a` a type
     */
    // let a = (0..255).sum();
    let b = (0..255).sum::<u32>(); // 用 turbofish 语法指定类型参数
    let c:u32 = (0..255).sum(); // 指定变量类型
}

/*
3. turbofish 使用场景
   a. 可为泛型函数，方法，结构或枚举指定具体类型。
   b. 在类型定义中使用 IDENT<T> 而在表达式上下文中使用 IDENT::<T> 来指定泛型参数的类型

 */
/* 泛型函数：
   a. 标准库里面的 std::mem::size_of()函数签名" pub const fn size_of<T>() -> usize "
   b. str 的 parse() 方法签名" pub fn parse<F: FromStr>(&self) -> Result<F, F::Err> "
   c. Iterator 上 collect() 签名"fn collect<B: FromIterator<Self::Item>>(self) -> B where Self: Sized"
 */
fn main_2() {
    // std::mem::size_of()函数
    std::mem::size_of::<u8>(); // u8的字节数
    std::mem::size_of::<u32>(); // u32的字节数
    std::mem::size_of::<i32>(); // i32的字节数

    // str 的 parse() 方法签名
    "2048".parse::<u32>();

    // Iterator 上 collect() 签名：编译器可推断收集的 Self::Item 类型故通常使用"_"让其自动推断
    let a = [1, 2, 3, 4].iter().collect::<Vec<_>>();
}

/* 泛型结构：
    编译器在创建通用结构时无法推断出足够信息，可用 turbofish 语法来指定
    如 Vec 被定义为" pub struct Vec <T> {/ *字段省略* /} "
    使用 Vec::new() 创建新 Vec 时可将其写为" Vec::<u8>::new(); "
    // 注：将 turbofish 置于 Vec 而非 new() 方法后面(因为是泛型结构而非泛型方法）
 */

/* 泛型枚举
    枚举 turbofish 语法不同于经验法则。RUST 枚举 turbofish 限定未置于枚举类型后而是置于枚举变量之后
    如: pub enum Result<T, E> {
            Ok(T),
            Err(E),
        }
        pub enum Option<T> {
            None,
            Some(T),
        }

 */
fn main_3() {
    // 由于turbofish 限定置于枚举变量(而非放在枚举类型)后面
    Result::Ok::<u8, ()>(10);
    Result::Err::<u8, ()>(());
    Option::None::<u8>.unwrap();
    // 可简写为
    Ok::<u8,()>(10);
    Err::<u8,()>(());
    None::<u8>.unwrap();
}

/* 泛型 Trait

 */
trait A<T>{
    fn method(&self);
}
struct S;
impl<T> A<T> for S{
    fn method(&self) {
        println!("S impl Trait A");
    }
}

fn main() {
    // 泛型 Trait : turbofish
    let s = S;
    A::<i32>::method(&s);
    // 泛型 Trait : 完全限定语法
    <S as A<i32>>::method(&s);
    // 完全限定语法简写
    <A<i32>>::method(&s);
}