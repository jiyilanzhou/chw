trait A<T> {
    fn method(&self){println!("Trait A");}
}
struct S;
impl<T> A<T> for S {
    fn method(&self){println!(" S override Trait A");}
}
impl S {
    fn self_impl_method(&self){println!(" S ");}
}
fn main() {
    let s = S;
    // 完全限定语法
    <S as A<i32>>::method(&s);
    // 完全限定语法简写
    <A<i32>>::method(&s);
    // turbofish 用法
    A::<i32>::method(&s);

    /* 编译错误：
      error[E0282]: type annotations needed
         s.method();
           ^^^^^^ cannot infer type for type parameter `T` declared on the trait `A`
       // 无论是否有默认实现或者重写，调用泛型 Trait 必须进行类型注解(完全限定语法或 turbofish 语法)
     */
    // s.method();

    // 直接调用仅用于自身实现的特有方法
    s.self_impl_method();
}