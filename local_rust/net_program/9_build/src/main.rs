
/*
0. Cargo 构建脚本(源于《 Cargo 之书 》)
   #文档(可参阅文档" http://llever.com/cargo-book-zh/reference/index.zh.html "
            " https://books.budshome.com/cargo/reference/build-scripts.html ")
    a. 实际开发中有些包需要编译或依赖第三方非 Rust 代码(如 C 库[这些库可位于
       系统上亦可从源代码构建]：在 Rust 中可由 Cargo 提供构建脚本来满足需求)
    b. 通过指定 build 命令执行的 Rust 文件应在包编译其它内容之前被编译，从而
       具备 Rust 代码所依赖或生成的组件
    c. Build 通常被用用于
        (0). 构建一个捆绑的 C 库
        (1). 在主机系统上找到 C 库
        (2). 生成 Rust 模块
        (3). 为 crate 执行所需的某平台特定配置

1. 具体操作
    a. 先于" Cargo.toml "中添加依赖如：
        [package]
        name = "crate_name"
        ...
        build = "build.rs"      # 指定构建文件(执行于其它文件编译之前)
    b. 在 crate 目录下创建 build.rs (即与 src 同级)

 */
/*  // 通过 build.rs 构建 say_hello() 函数
    fn say_hello()->&'static str {
       "hello"
    }
 */
include!(concat!(env!("OUT_DIR"), "/say_hello.rs"));
extern { fn hello(); }
fn main() {
    // 调用源于" say_hello.rs "文件( build.rs 生成)
    println!("{}", say_hello());

    // 调用源于" src/hello.c "文件(须手动编辑)
    unsafe {
        hello();
    }

}
