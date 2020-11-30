
/*
0. 模式匹配
    a. 一个模式通常由以下组件组合而成：
       (0). 字面量
       (1). 解构的数组、枚举、结构体或元组
       (2). 变量
       (3). 通配符
       (4). 占位符
    b. 模式被用来与某个特定的值进行匹配

1. for 循环
   for 循环是 Rust 中最为常见的循环结构，可在 for 循环内使用模式
   (紧随关键字 for 的值就是一个模式，如" for x in y "中的" x ")

2. 可失败性：模式是否会匹配失败(p537[*])
    a. 模式可分为不可失败(irrefutable)和可失败(refutable)两种类型
    b. 不可失败的模式可匹配任何传入的值(如" let x = value "中的 x
       便是不可失败模式，因为它能够匹配表达式右侧所有可能的返回值)
    c. 可失败模式则可能由于某些特定的值而匹配失败(如 if let 表达式
       " if let Some(x) = value "中的 Some(x) 便是可失败模式，若
       值 value 为 None 则匹配失败)
    d. 函数参数、let 语句及 for 循环只接收不可失败模式，因这些场合
       下程序无法在值不匹配时执行任何有意义的行为
    e. if let 及 while let 表达式用于接收可失败模式，因其设计时就
       将匹配失败的情形考虑在内了：条件表达式的功能就是根据条件成功
       与否执行不同操作(注:亦可接收不可失败模式[编译警告])
    f. 试图在"不可失败模式"的场合中使用"可失败模式"则编译错误如：
       let Some(x) = some_option_value;
       // 若 some_option_value 值为 None 则无法成功匹配 Some(x),
          意味着模式本身是可失败的。然而 let 语句只接收不可失败模式
       // 可使用" if let "修复
    g. 试图在"可失败模式"场景使用"不可失败模式"则为编译警告如：
       if let x = 5 {};
       // 同时使用 if let 与不可失败模式没有任何意义
       // 因此在 match 表达式的匹配分支中，除了最后一个其它必须使用
          可失败模式，而最后的分支则应该使用不可失败模式，因为它需要
          匹配值的所有剩余情况

3. 模式语法
    a. 匹配字面量
    b. 匹配命名变量
       命名变量(named variable)是一种可以匹配任何值的不可失败模式
    c. 多重模式
        match 表达式的分支匹配中可使用" | "一次性匹配多个模式
    d. 使用" ..= "([自]暂不支持" .. "右侧开区间匹配)来匹配区间
       (范围模式只被允许用于数值或 char 值来进行定义，因编译器需要在
        编译时确保范围的区间不为空而" char 和数值正好是 Rust 仅有的
        可以判断区间是否为空的类型)
    e. 使用解构来分解值
       解构"结构体、枚举、元组"及其嵌套模型

4. 忽略模式中的值
    a. 使用" _ "忽略
    b. 使用" .. "忽略

5. 使用匹配守卫添加额外条件
    匹配守卫(match guard)是附加在 match 分支模式后的 if 条件语句
    (匹配守卫的条件可以使用模式中创建的变量)

6. @ 绑定
    @ 运算符允许在测试一个值是否匹配模式的同时创建存储该值的变量

*/
fn main() {
    enum Message {
        Hello { id: i32 },
    }
    let msg = Message::Hello { id: 5 };
    match msg { // Console:" Found an id in range: 5 "
        // 可将 id_variable 命名为与字段同名 id (出于示例目的此处选择不同名称)
        Message::Hello { id:id_variable @ 3..=7 } => {
            println!("Found an id in range: {}", id_variable)
        },
        /* [自]假设去除 @ 符号且同时能绑定到字段的同名变量则更省事(但局限于同名
           变量[不能绑定到非同名变量])即自假想模块为：
               Message::Hello { id: 3..=7 } => {
                  // 假设去除 @ 符号且同时能绑定到字段的同名变量(假想模式[并未成立])
                  println!("Found an id in range: {}", id)
               },
            [自] 为了更加通用使用 @ 符号绑定(是否为同名变量皆可[但皆需显式声明])
                Message::Hello { id:id @ 3..=7 } => {
                    // 同名变量亦需显式声明(统一格式)
                    println!("Found an id in range: {}", id)
                },
            // 总之 ：判定范围并绑定需要使用 @ 显示绑定(包括同名变量)
        */
        Message::Hello { id: 10..=12 } => {
            println!("Found an id in another range ")
        },
        Message::Hello { id } => {
            println!("Found some other id: {}", id)
        },
    }
}