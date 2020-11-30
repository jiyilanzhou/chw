/*
0. 模式匹配
    a. 模式匹配介绍
        模式匹配是一种结构性的解构。模式匹配的典例即是解构结构体或元组
            // 模式匹配：解构结构体、元组
            struct Point {
                x: i32,
                y: i32,
            }
            let (a, b) = (1, 2);
            let Point { x, y } = Point { x: 3, y: 4 };
            assert_eq!(1, a);
            assert_eq!(2, b);
            assert_eq!(3, x);
            assert_eq!(4, y);
    b. 模式匹配的位置与模式: Rust 中支持模式匹配的位置有 6 个
        a. let 声明
        b. 函数和闭包参数
        c. match 表达式
        d. if let 表达式
        e. while let 表达式
        f. for 表达式

1. (模式匹配位置与模式)函数与闭包参数

 */
fn sum(x: String, ref y: String) -> String {
    x + y
}
fn main() {
    /*  " to_owned "源码：
        impl ToOwned for str {
             fn to_owned(&self) -> String {
                unsafe { String::from_utf8_unchecked(self.as_bytes().to_owned()) }
             }
        }
     */
    let s = sum("1".to_owned(), " 2".to_owned());
    assert_eq!(s, "12".to_owned());
}