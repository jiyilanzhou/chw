
// launch[lɔːntʃ]v.发射,开始(应用程序)
/*
0. Windows 环境搭建

1. 安装 MSYS2
    a. 下载 https://www.msys2.org/
        国内镜像" https://mirrors.tuna.tsinghua.edu.cn/msys2/distrib/x86_64/ "
    b. 下载后为" msys64 "阄将其置于" E:\System\msys64 "
    c. 进入 msys64 目录修改 msys2_shell.cmd 文件(因其默认不会集成" Windows "环境
       变量)。将 17 行首个单词" rem "删除即更改为" set MSYS2_PATH_TYPE=inherit "
       后保存退出(即默认使用 Window 环境变量)。
    d. 双击执行" msys2_shell.cmd "文件

2. 添加环境变量及设置源
    a. 配置 CARGO_HOME、RUSTUP_HOME 环境变量
        CARGO_HOME 如 d: rust\cargo
        RUSTUP_HOME 如 d: rust\rustup
        // 在 PAT H环境变量中加入:" %CARGO_HOME%\bin; "
    b. 设置源
       RUSTUP_UPDATE_ROOT 为  https://mirrors.ustc.edu.cn/rust-static/rustup
       RUSTUP_DIST_SERVER 为  https://mirrors.ustc.edu.cn/rust-static
    c. 将下载好的" rust-init.exe "一般置于与" CARGO_HOME 如 d:rust\cargo "同级
       目录并执行
    d. 安装源码" rustup component add rust-src "

3. Cargo
   a. cargo 是 Rust 项目组织和管理工具，提供项目的建立、构建到测试、运行直至部署
      (类比 go 的 module 与 Java 中 Maven 的结合体 )
   b. cargo build 生成的可执行文件名源于"Cargo.toml"内"[package]"下的" name "
      (其 name 名称一般同于项目名[虽亦可修改但"不推荐"])
   c. 命令可如此执行" cargo build && cargo run "

 */