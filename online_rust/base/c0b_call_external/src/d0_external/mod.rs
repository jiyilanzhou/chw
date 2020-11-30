
// 同理：加载 mod.rs 同级目录下 config.rs 或 config/mod.rs
// mod config; // 仅本文件内可调用(未向外界暴露故外部不可调用)
pub mod d0f0_config; // 使用 pub 向外部暴露(即外部可调用)

pub fn extern_directory_func() {
    println!("External directory func")
}

pub mod d0m0_mod{
    pub fn extern_directory_mod_func() {
        println!("External directory mod func")
    }
}

/*
实际开发中 ：
    mod.rs 常用于仅注册模块(一般不在 mod.rs 内部声明函数)

 */