
/*
0. 结构体

1. 自定义输出

 */
use std::fmt::Formatter;

#[derive(Debug)]
struct SN {
    name: String,
    age: u8,
}
impl SN {
    fn to_string(&self)->String{
        format!("the name is : {}\t and The age is {}",&self.name,&self.age)
    }
}

fn main_0() {
    let sn = SN{ name: "chw".to_string(), age: 36 };
    println!("{}",sn.to_string());
    println!("{:#?}",sn);
    println!("{}",sn);    // 需自定义实现 Display trait
}

use std::fmt;
impl fmt::Display for SN{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "SN {{ name: {}, age: {} }}", self.name, self.age)
    }
}

/*
2. 数组、元组
    a. 数组：长度固定且必须是类型相同的集合
       (0). 声明:" [Type;len] "
       (1). 初始化："[elem0,elem1...elemn]" / " [elem;n] "
       (2). Rust 中没有默认零值(须手动指定)，故显式声明数组必须完全初始化如
            " let tags:[&str;3] = ["c","h","w"]; "
            " let tags:[&str;3] = ["";3]; "// 类似的默认零值亦必须手动指定
            // 但前后数量不一致则报错如" let tags:[&str;8] = ["c";3]; "
    b. 元组：类型可不同的集合

 */
fn main() {
    // 声明(数组封装仅有" [] "没有" {}代码块 "或" ()元组 "的用法)
    let tags: [&str; 3] = ["c", "go", "rust"];  // 显示声明类型
    //let tags = ["c","go","rust"]; // 自动推导类型
    //let tags:[&str;8] = ["c";8];  // 显示声明类型
    //let tags = ["c";8];  // 自动推导类型

    // println!("{:?}",tags);  // Console: ["c", "go", "rust"]
    // 范围遍历
    for i in 0..tags.len() {     // 范围：range (0 .. len)
        print!("{}\t", tags[i]); // Console:" c  go  rust "
    }
    // 迭代器遍历
    for i in tags.iter() {     // 迭代器 Iterator
        print!("{}\t", i); // Console:" c  go  rust "
    }
    // Rust 无默认零值(类似的默认零值亦必须自定义初始化)
    let mut arr: [u8; 3] = [0; 3];
    for i in 0..arr.len() {
        // 类型转换: as
        arr[i] = (i + 3) as u8;
        //print!("{}\t",arr[i]);  // console:" 3 4 5 "
    }
    // 带索引的遍历 ：enumerate 组装为携带索引(元组形式)的迭代器
    for item in arr.iter().enumerate() {
        print!("{:?}\t", item);
        // Console:" (0, 3) (1, 4) (2, 5) "
    }
}

/*
3. 练习
   在外部模块创建一个实体类,包含:
    用户 ID i32
    用户名 String
    用户年龄 u8
    用户标签:数组(最多5个)
    提供 new 函数: 用来初始化实体
    // 说情参阅" practice " crate
 */