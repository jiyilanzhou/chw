
/*
0. 错误处理
    a. panic! 宏(不可恢复的异常)
    b. Result 类型(泛型枚举)
       enum Result<T,E>{
            Ok(T),
            Err(E),
       }

1. unwrap / expect、默认值及闭包
    a. match 处理 Result 需穷尽匹配
    b. if let 可简化 match 部分分支但仍需处理 Result
    c. unwrap 用于开发中简化 Result 处理(当 Result 值类型为 Ok 则 unwrap 返回 Ok
       中的值否则 panic!)
    d. expect 相较于 unwrap 优势在于可传递自定义错误信息
    e. unwrap_or 相较于 unwrap 优势在于 Result 为 Err 时提供默认值(而非 panic! )
    f. unwrap_or_else 在 Err 时返回闭包处理 err 的结果

 */

fn func(i: i32) -> Result<String, u8> {
    if i < 6 {
        Ok(String::from("info"))
    } else {
        Err(1)
    }
}


fn main_0() {
    // 接收 Result
    let rs = func(3);
    /* // 使用 if let 处理 Result
        if let Ok(info) = &rs{
            println!("Success : {}",info);
        }else if let Err(code) = &rs{
            println!("Error exit : {}",code)
        }
     */

    /* // 使用 unwrap 处理 Result :
       unwrap 源码:
            pub fn unwrap(self) -> T {
                match self {
                    Ok(t) => t,
                    Err(e) => unwrap_failed("called `Result::unwrap()`
                              on an `Err` value", &e),
                }
            }
     */
    println!("{}", rs.unwrap());

    /* // 使用 expect 处理 Result:
        expect 源码：
            pub fn expect(self, msg: &str) -> T {
                match self {
                    Ok(t) => t,
                    Err(e) => unwrap_failed(msg, &e),
                }
            }
     */
    let rs = func(6);
    //println!("{}",rs.expect("Custom Error Info ... "));

    /* 使用" unwrap_or(default) "在 Err 时提供默认值处理
       unwrap_or 源码：
            pub fn unwrap_or(self, default: T) -> T {
                match self {
                    Ok(t) => t,
                    Err(_) => default,
                }
            }
     */
    let rs = func(8);
    println!("{}", rs.unwrap_or("Default Value ... ".to_string()));

    /* // 使用" unwrap_or_else " : 在 Err 时返回闭包处理 err 的结果
        unwrap_or_else 源码：
            pub fn unwrap_or_else<F: FnOnce(E) -> T>(self, op: F) -> T {
                match self {
                    Ok(t) => t,
                    Err(e) => op(e),
                }
            }
     */
    let rs = func(10);
    println!("{}", rs.unwrap_or_else(|err| (err + 3).to_string()));
}

/*
2. 错误传播 ?

 */

fn step1() -> Result<String, String> {
    Ok("step1 Success".to_string())
}

fn step2() -> Result<String, String> {
    Err("step2 Fail".to_string())
}

fn step() -> Result<String, String> {
    /* // 一般处理
        if let Err(e1) = step1() {
            // if 分支不完整(缺失 else 部分)故使用 return
            return Err(e1);
        } else if let Err(e2) = step2() {
            return Err(e2);
        }
     */
    // 使用 ? 处理错误传播
    step1()?;
    step2()?;
    Ok("Success ... ".to_string())
}

fn main() {
    println!("{:?}",step());
}