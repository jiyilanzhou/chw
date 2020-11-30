
/*
1、模式有两种：refutable（可反驳的）和 irrefutable（不可反驳的）
   a. 匹配可能失败的模式(可反驳的): 如" if let / while let  "按需匹配
   b. 能匹配任何可能值的模式(不可反驳的): 如" match "穷举匹配

2、函数、let 语句、for 循环:
   仅接收"不可反驳"模式(因为通过不匹配的值程序无法进行有意义的工作)

3、if let 和 while let 表达式
   皆被限制为仅接受可反驳的模式(因其定义即是为处理有可能失败的条件)

*/

fn main() {
    // 不可反驳模式(Option<T>) // Option 匹配 Some(value), None
    let a: Option<i32> = Some(5);
    let b: Option<i32> = None;

    /* 编译报错：let 为不可反驳模式而" Some(x) = a " 未能覆盖 None
       error[E0005]:refutable pattern in local binding:`None` not covered
          let Some(x) = a;
            ^^^^^^^ pattern `None` not covered  // " None "未被覆盖到
       //[自]"let"为不可反驳模式而"Some(x)=a"为可反驳模式(因其未能覆盖"None")
    */
    //let Some(x) = a;

    // 可反驳模式
    if let Some(v) = a {
        println!("v = {}", v);
    }

    /* 编译警告："if let"为可反驳模式其后亦可接收不可匹配模式(因不可反驳匹配)
       warning: irrefutable if-let pattern
           /     if let v = 5{
           |         println!("v = {}", v);
           |     }
           |_____^
          = note: `#[warn(irrefutable_let_patterns)]` on by default

        // [自]"if let"为可反驳模式(而"v = 5"虽为不可反驳模式但其匹配结果[true]
               却符合" if let "后继条件[即" if let bool "])
        // 编译器警告：开发中应当作"错误"处理
    */
    if let v = 5{
        println!("v = {}", v);
    }

}