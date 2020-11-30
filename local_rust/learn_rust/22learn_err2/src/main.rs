
/*
1、当编写一个函数，但是该函数可能会失败，此时除了在函数中处理错误外还可将错误传给
   调用者让其决定如何处理，这被称为传播错误。

2、传播错误的简写方式，提倡的方式

3、更进一步的简写

4、什么时候用 panic！，什么时候用 Result
    (1)示例、代码原型、测试用 panic!、unwrap、expect
    (2)实际项目中应该用 Result

5、Option 和 Result

*/

use std::io;
use std::io::Read;
use std::fs::File;
fn main() {
    // 接收处理返回值
    let r = read_username_from_file();
    match r {
        Ok(s) => println!("s = {}", s),
        Err(e) => println!("err = {:?}", e),
    }
}

// 使用 match
fn read_username_from_file0() -> Result<String, io::Error> {
    let f = File::open("hello.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(error) => return Err(error),
    };
    // 匹配返回
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(error) => Err(error),
    }       // 此处为表达式(即返回值[故不可添加分号";"])
}

// 简写方式 1 ：" ? "(代表出错即返回"io::Error")
fn read_username_from_file1() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    // 读取内容到 s
    f.read_to_string(&mut s)?;
    Ok(s)
}

// 简写方式 2 : " ? "链式调用
fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}
