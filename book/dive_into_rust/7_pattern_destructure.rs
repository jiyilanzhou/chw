
// exhaustive[ɪɡˈzɔːstɪv]adj.详尽的，彻底的
/*
0. 模式解构

7.1 简介
    a. Pattern Destructure 模式解构 与 Destrutor 析构器区别
       (0). Pattern Destructure 模式解构：即把原来的结构肢解为单独的、局部的、
            原始的部分
       (1). Destructor 析构器(与"构造器"对应): 对应销毁时调用
    b. 赋值符号左侧即是"模式"而右侧则是需要被"解构"的内容
    c. 模式解构可用于" let 语句、match、if let、while let、函数调用、闭包调用 "
       等情况中而 match 具有功能最强大的模式匹配

7.2.1 exhaustive    // exhaustive[ɪɡˈzɔːstɪv]adj.详尽的，彻底的
    " #[non_exhaustive] "强制使用者 match 语句必须使用下划线"_"方能穷尽枚举)
    // 注：在使用 match 时在匹配成员可能更改的数据时应必须使用"_"穷尽枚举(可扩展性)

*/
#[non_exhaustive]
/* 若未使用" #[non_exhaustive] "则在上游库中 enum 增加成员
   则导致下游库使用者 match 语句(未使用"_"的穷尽成员枚举)编译
   不过(解决方案 : 使用" #[non_exhaustive] "强制使用者 match
   语句必须使用下划线"_"方能穷尽枚举[便于上游可扩展])
*/
pub enum Error {
    NotFound,
    PermissionDenied,
    ConnectionRefused,
}
fn main_0() {
    let e = Error::NotFound;
    match e {
        Error::NotFound => { println!("NotFound"); }
        Error::PermissionDenied => { println!("PermissionDenied") }
        Error::ConnectionRefused => { println!("ConnectionRefused") }
        // " #[non_exhaustive] "强制 match 语句使用"_"穷尽枚举[便于上游可扩展])
        _=>println!("Other")
    }
}

/*
7.2.2 下划线
    a. 下划线"_"(更类似"关键字"而非普通"标识符")不能当作普通标识符使用如:
            let _ = 3;
            println!("{}",_);
        // 编译报错 : " expected expression, found reserved identifier `_` "
    b. 若在下划线后紧接"字母、数字或下划线"则其成为一个正常的标识符(Identifier)
       即"两个连续下划线就是一个合法的、正常的标识符"如
            let __ = 3;
            println!("{}",__);      // Console:" 3 "
    c. " let _ = x; "与" let _y = x； "区别
        (0). " let _ = x; "若变量 x 为非 Copy 类型则为"忽略绑定"
        (1). " let _y = x; "若变量 x 为非 Copy 类型则为"所有权转移(即正常绑定)"
    d. 下划线亦可用于 match 表达式中表示其它分支，在模式中作为占位符(两个点" .. "
       亦用作模式中的"占位符")，还可在类型中做占位符，在整数和小数字面量中做连接符等
    e. 下划线" _ "表示省略一个元素，两个点" .. "可表示省略多个元素

7.2.3 match 表达式(p82~83[*])
    a. match 表达式的每个分支可为表达式，它们要么使用逗号分开要么使用大括号包起来且
       每个分支都必须具备相同的类型
    b. match 可匹配"结构"亦可匹配"值"：可使用或运算符" | "匹配多个条件、使用范围" .. "
       匹配等

*/
//#![feature(exclusive_range_pattern)]    // 解决" .. "编译报错
fn main_1() {
    let x = 3;
    /* match 表达式：若 match 之" {} "后无分号即" match x {} "
      则返回 match 表达式的值
    */
    match x {
        0 => 6,             // 值匹配
        1 | 2 | 3 => 8,     // 多条件匹配
        // 未添加" #![feature(exclusive_range_pattern)] "会编译报错
        4..9 => { 3 }       // 范围匹配(可用" 4..=8 "替代)
        _ => { 0 }
    };  /* 此处无分号则编译报错(因无分号则返回 match 表达式的值)
            error[E0308]: mismatched types
             /     match x{
             |         1 =>10,
             |         _=>3
             |     }
             |_____^ expected (), found integer
             // 分析：match 之 {} 后无分号则将 match 表达式的值作为
                     返回值返回(而" main() "函数未声明返回值故不匹配)
        */
    println!("{:?}",x);
}

/*
7.2.4 Guards 匹配守卫
    a. 可用 if 作为 匹配守卫(match guards)如
        let x = 10;
        match x {
            i if i > 5 => println!("bigger than five"),
            i if i <= 5 => println!("small or equal to five"),
            / * 匹配时编译器依然会保证"完整无遗漏"检查，但编译器内部并没有
               完整的数学解算功能故需添加" _ "分支(单纯为避免编译错误)
            * /
            _ => unreachable!()
        }
    b. 当匹配的值同时符合多条分支时则仅执行首匹配成功的分支(与分支先后顺序相关)

7.2.5  变量绑定
    a. 可用符号 @ 绑定变量：@ 符号前为新声明的变量；@ 符号后为需匹配的模式
        let x = 1
        match x {
           e @ 1..=3 => println!("got a range element {}",e),
           _ => println!("anything"),
        }
    b. 使用 @ 的同时使用 | 则需在每个条件上都绑定变量名
       let x = 1
        match x {
           e @ 1..=3 | e @ 6..=8 => println!("got a range element {}",e),
           _ => println!("anything"),
        }

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
       (4). mut 关键字不仅可在模式中用于修饰变量绑定亦可修饰指针如
            let mut x : &mut i32;
            // 第一个 mut 表示变量 x 可变，第二个 mut 修饰的是指针代表其所指向的内存
                具有修改能力(因此可用类似" *x = 1; "语句修改其所指向的内存值)
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
    println!("{:?}",x); // Console:" Some("hello world") "
}

/*
7.2.3 if-let 和 while-let
    简化 match 穷尽枚举

7.4 函数和闭包参数做模式解构
    函数接受结构体参数，可直接于参数列表做模式解构

*/