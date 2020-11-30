
/*
5、newtype 模式(newtype pattern)：用以在外部类型上实现外部 trait
   a. 孤儿规则(orphan rule):
      本地 trait 或类型(对于当前 crate )：可在此类型上实现该 trait
   b. newtype 模式可绕开孤儿规则
      (可理解为 Rust 的某种用法技巧)

*/

/*
1. 孤儿规则：
    " Vec "为外部类型而" fmt::Display "为外部特征，若执意直接为外部
    类型" Vec "实现外部特征" fmt::Display "肯定报错(因违反孤儿规则)

2. newtype 模式(newtype pattern)
   若欲使用"Vec<T>"类型且为其实现" fmt::Display "特征则可用 newtype
   模式。例如在本地使用" Wrapper "将其包裹再为其实现" fmt::Display "
   (即绕开" 孤儿规则 ")

*/

/* // 编译报错： // 只有在当前板条箱中定义的特征才能实现为任意类型
    error[E0117]: only traits defined in the current crate can be
                                           implemented for arbitrary types
       impl fmt::Display for Vec<String>{
       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ impl doesn't use types inside crate
       = note:the impl does not reference only types defined in this crate
       = note:define and implement a trait or new type instead
    // 原因分析：违反孤儿规则
*/
/*  暂且注释：以通过编译
    // 为外部类型实现外部 trait (违反孤儿规则)
    impl fmt::Display for Vec<String>{
        //...
    }
*/

use std::fmt;
// newtype 模式(绕开孤儿规则)
struct Wrapper(Vec<String>);    // " Vec<T> "为外部类型
impl fmt::Display for Wrapper {     // " fmt::Display "为外部特征
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({})", self.0.join(","))
    }
}

/* 说明：
   孤儿规则阻止直接在 Vec<T> 上实现 Display(因 Display trait 和 Vec<T>
   皆定义于当前 crate 之外)，可使用"newtype 模式"创建包含 Vec<T> 实例的
   Wrapper 结构体后再实现(绕开孤儿规则)

*/
fn main() {
    // 声明
    let w = Wrapper(vec![String::from("hello"), String::from("World")]);
    println!("w = {}", w);
}