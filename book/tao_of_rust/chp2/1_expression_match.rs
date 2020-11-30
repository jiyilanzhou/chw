
/*
2.5 流程控制
    一般编程语言都会有常用的流程控制语句：条件语句及循环语句。Rust 亦不例外但在 Rust 中其确切
    语义为" 流程控制表达式 "(而非流程控制语句)

2.5.1 条件表达式
    a. 表达式一定会有值故 if 表达式的分支必须返回同一类型值(这也是 Rust 无"三元表达式 ?: "的
       原因)。if 表达式求值规则同"块表达式"一致
    b. 对比 Golang 之" if...else "语句：不同分支可返回不同类型的结果值
        // ternary['tɜːnərɪ]adj&n.三元(的)，三重   // ternary expression 三元表达式
        func ternaryExpression(i interface{}) interface{}{
            if val,bl:= i.(int);bl{
                return  val;
            }else if value,bool:=i.(string);bool{
                return value;
            }
            return nil;
        }
        func main() {
            variable:=3
            fmt.Println(ternaryExpression(variable))
            vary :="chw"
            fmt.Println(ternaryExpression(vary))
        }

2.5.2 循环表达式
    // sense[sens]n.感觉，感官  // sensitive[ˈsensətɪv]adj.敏感的
    a. Rust 中包括三种循环表达式：while、loop 和 for...in 表达式
    b. 需要用无限循环时务必使用 loop 而避免使用 while true : 编译器在对 while 循环做流分析
       (Flow Sensitive)时不会检查循环条件(因循环条件可真可假[即并非一定能进入循环]故循环体
       内的表达式会被忽略[即编译器仅知晓 while true 循环返回的是单元值" () "])。这也是受到
       " CTFE "功能限制以致" while 条件表达式"无法作为编译器常量来使用(只有等将来" CTFE "
       功能完善其或许可用)。同理" if true "单分支情况类似
    c. 示例代码
            fn while_ture(x:i32)->i32{
                // 飘红报错：mismatched types [E0308] expected `i32`, found `()`
                while true{
                   return  x+1;
                }
            }

2.5.3 match 表达式与模式匹配
    a. Rust 提供 match 表达式(类比其它编程语言中的 switch 或 case 语句)用于匹配各种穷举情况。
       在 Rust 编程语言中 match 分支模式匹配(Pattern Matching)技术用于判断类型或值是否存在
       可匹配的模式( match 分支左侧为模式右侧为执行代码)。模式匹配亦是表达式(类似 if 表达式)
       故其所有分支必须返回同一类型(但左侧模式可不同)
    b. 代码清单：
           fn main() {
               let number = 42;
               // 左侧模式分别为"单个值、范围、多个值"
               match number {
                   0 => println!("Origin"),
                   1..=3 => println!("All"),
                   5 | 7 | 13 => println!("Bad Luck"),
                   // 绑定模式(Binding Mode):操作符"@"将模式中的值绑定给变量以供右侧代码使用
                   n @ 42 => println!("Answer is {}", n),
                   // match 表达式必须穷尽每一种故一般情况下使用" _ "通配符来处理剩余情况
                   _ => println!("Common"),
               }
           }
    c. 除 match 表达式还有 let 绑定、函数参数、for 循环等位置皆使用到了" 模式匹配"

2.5.4 "if let、while let"表达式：左侧为"模式"右侧为"要匹配的值"([自]"="左右侧组合为"模式匹配"
      其后的"代码块"为"执行代码"),区别于"match"表达式(其内 match 分支即"=>"左侧为"模式"[用于
      "匹配"外部的 match 表达式：即"内外组合"为"模式匹配"]，"=>"右侧为"执行代码")
    a. 使用 if let 表达式(简化 match 表达式)
            fn main() {
                let boolean = true;
                let mut binary = 0;
                /*// 使用 match 表达式
                    match binary {
                        0 => {
                            binary = 1;
                        }
                        _ => (),
                    }
                */
                / * "if let、while let"表达式：左侧为"模式"右侧为"要匹配的值"([自]"="左右侧组合
                   为"模式匹配"其后的"代码块"为"执行代码"),区别于"match"表达式(其内 match 分支
                   即"=>"左侧为"模式"[用于"匹配"外部的 match 表达式：即"内外组合"为"模式匹配"]，
                   "=>"右侧为"执行代码")
                * /
                if let ture = boolean { // 使用 if let 表达式简化 match 表达式
                    binary = 1;
                }
                assert_eq!(binary, 1);
            }
    b. 使用 match 表达式
            fn main() {
                // 创建动态数组 v
                let mut v = vec![1, 2, 3, 4, 5];
                loop {
                    /* 通过 pop 方法依次弹出："pop"方法返回 Option 类型(引入 Option 类型是为了
                       防止空指针出现)故用 match 匹配" Some(x) 及 None "两种情况：其 Some(x)
                       用于匹配数组中的元素而 None 用于匹配数组被取空的情况)
                    */
                    match v.pop() {
                        // Console:" 5	4	3	2	1 "
                        Some(x) => print!("{}\t",x),
                        // None 分支代码仅跳出循环(较为繁锁：可用 while let 简化)
                        None => break,
                    }
                }
            }
    c. 使用 while let (简化 match 表达式)
            fn main() {
                // 创建动态数组 v
                let mut v = vec![1, 2, 3, 4, 5];
                /* "while let"表达式(类似"if let")："="左侧为"匹配模式"，其会匹配
                   右侧 pop方法返回的 Option 类型结果并自动创建 x 绑定(以供 print!
                   宏使用)，若数组中的值取空则自动跳出循环
                */
                while let Some(x)=v.pop(){
                    // Console:" 5	4	3	2	1 "
                    print!("{}\t",x);
                }
            }
    Interview: "if let、while let"作用(简化 match 表达式)及其与" match 分支表达式的区别"

*/