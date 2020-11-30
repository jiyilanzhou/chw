
/*
0. 路径 ( Rust 中单个文件即是模块)
   a. 路径的第一个用法：使用 supser 或 self 调用
   b. 路径的第二个用法：使用 as 完全限定语法无歧义调用
   c. 路径的第三个用法：泛型函数使用 turbofish 操作符(泛型函数专用)

 */

fn main_0() {
    // a > b > c
    pub mod a {
        fn foo() { println!("mod a "); }
        pub mod b {
            pub mod c {
                // 路径用法 1
                pub fn foo(){
                    // 在 mod c 内故第一个 super 为 mod b 第二个 super 为 mod a
                    //super::super::foo();     // call a's foo function
                    // self 即为 mod c
                    self::super::super::foo();   // call a's foo function
                }
            }
        }
    }
    a::b::c::foo();
}

/*
1. 函数调用
   使用 as 完全限定语法无歧义调用

 */
fn main() {
    struct S;
    impl S {
        fn f() { println!("S"); }
    }
    trait T1 {
        fn f() { println!("T1 f"); }
    }
    impl T1 for S {}
    trait T2 {
        fn f() { println!("T2 f"); }
    }
    impl T2 for S {}
    // inherent[ɪnˈherənt; ɪnˈhɪərənt]n.固有的,内在的
    S::f();  // Calls the inherent impl.
    // 路径用法 2 ：完全限定无歧义调用
    <S as T1>::f();  // Calls the T1 trait function.
    <S as T2>::f();  // Calls the T2 trait function.

    // 路径用法 3 : 调用泛型函数时指定具体类型即 turbofish 操作符(因其调用外观类似 fish )
    //(0..10).collect::<Vec<i32>>();
    (0..10).collect::<Vec<_>>();    // 使用通配符" _ "
    Vec::<u8>::with_capacity(1024);
}