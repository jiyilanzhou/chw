
fn main() {
    let x = 4;
    // 闭包捕获环境变量
    let equal_to_x = |z| z==x;
    let y = 4;
    assert!(equal_to_x(y)); // 断言" y "与" x "相等

    let x = vec![1, 2, 3];
    // 闭包调用结束 drop 释放 x
    let equal_to_x = move |z| {z==x};
    /* 编译报错：使用" move "将所有权移至" 闭包 "后再次尝试使用
       error[E0382]: borrow of moved value: `x`
          let x = vec![1, 2, 3];
              - move occurs because `x` has type `std::vec::Vec<i32>`, which
                does not implement the `Copy` trait
          let equal_to_x = move |z| {z==x};
                           --------     - variable moved due to use in closure
                           |
                           value moved into closure here
          println!("x === {:?}", x);
                                 ^ value borrowed here after move

    */
    // println!("x === {:?}", x);
    let y = vec![1, 2, 3];
    assert!(equal_to_x(y));

}

// 闭包是可捕获环境变量的匿名函数
/* 闭包可通过三种方式捕获相应环境，其对应函数三种获取参数的方式，分别为" 获取所有权、
   可变借用、不可变借用"。这三种捕获值的方式被编码为如下三个 Fn trait：
    （1）FnOnce 消费从周围作用域捕获的变量，闭包周围的作用域被称为其环境。为了消费
        捕获到的变量,闭包必须获取其所有权并在定义闭包时将其移进闭包。其名称的 Once
        部分代表了闭包不能多次获取相同变量的所有权。
    （2）FnMut 获取可变的借用值，所以可以改变其环境。
    （3）Fn 从其环境获取不可变的借用值。
     注：当创建一个闭包时，rust 会根据其如何使用环境中的变量来推断希望如何引用环境。
         由于所有闭包都可以被调用至少一次，因此所有闭包都实现了 FnOnce。没有移动被
         捕获变量的所有权到闭包的闭包也实现了 FnMut，而不需要对捕获的变量进行可变
         访问的闭包实现了 Fn。

*/