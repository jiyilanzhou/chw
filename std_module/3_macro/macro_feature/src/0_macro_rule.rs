/*
0. Rust 宏 ：
    a. 声明宏模式
       (0). 模式
            macro_rules! macro_name{
                 (pattern0) => { code block };
                 (pattern1) => { code block };
                 (pattern2) => { code block };
                 ...
            }
            // 类比不同参数列表的重载函数 (形似 match )
       (1). 调用：可匹配" {} / [] / () "如下：
            macro_name!{args} / macro_name![args]; / macro_name!(args);
       (2). " macro_name!{} / [] / () "使用区别：
             单独调用宏仅用于展开代码时时使用" {} "其后一般无需添加";"(因
             " {} "即表示代码块)而使用" [] "或" () "其后必须使用" ; "表示
             代码块结束。若调用宏展开代码同时绑定变量等则" {} / [] / () "
             其后都必须使用" ; "表示绑定。
             比如" macro_name!{} "或" macro_name![]; / macro_name!(); "
             及" let v = vec!{args}; / vec!(args); / vec![args]; "
    b. 过程宏
       派生宏、属性宏、函数宏

1. 声明宏
   核心本质：代码替换

2. 声明宏可捕获类型列表(tao[p448])
    expr : 表达式(会生成具体值)
    item : 语言项(即组成 Rust 包的基本单位)
           如" 模块、声明、定义(函数定义、类型定义、结构体定义)、impl 实现等"
    block : 代码块(由花括号限定的代码)
    stmt : 语句(一般指以分号结尾的代码)
    pat : 模式
    ty : 类型
    ident : 标识符
    path : 路径(如" foo、std::iter "等)
    meta : 元信息(包含在" #[...] "或" #![...] "属性内的信息)
           " #[meta] "外部属性；" #![meta] "内部属性
    tt : TokenTree(词条树)
    vis : 可见性(如" pub ")
    lifetime : 生命周期参数
    // 编写声明宏捕获类型时注意匹配的范围(如词条树 tt 比 expr 能匹配的范围要广)

3. 声明宏的实现技巧(tao[p449])

*/
// 示例 1 ：
macro_rules! declare_macro {    // 简单宏
    ()=>{ println!("tuple pattern!"); };
    // 声明宏内部使用匹配的变量需使用" $ "
    ($name:expr) => { println!("Single variable : {}",$name); };
    ($xname:expr,$yname:expr) => {
        println!("Multiple variable : {} and  {}",$xname,$yname);
     };
}
// 内部重复的宏
macro_rules! vecs {
    // 匹配次数: " + 匹配 1 次或多次 "
    //          " * 匹配 0 次或多次(即任意次数) "
    ( $($elem:expr),+)=>{
        {   // 此处"花括号"不可或缺：对应外层" $(),+ "
            let mut sv : Vec<String> = Vec::new();
            $(  // 对应内层 " $() "
                sv.push(format!("{}",$elem));
            )+
            sv
        }
    };
    /* // " macro "内部有多个模式分支若有同一代码块符合多个分支定义，
          则从上至下检索分支，仅执行首个匹配的模式分支(类比 match )。
          如" let vec = vecs![true,'c']; "虽符合多个分支模式但仅
          展开首个模式匹配的代码块
     */
     ($xname:expr,$yname:expr) => {
        println!("Multiple variable : {} and  {}",$xname,$yname);
     };
}
fn main_0() {
    // 简单宏 // 其" {} "内亦是返回空 tuple 即" () "
    declare_macro! {}
    declare_macro!["chw"];
    declare_macro!("Alice","Bod");
    // 内部重复的宏：
    let str_vec = vecs!["0",1,true,'c'];
    println!("{:?}", str_vec)   // Console: ["0", "1", "true", "c"]
}

// 示例 2 ：使用宏展开生成函数
use std::ops::{Add,Mul,Sub};
/*
// 方式 1 ：定义函数
// 定义向量加法计算
fn func_add_org(xs: &mut Vec<i32>, ys: &Vec<i32>) {
    for (x, y) in xs.iter_mut().zip(ys.iter()) {
        // *x += *y;
        *x = Add::add(*x,*y);
    }
}
// 定义向量减法计算
fn func_sub_org(xs: &mut Vec<i32>, ys: &Vec<i32>) {
    for (x, y) in xs.iter_mut().zip(ys.iter()) {
        // *x -= *y;
        *x = Sub::sub(*x,*y);
    }
}
// 定义向量乘法计算
fn func_mul_org(xs: &mut Vec<i32>, ys: &Vec<i32>) {
    for (x, y) in xs.iter_mut().zip(ys.iter()) {
        // *x *= *y;
        *x = Mul::mul(*x,*y);
    }
}
 */

// 方式 2 ：编译前使用 macro 展开定义函数
macro_rules! ops_m {    // 宏根据匹配模式进行匹配成功后展开花括号内代码
    // 编辑技巧：先搭好架子" ()=>{}; "再编辑细节
    ($func:ident, $bound:ident, $method:ident)=>{
        /* 参照最终展开的样式比如：
            fn func_add_org(xs: &mut Vec<i32>, ys: &Vec<i32>) {
                for (x, y) in xs.iter_mut().zip(ys.iter()) {
                    // *x += *y;
                    *x = Add::add(*x,*y);
                }
            }
         */
        fn $func<T:$bound<T, Output=T> + Copy>(xs:&mut Vec<T>, ys:&Vec<T>){
            // [自]以" min(xs.len(),ys.len()) "为基础组装
            for (x, y) in xs.iter_mut().zip(ys.iter()) {
                  // $bound 为" Add、Sub、Mul "等 : 有固定形式
                  // (故需对 $bound 进行约束)
                  *x = $bound::$method(*x,*y);
            }
        }
    }
}
// 通过宏编译前展开函数(可用" cargo expand "查看展开效果[降低代码冗余])
ops_m!(func_add_new,Add,add);       // 其本质即是代码替换

fn main() {
    /* 动态数组(集合)推荐使用" vec![] "形式
          let mut x1 = vec!{1, 2, 3};
          let mut x1 = vec!(1, 2, 3);
     */
    let mut x1 = vec![1, 2, 3];
    let y1 = vec![1];
    // 方式 1 : 调用自定义函数
    // func_add_org(&mut x1, &y1);

    // 方式 2 ：调用宏生成的函数
    func_add_new(&mut x1,&y1);
    println!("{:?}", x1);   // Console:" [2, 2, 3] "

}
