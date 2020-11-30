
/*
1. 工作空间配置:
    a. 先于" crate\Cargo.toml "下配置如
            [workspace]
            members = [
                "adder",
                "add-one",
            ]
    b. 接着" crate "下创建二进制或库文件如执行以下命令:
       "cargo new adder"及"cargo new add-one --lib "

2. 使用工作空间
    a. 按需定义
          如可于" crate\add-one\src\lib.rs "定义函数
    b. 按需配置
        如可于" ..crate\adder\Cargo.toml " 配置
            [dependencies]
            add-one = {path = "../add-one"}
    c. 按需调用
          如可于" crate\adder\src\main.rs "用 use 引入

3. 使用场景
   在构建中大型项目时其模块间常用的方式

*/

use add_one;    // 引用" 作用域 "

fn main() {
    let num = 10;
    let r = add_one::add_one(num);
    println!("r = {}", r);
}
