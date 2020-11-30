
/* 实现 Deref trait 允许重载解引用运算符如:
    let a: A = A::new();
    let b = &a;
    // 解引用(前提：A 类型必须实现 Deref trait)
    let c = *b;

*/

fn main() {
    let x = 5;
    let y = &x;
    assert_eq!(5, x);
    assert_eq!(5, *y); // 解引用

    let z = Box::new(x);
    assert_eq!(5, *z);  // 解引用
}
