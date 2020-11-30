
use std::ops::Deref;
// 包装类(元组结构体)及其实现
struct MyBox<T>(T);
impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
/* 实现 Deref trait :
   源码： pub trait Deref {
                /// The resulting type after dereferencing.
                #[stable(feature = "rust1", since = "1.0.0")]
                type Target: ?Sized;
                /// Dereferences the value.
                #[must_use]
                #[stable(feature = "rust1", since = "1.0.0")]
                fn deref(&self) -> &Self::Target;
           }
*/
impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

fn main() {
    let x = 5;
    let y = MyBox::new(x);
    assert_eq!(5, x);
    /* 未实现" Deref trait "则编译报错：
        error[E0614]: type `MyBox<{integer}>` cannot be dereferenced
            assert_eq!(5, *y);
                          ^^
        error[E0308]: mismatched types
             hello(&m);
                   ^^ expected str, found struct `MyBox`
    */
    assert_eq!(5, *y);

    // 使用泛型(区别于"闭包"[闭包仅首次调用推断并绑定参数及返回值类型])
    let m = MyBox::new(String::from("Rust"));
    /* 解引用强制多态特性:
           Rust 通过 Deref 会将 MyBox 变为 &String 再将标准库 String 的
           解引用变为字符串 slice
    */
    hello(&m);
}

fn hello(name: &str) {
    println!("Hello, {}", name);
}

/* 解引用多态与可变性交互：// (待理解[?])
（1）当 T：Deref<Target=U> 时，从 &T 到 &U
     如"  let m = MyBox::new(String::from("Rust")); "则 T 即为 String
     而 Target 即为 &str
 (2) 当 T：DerefMut<Target=U> 时，从 &mut T 到 &mut U
（3）当 T：Deref<Target=U> 时，从 &mut T 到 &U (从可变到不可变[反之不行])

*/