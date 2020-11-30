
/* 调用非本" crate "步骤:
1. 配置：" workspace "(配置于"...\66learn_oo2\Cargo.toml")
    [workspace]
    members = [
        "gui",
        "main",
    ]

2. 使用：在"...\66learn_oo2\main\Cargo.toml"配置依赖
    [dependencies]
    gui = {path = "../gui"}

3. 按需导入
    use gui::{Screen, Button, SelectBox};

*/

use gui::{Screen, Button, SelectBox};

fn main() {
    // 创建屏幕(内附组件)
    let s1 = Screen {
        // 组件:内置组件具体实例
        components: vec![
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("ok"),
            }),
            Box::new(SelectBox {
                width: 60,
                height: 40,
                option: vec![
                    String::from("Yes"),
                    String::from("No"),
                    String::from("MayBe"),
                ],
            }),
        ],
    };
    s1.run();

    /* 使用泛型 : (编译错误)类型匹配错误
       error[E0308]: mismatched types
          /   Button {
          |       width: 50,
          |       height: 10,
          |       label: String::from("ok"),
          |   },
          |___^ expected struct `std::boxed::Box`,found struct `gui::Button`
          = note: expected type `std::boxed::Box<dyn gui::Draw>`
                     found type `gui::Button`
        // 原因分析：使用泛型为静态分发(单态化[编译时已确定具体类型])[?]
        // 解决方案：可用动态分发" dyn "(类似多态[通过 dyn trait 指针指向具体对象])
    */
    /* 暂且注释：以通过编译
        let s2 = Screen {
            components: vec![
                Button {
                    width: 50,
                    height: 10,
                    label: String::from("ok"),
                },
            ],
        };
        s2.run();
    */

}