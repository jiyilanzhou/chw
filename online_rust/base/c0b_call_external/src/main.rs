/*
0. 模块
    用于将函数或结构体按照功能划分

1. 调用外部文件函数的方式 1 ：将模块移至外部同级目录
    a. 将 mod 移至外部 main.rs 的同级目录
    b. 在 main.rs 中欲使用须先引入外部文件中声明的模块如" mod external "，自动
       加载同级目录下的" external.rs "或" external/mod.rs "(外部文件名即模块名[
       外部文件内不可再重要声明此模块名：因外部文件中声明的模块是"外部文件名模块下
       的子模块，即再声明则为模块中的模块"])
    c. 引入作用域后使用

 2. 调用外部文件函数的方式 2 ：调用外部同级目录下" external_directory "目录内的
    mod.rs 文件(若还需内置其它文件，则需要将内置的文件在 mod.rs 中声明)，同理若
    在 mod.rs 声明" mod xxx "则其会自动加载同级目录下的 xxx.rs 或 xxx/mod.rs "

 */

// 实际开发中，模块置于外部
mod m0_local {
    #[allow(non_snake_case)]
    pub fn localFunc() {
        let variable = "chw";
        println!("{}", variable);
    }
}

// 引用作用域(自动加载同级目录下的" f0_external.rs "或" d0_external/mod.rs ")
mod f0_external;
mod d0_external;

fn main() {
    // 调用本地 mod
    m0_local::localFunc();

    // 调用外部同级目录 module 方式 1
    f0_external::extern_file_func();
    // 外部同级目录文件内的模块
    f0_external::f0m0_mod::extern_file_mod_func();

    // 调用外部同级目录 module 方式 2
    d0_external::extern_directory_func();
    d0_external::d0m0_mod::extern_directory_mod_func();
    // 调用外部同级目录 module 下 mod.rs 向外暴露的模块
    d0_external::d0f0_config::conf();
}

