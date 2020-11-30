
/*1、标准库定义的 Option 枚举形式：
        enum Option<T> {
            Some(T),
            None,
        }
*/
//2、使用方式
fn main() {
    let some_number = Some(5);
    let some_string = Some(String::from("a string"));
    let absent_number: Option<i32> = None;

    let x: i32 = 5;
    let y: Option<i32> = Some(5);
    let mut temp = 0;
    match y {
        // 完整写法" Some(i)=>{ temp=i;}, "单条语句可省略"{}"
        Some(i) => temp = i,
        // [自]末分支可省略逗号","(分支分隔符)
        None => println!("do nothing")
    }
    let sum = x + temp;
    println!("sum = {}", sum);

    let result = plus_one(y);
    // match 须穷尽枚举
    match result {
        Some(i) => println!("result = {}", i),
        None => println!("nothing"),
    };

    // if let (按需匹配[避免繁锁的穷尽枚举])简化 match 表达式(穷尽枚举)
    if let Some(value) = plus_one(y) {
        println!("value = {}", value);
    } /*else {
        println!("do nothing");
    }*/

}

// 返回值" Option<i32> "
fn plus_one(x: Option<i32>) -> Option<i32> {
    // match 返回值取决于分支后的代码块
    match x {
        None => None,
        Some(x) => Some(x + 1),
    }
}

/*
Option 是标准库定义的枚举:
源码：   pub enum Option<T> {
             /// No value
             #[stable(feature = "rust1", since = "1.0.0")]
             None,
             /// Some value `T`
             #[stable(feature = "rust1", since = "1.0.0")]
             Some(#[stable(feature = "rust1", since = "1.0.0")] T),
         }

*/