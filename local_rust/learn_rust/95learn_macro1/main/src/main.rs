/*
1. Linux 系统下 Rust 镜像源配置：
   配置于" ~/.cargo/config "文件

2. 使用" workspace "
   a. " ...\95learn_macro1\Cargo.toml "配置" workspace "
        [workspace]
        members = [
            "mac",
            "main",
        ]
   b. 按需配置依赖如" ...\95learn_macro1\main\Cargo.toml "：
        [dependencies]
        mac = {path = "../mac"}
   c. 按需导入使用
        use mac

3. Rust 中的宏主要有两种：声明宏、过程宏
   a. 使用" macro rules！"的声明宏
      典例:" let v = vec![1,2,3]; "即是使用声明宏
   b. 过程宏又主要分为三种
      自定义宏：通过指定" #[derive] "属性添加代码(如在"结构体、枚举"等上)
      类属性宏：定义可用于任意项的自定义属性;
      类函数宏：类似函数但作用于作为参数传递的 Token

4、宏和函数
    a. 宏是为写其它代码而写的代码：主用于减少大量编写和维护代码
    b. 宏仅接受可变参数而函数标签须声明函数参数个数和类型
    c. 宏的定义比函数更复杂
    d. 宏调用前必须定义并将其引入作用域而函数则可以在任何地方定义和调用

*/

use mac;    // 引入作用域

fn main() {
    // 声明宏
    let v = mac::my_vec![1, 2, 3];

    /* " mac::my_vec![1, 2, 3] "据其宏定义等价于：
            let mut temp_vec = Vec::new();
            temp_vec.push(1);
            temp_vec.push(2);
            temp_vec.push(3);
            temp_vec
    */

    println!("v = {:?}", v);

}