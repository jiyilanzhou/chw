/*
0. 路径相关语法
    a. " ident::ident ":
            命名空间路径
    b. " ::path ":
            从根模块开始的相对路径 (比如一个显式的绝对路径)
    c. " self::path ":
            从当前模块开始的相对路径(比如一个显式的相对路径)
    d. " super::path ":
            从当前模块的父结点开始的相对路径
    e. " type::ident,<type as trait>::ident ":
            关联常数、函数和类型
    f. " <type>::... ":
            不能被直接命名的类型的关联条目( <&T>::...、<[T]>::... 等)
    g. " trait::method(...) ":
            通过命名定义该方法的 trait 来消除方法调用的歧义
    h. " type::method(...) ":
            通过命令定义该方法的类型来消除方法调用的歧义
    i. " <type as trati>::method(...) ":
            通过命名定义该方法的 trait 和类型来消除方法调用的歧义

1. " ident::ident " : 命名空间路径

2.

 */

/*
0. 类型别名(简化代码)
    类型别名亦可用于泛型场景如" type Double<T> = (T,Vec<T>); "
    则后续使用 Double<i32> 时等同于" (i32,Vec<i32>) "

1. trait 别名(trait alias)
    trait 别名用于已确定关联类型的场景下简化代码(类比 type alias [类型别名])

 */
// 泛型 Trait
trait Trait<T>{
    fn func();
}
struct SN;
impl<T> Trait<T> for SN{
    fn func() {
        println!(" Trait::T::func ")
    }
}
fn main_0() {
    // Console:" Trait::T::func "
    <SN as Trait::<i32>>::func();
}
// 泛型别名(Dive[p60])
#![feature(trait_alias)]
trait Service {
    type Item;
    type Output;
    fn call(&self, rhs:Self::Item) -> Self::Output;
}
// 确定关联类型场景下可取别名(简化代码)
trait HttpService = Service<Item=i32,Output=String>;
/* 编译报错：
   error[E0404]: expected trait, found trait alias `HttpService`
        impl HttpService for SN {}
             ^^^^^^^^^^^ not a trait
 */
//impl HttpService for SN {} // 问题：trait alias 适用场景[???]
fn main() {

}