
// 函数中的生命周期(避免"悬垂引用"[即避免引用已释放的内存])
/*  // 生命周期未能自动计算出则需标注
    fn longest_0(x: &str, y: &str) -> &str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
    // 编译错误：错误的生命周期(避免"悬垂引用")
        error[E0106]: missing lifetime specifier
          fn longest_0(x: &str, y: &str) -> &str {
                                            ^ expected lifetime parameter
        = help: this function's return type contains a borrowed value, but
          the signature does not say whether it is borrowed from `x` or `y`

*/

// 生命周期欲先声明后使用(类比"泛型"):格式" 'identifier "
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// [自]根据输出避免"悬垂引用"限定相应输入的生命周期
fn get_str<'a>(x: &'a str, y: &str) -> &'a str {
    x
}

/* // 错误的生命周期("悬垂引用"):
    fn a_str<'a>(x: &'a str, y: &'a str) -> &'a str {
        let r = String::from("abc");
        r.as_str()      // 返回 r 的引用
    }      // 离开作用域调用" drop "释放内存导致返回的 r 为悬垂引用
    // 编译错误：不能返回本地变量值的引用(即"悬垂引用")
    error[E0515]: cannot return value referencing local variable `r`
         r.as_str()
         -^^^^^^^^^
         |
         returns a value referencing data owned by the current function
         `r` is borrowed here

*/

fn main() {
    let s1 = String::from("abcde");
    let s2 = String::from("ab");
    let r = longest(s1.as_str(), s2.as_str());
    println!("r = {}", r);

    let ss = get_str(s1.as_str(), s2.as_str());

}