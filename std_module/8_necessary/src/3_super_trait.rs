
/*
0. 完全限定语法(消除歧义)
    格式：" <Type as Trait>::function(receiver_if_method,next_arg ... ); "
    对于关联函数而言上述形式会缺少" receiver "而只保留剩下的参数列表，可在任何
    调用函数(包含方法)的地方使用完全限定语法，而 Rust 允许忽略能够从上下文推导
    的部分。只有存在多个同名实现以致无法区分调用的具体实现时才需要这种较为烦琐
    的显式语法

1.  超 Trait (supertrait)
    a. 继承父 trait：" trait subTrait : superTrait { // ... } "
       当前 trait 需要使用到另一个 trait 的功能(即当前 trait 依赖于另一个同时
       被实现的 trait [父 trait]：欲使用则须同时实现" subTrait 及 superTrait ")
    b. (区别于泛型自动实现)" impl<T:Trait> otherTrait for T {} "用于为实现 Trait
       的具体类型自动实现" otherTrait "(须在" impl<T:Trait> otherTrait for T {} "
       的实现体" {}＂内实现所有定义于 otherTrait 内没有默认实现的函数[包含方法])

*/
// a. Trait 继承
trait SuperTrait {
    fn func();
}
trait SubTrait: SuperTrait {}
struct SN0;
/* 若未先实现 SuperTrait 则飘红报错：
       the trait bound `SN0: SuperTrait` is not satisfied [E0277] the trait
       `SuperTrait` is not implemented for `SN0`
 */
// impl SubTrait for SN0{}
impl SuperTrait for SN0{
    fn func() {
        println!("impl SuperTrait");
    }
}
// 实现 SuperTrait 为实现 SubTrait 的必要条件(非充要条件)
//impl SubTrait for SN0{}
fn main_0() {
    SN0::func()
}

// b. (泛型)自动实现 OtherTrait
trait OtherTrait {
    fn func();
}
trait Trait {}
struct SN1;
// 为实现 Trait 的具体类型实现 OtherTrait
impl<T: Trait> OtherTrait for T {
    fn func() { println!(" T impl OtherTrait ") }
}
// 具体实现类型不能重写不属于 Trait 的函数(包含方法)
impl Trait for SN1 {
    // 飘红报错" Method `func` is not a member of trait `Trait` [E0407] "
    //fn func() { println!(" T impl OtherTrait ") }
}
fn main() {
    // 无歧义则系统可从上下文推导
    SN1::func();
    // 完全限定语法
    <SN1 as OtherTrait>::func();
    /* 编译错误：
       error[E0576]: cannot find method or associated constant `func` in trait `Trait`
          <SN as Trait>::func();
                         ^^^^ not found in `Trait`
     */
    // <SN as Trait>::func();   // 暂且注释以通过编译
}