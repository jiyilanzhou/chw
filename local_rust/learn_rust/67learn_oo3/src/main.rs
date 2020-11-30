
/*
1、trait 对象动态分发
    a. 泛型类型 trait bound ：单态化处理(静态分发[即编译器在编译时已知晓具体类型])
    b. 使用 trait 对象必须为动态分发：编译器编译时无法知晓所有可能实现 trait 的具体
       类型故于运行时使用 trait 对象中的指针来知晓实现 trait 的具体类型

2、trait 对象 : 要求对象安全
    只有对象安全(object safe)的 trait 才能组成 trait 对象。trait 的方法满足以下两条
    要求才是对象安全的：
        a. 返回值类型不为 Self
        b. 方法没有任何泛型类型参数

*/

/* 编译错误：非对应安全(不可作为 trait 对象)
    error[E0038]: the trait `Clone` cannot be made into an object
        pub com: Vec<Box<dyn Clone>>,
        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `Clone` cannot be made into an object
     = note:method `clone` references the `Self` type in its arguments or return type
*/
// Clone
pub trait Clone {
    fn clone(&self) -> Self;
}

pub struct Screen {
    pub com: Vec<Box<dyn Clone>>,
}

fn main() {
    println!("Hello, world!");
}

/* 作业 ：
    将《Rust程序设计语言》中 17.3 节中的例子敲一遍
    实现一个发布博客的工作流如下：
        博文从空白的草案开始。
        一旦草案完成，请求审核博文。
        一旦博文过审，它将被发表。
    // 只有被发表的博文的内容会被打印，这样就不会意外打印出未被审核的博文文本。

*/