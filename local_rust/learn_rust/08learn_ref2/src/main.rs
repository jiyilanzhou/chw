
// dangle [ˈdæŋɡl]n.摇晃，悬挂
/*
1、可变引用后不能再有不可变引用
2、引用必须有效
*/

fn main() {
    let ref_s = dangle();
    println!("Hello, world!");
}

/* 悬垂引用:编译错误
    error[E0106]: missing lifetime specifier
         fn dangle() -> &String {
                        ^ help: consider giving it a 'static lifetime: `&'static`
       = help: this function's return type contains a borrowed value, but there is
                                               no value for it to be borrowed from
    // 分析：指向被回收的数据(悬垂引用)
*/
fn dangle() -> &String {
    let s = String::from("hello");
    &s
}
