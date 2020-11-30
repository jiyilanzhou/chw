
// dump[dʌmp]v.清除,转储
/*
0. 安装 Rust 编译工具链
    a. 查看已安装
       rustup show
    b. 安装带日期的版本
       rustup install nightly-2020-09-08
    c. 切换至指定版本
       rustup default nightly-2020-09-08
1. 卸载工具链
    a. 普通卸载
        rustup toolchain uninstall
    b. 卸载指定版本
        rustup toolchain uninstall nightly-2020-09-08

 */

/*
0. 程序入口" bifrost/bin/node/cli/bin/main.rs "
    fn main() -> sc_cli::Result<()> {
        node_cli::run()
    }

 */