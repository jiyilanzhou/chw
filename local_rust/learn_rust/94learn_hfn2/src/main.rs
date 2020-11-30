
/*
2、函数返回闭包

*/

/* 编译报错: // 编译时未知其类型大小
    error[E0277]: the size for values of type `(dyn std::ops::Fn(i32) -> i32 + 'static)`
                                                   cannot be known at compilation time
        fn return_closure() -> Fn(i32) -> i32 {
                               ^^^^^^^^^^^ doesn't have a size known at compile-time
         = help: the trait `std::marker::Sized` is not implemented for
                                            `(dyn std::ops::Fn(i32) -> i32 + 'static)`
        = note: the return type of a function must have a statically known size

    error[E0308]: mismatched types      // 错误类型
        fn return_closure() -> Fn(i32) -> i32 {
                             ----- expected `(dyn std::ops::Fn(i32) -> i32 + 'static)`
                                                              because of return type
            |x| x+1
            ^^^^^^^ expected trait std::ops::Fn, found closure
        = note: expected type `(dyn std::ops::Fn(i32) -> i32 + 'static)`
                  found type `[closure@src\main.rs:18:5: 18:12]`
  // 解决方案：可用" Box<dyn Fn(prm)> "解决
*/
/* // 暂且注释：以通过编译
    fn return_closure() -> Fn(i32) -> i32 {
        |x| x+1
    }
*/

// 使用" Box<dyn Fn(prm)> "处理返回闭包
fn return_clo() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}

fn main() {
    // 获取闭包
    let c = return_clo();
    println!("1 + 1 = {}", c(1));
    // 函数亦是指针故亦可使用"解引用多态"
    println!("1 + 1 = {}", (*c)(1));

}