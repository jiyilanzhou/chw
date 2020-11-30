
/*
7.2.6 ref 和 mut (p85~88[*])
    a. 需要绑定被匹配对象的引用时可用 ref 关键字(避免所有权转移：因模式匹配有可能
       发生变量的所有权转换)如:
       let x = 5_i32;
       match x {
           ref x => println!("Got a reference to {}", r),   // r 类型为" &i32 "
       }
    b. ref 与 & 符号
       (0). ref 是"模式"的一部分，只能出现在赋值符号左侧
       (1). & 符号是借用运算符，只能出现在赋值号右侧
       (2). "let ref x = 5_i32;"及"let x = &5_i32;"中变量 x 是同样的类型" &i32 "
            "let ref x = &5_i32;"语句中变量 x 绑定的类型是" &&i32 "
       (3). mut 亦是模式的一部分(类比 ref )，只有使用 mut 修饰的变量才可变。
       (4). mut 关键字不仅可在模式中用于修饰变量绑定亦能修饰指针如
            let mut x : &mut i32;
            // 第一个 mut 表示变量 x 可变，第二个 mut 修饰的是指针代表其所指向的
               内存具有修改能力(因此可用类似" *x = 1; "语句修改其所指向的内存值)
       (5). 某些场景必须使用 ref 来进行变量绑定，其原因与" move "有关

*/
fn main() {
    let mut x: Option<String> = Some("hello".into());
    /* // 修复方案1 ：使用 ref
        match x {
            // 编译报错:" error[E0382]: borrow of moved value: `x` "
            // Some(mut i) => i.push_str(" world"),
            // 修改方案1 ：使用 ref
            Some(ref mut i) => i.push_str(" world"),
            None => println!("None"),
        }
    */
    // 修复方式2：使用 &mut
    match &mut x {
        Some(i) => i.push_str(" world"),
        None => println!("None"),
    }
    println!("{:?}",x);
}