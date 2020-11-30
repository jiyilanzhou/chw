
/*
0. 枚举

1. Option 枚举
    a. 很多其它语言都有空值(null、none、nil)概念
    b. Rust 中使用特殊枚举 Option 来代替来表达是否存在
    c. Option 源码:
         enum Optin<T> {
             Some(T),
             None,
         }

*/
#[derive(Debug)]
enum Sex {
    Male,
    Female(String, i32),
}

#[derive(Debug)]
struct User {
    id: i32,
    sex: Sex,
    name: Option<String>,
}

fn check(u: User) {
    /*  // 使用 match 解构(穷尽枚举)
        match u.sex {
            Sex::Male=>println!("男性"),
            // 使用" _  / .. "忽略单个或多个参数
            Sex::Female(_,_)=>println!("女性"),
            Sex::Female(_,..)=>println!("女性"),
            Sex::Female(..)=>println!("女性")
        }
     */
    /* 使用 if let 解构(简化 match )：无需穷尽枚举
          解构使用" = "即模式匹配(而非" == "[用于判断])
       如" if let Sex::Male = u.sex "可理解为
          " 尝试将 u.sex 解构为 Sex::Male "或
          " 尝试将 Sex::Male 模式匹配 u.sex "
     */
    if let Sex::Female(.., age) = u.sex {
        print!("女性:\tage = {}", age);
    } /*else if let Sex::Male = u.sex {
        println!("男性");
    }*/
    // 使用 Option
    //println!("{}",u.name.unwrap());
    if let Some(x) = u.name{
        print!("\tname : {}",x);
    }
}

fn main() {
    // 直接读取枚举值
    println!("{:?}", Sex::Male);
    println!("{:?}", Sex::Female(String::from("woman"), 0));
    // 使用枚举
    let u = User {
        id: 0,
        sex: Sex::Female(String::from("woman"), 36),
        name: Some(String::from("静心道"))
    };
    println!("{:?}", u);
    check(u);
}