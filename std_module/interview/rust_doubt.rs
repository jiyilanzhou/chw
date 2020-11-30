
// exclude[ɪkˈskluːd]v.排除,排斥        // exclusive[ɪkˈskluːsɪv]n.独有的,排外的,排斥
/*
0. Rust 中除了宏以外是否还有其它方式传递不定参

1.  Trait 约束
    fn my_print<T:Debug>(x:T){}
    // 不能直接写为:
    fn my_print(x:Debug){}  // 飘红报错
    // 源于： trait 不能用作局部变量的类型；不能直接用作参数及返回值的类型
    // 改进
    fn my_print(x:impl Debug){}

2. " #![feature(exclusive_range_pattern)] "解决" .. "编译报错

 */
#![feature(exclusive_range_pattern)]    // 解决" .. "编译报错
fn main_0() {
    let x = 3;
    match x {
        0 => 6,             // 值匹配
        1 | 2 | 3 => 8,     // 多条件匹配
        // 未添加" #![feature(exclusive_range_pattern)] "会编译报错
        4..9 => { 3 }       // 范围匹配(酌情可用" 4..=8 "替代)
        _ => { 0 }
    };
    println!("{:?}",x);
}

/*
3. macro! 替代 macro_rules!

4. impl Sharp for Round{} 和 impl<T:Round> Sharp for T{} 区别

4. 类型转换
    as
    TryFrom

 */

fn main() {

}