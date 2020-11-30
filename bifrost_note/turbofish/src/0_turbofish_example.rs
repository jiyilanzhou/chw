

/*
0. turbofish 操作符
    a. 格式：" path::<...>,method::<...> "
    b. 用途：为表达式中的"泛型、函数或方法"指定参数
             如: "42".parse::<i32>(); 或" Trait::<Type>::func "


 */
// 泛型 Trait A
trait A<T> {
    fn func();
    fn method(&self);
    fn default_impl() { println!("default impl") }
}
// 泛型 Trait B
trait B<T> {
    fn method(&self);
    fn funca();
}
// 结构体
struct S;

// 抽象 : 使用泛型统一实现(为结构体 S 实现 A、B Trait)
impl<T> A<T> for S {    // 实现 A Trait
fn func() { println!("A trait func impl"); }
    fn method(&self) { println!("A trait method impl"); }
}
impl<T> B<T> for S {    // 实现 B Trait
fn method(&self) { println!("B trait method impl"); }
    fn funca() { println!("B trait func impl"); }
}

fn main() {
    /* 编译错误：
       # 实现的多个 Trait 中有相同 func 时
           error[E0034]: multiple applicable items in scope
                 S::func();
                    ^^^^ multiple `func` found
       # 实现的多个 Trait 中没有相同 func 时
           error[E0282]: type annotations needed
           S::func();
              ^^^^ cannot infer type for type parameter `T` declared on the trait `D`
     */
    //S::func();

    /* a. trait 不可直接调用函数(无论是否携带泛型、是否有默认实现[因 trait 非具体类型])
          如"  A::func(); 或 A::<i8>::func()；"(注：没有" A<i8>::func() "的用法)
       b. 但当声明的函数为方法时可用" Trait(::<Type>)::method(&implType) "调用
          注：" Trait::<Type> " 为 turbofish 操作符用法
     */
    // A::<i8>::func();   // 暂且注释以通过编译

    // 使用完全限定语法
    <S as A<i8>>::func();
    let s = S;
    // 当声明的函数为方法时可用" Trait::<Type>::method(&implType) "调用
    <S as B<bool>>::method(&s);
    B::<bool>::method(&s);

    /* 编译错误：
        error[E0034]: multiple applicable items in scope
             s.method();
               ^^^^^^ multiple `method` found
     */
    // s.method();
}