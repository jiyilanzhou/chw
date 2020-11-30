
//1、结构体中的生命周期
/* // 结构体
    struct A {
        name: &str, // 飘红报错:"Missing lifetime specifier [E0106]"
    }
// 编译报错:
    error[E0106]: missing lifetime specifier
       name: &str,
             ^ expected lifetime parameter
*/

#[derive(Debug)]
struct A<'a> {
    name: &'a str,
}

fn main() {
    // 结构体内的生命周期
    let n = String::from("hello");
    let a = A { name: &n };
    println!("a = {:#?}", a);

    // 生命周期省略
    let s = get_a_str(&n);
    println!("s = {}", s);
}

//2、生命周期省略
/*
（1）没有生命周期注解却能够编译，原因：早期的 rust 中必须显式的声明生命周期，后来 rust 团队
     将很明确的模式进行了注解的简化。
（2）遵守生命周期省略规则的情况下能明确变量的声明周期，则无需明确指定生命周期。函数或者方法
     参数的生命周期称为输入生命周期，而返回值的生命周期称为输出生命周期。
（3）编译器采用三条规则判断引用何时不需要生命周期注解，当编译器检查完这三条规则后仍不能计算
     出引用的生命周期，则会停止并生成错误。
（4）生命周期注解省略规则适用于 fn 定义以及 impl 块定义，如下：
     a、每个引用的参数都有它自己的生命周期参数。如下：
         单引用参数的函数有一个生命周期： fn foo<'a>(x: &'a i32)
         双引用参数函数有两个生命周期：fn foo<'a, 'b>(x: &'a i32, y: &'b i32)
         (以此类推)多引用参数函数有多个生命周期
      b、如果只有一个输入生命周期参数，那么它被赋予所有输出生命周期参数：
         " fn foo(x: &i32) -> &i32 "等价于" fn foo<'a>(x: &'a i32) -> &'a i32 "
      c、方法有多个输入生命周期参数，不过其中之一因为方法的缘故为 &self 或者 &mut self，
         那么 self 的生命周期被赋予所有输出生命周期参数如:
         fn function(&self, x: &str, y: &str, ....) -> &str

*/
fn get_a_str(s: &str) -> &str {
    s
}
