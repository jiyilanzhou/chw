
/* 使用外部库：
    1. 先在" Cargo.toml "文件内" [dependencies] "模块下配置依赖
        [dependencies]
        rust-crypto = "0.2"     // 库名 = "版本"
    2. 在项目中按需引入(如在" project/src/main.rs "中引入)

*/
/*
" cargo check "出错:
    a.错误信息：// 信息询问是否已安装" gcc "(Windows下即使安装亦无效)
        Internal error occurred: Failed to find tool. Is `cc` installed?
        (see https://github.com/alexcrichton/gcc-rs#compile-time-requirements for help)
    b. 分析：
       在路径中找不到开发工具"cc"；
    c. 排查
       确保已安装包"gcc",若系统上已安装"gcc"则检查"CC"是否在您的路径中
       命令" which cc ":
        ​$ which cc
         which: no cc in ... (path)
    d. 解决方案( Windows 下待解决)：
          Linux 系统下安装" gcc "即可正常" cargo build "

*/

extern crate crypto;

use crypto::digest::Digest;
use crypto::sha3::Sha3;

fn main() {
    let mut hasher = Sha3::sha3_256();
    hasher.input_str("hello world");
    let result = hasher.result_str();
    println!("hash = {}", result);

}
