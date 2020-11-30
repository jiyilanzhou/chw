/*
0. 根据泛型的 trait 约束有条件地实现方法(Base[p255])
   a. "标准库对所有满足 Display trait 约束的类型实现了 ToString trait "
       impl <T:Display> ToString for T { //... }
   b. 自动实现主要用于自动为实现了 Trait 的类型实现 OtherTrait, 不可用于
      自身实现如" impl <T:selfTrait> selfTrait for T { //... } "。有
      弊端：从语义上而言"为实现 selfTrait 的类型实现 selfTrait "，本就
      没有任何实在意义(仅符合语法定义)并且会与具体实现 selfTrait 的类型
      冲突。如" impl<T: A> A for T {} "与" impl A for S {} "冲突(因为
      重复实现[非重写实现]，故开发不可存在" 虽然没有违背语法但是没有任何
      意义的" impl <T:selfTrait> selfTrait for T {} 代码)
   d. 自动实现与" subTrait : supperTrait "(即 Trait 继承)的区别：前者为
      实现了 trait 的具体类型自动实现另一 trait ; 后者则为实现 subTrait
      的具体类型亦必须同时实现 superTrait (即 SuperTrait 是 SubTrait 的
      依赖条件)

 */
trait A {
    fn func();
}
struct S;
/* 编译报错：
    error[E0119]: conflicting implementations of trait `A` for type `S`:
         impl<T: A> A for T {}
       | ------------------ first implementation here
       | impl A for S {}
       | ^^^^^^^^^^^^ conflicting implementation for `S`
 */
// impl<T: A> A for T { fn func() {} }    // 暂且注释以通过编译
impl A for S {
    fn func() {
        println!("Hello Rust!");
    }
}

fn main_0() {
    <S as A>::func();
}

/*
1. 高级 trait (Base[p573~575])
    a. Iterator 关联类型
       源码:  pub trait Iterator {
                  tpe Item;
                  fn next(&mut self) -> Option<Self::Item>;
                  // ...
              }
        // " Item "为 trait 关联类型即 next 返回的是包含 Item 的 Option 类型
    b. 使用关联类型与泛型的区别
       (0). 使用关联类型可省略类型标注且针对具体类型仅单次实现(因 Item 固定
            [无论其下函数是否与 Item 有关])，故关联类型之一用途即是针对具体
            类型限定为单一实现。再者因没有类型标注故" impl T for S {} "格式
            唯一(所以也不允许有多个重复实现)
       (1). 使用泛型针对同一类型可以多次实现(因可进行类型标注[格式不一])

2. turbofish 操作符
    a. 格式：" path::<...>,method::<...> "
    b. 用途：为表达式中的"泛型、函数或方法"指定参数
             如: "42".parse::<i32>(); 或" Trait::<Type>::func "

 */
// 关联类型
trait B {
    type Item;
    //type Output;
}
struct C;
impl B for C {
    type Item = ();
}
/* 编译报错：仅从实现形式上即可知" impl B for C {} "实现重复
    error[E0119]: conflicting implementations of trait `B` for type `C`:
       | impl B for C{
       | ------------ first implementation here
       | impl B for C{
       | ^^^^^^^^^^^^ conflicting implementation for `C`
*/
// impl B for C{type Item = i32;}

// 泛型
trait D<T> {
    fn func();
    fn method(&self);
    fn default_impl() { println!("default impl") }
}
struct E;
/*
    // 具体类型多次实现(亦可抽象为使用泛型统一实现)
    impl D<i8> for E {
        fn func() { println!("func"); }
        fn method(&self) { println!("method"); }
    }
    impl D<String> for E {
        fn func() { println!("func"); }
        fn method(&self) { println!("method"); }
    }
*/
// 抽象 : 使用泛型统一实现
impl<T> D<T> for E {
    fn func() { println!("func"); }
    fn method(&self) { println!("method"); }
}

trait F<T> {
    fn method(&self);
    fn func();
}
impl<T> F<T> for E{
    fn method(&self) { println!("method"); }
    fn func() { println!("func"); }
}

fn main() {
    /* 编译错误：
       E::func();
          ^^^^ cannot infer type for type parameter `T` declared on the trait `D`
     */
    // E::func();

    /* a. trait 不可直接调用函数(无论是否携带泛型、是否有默认实现[因 trait 非具体类型])
          如"  D::func(); 或  D::<i8>::func()；"
       b. 但当声明的函数为方法时可用" Trait(::<Type>)::method(&implType) "调用
          注：" Trait::<Type> " 为 turbofish 操作符用法
     */
    // D::<i8>::func();   // 暂且注释以通过编译

    let e = E;
    // 使用完全限定语法
    <E as D<i8>>::func();
    <E as D<i8>>::method(&e);

    // 当声明的函数为方法时可用" Trait::<Type>::method(&implType) "调用
    F::<i32>::method(&e);
    /* 同理：不可直接使用泛型调用函数(因 trait 非具体类型):
       error[E0283]: type annotations needed
          F::<i8>::func();
          ^^^^^^^^^^^^^^^^ cannot infer type
     */
    // F::<i8>::func();
}