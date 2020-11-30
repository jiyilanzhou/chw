
/*
0. Rust 语言特性

1.1. 版本和发布策略
     a. 每隔 6 个星期发布一个新 stable 版本
     b. 新功能增加步骤:
        RFC -> Nightly -> Beta -> Stable
        // 推荐多阅读 RFC 文档( https://github.com/rust-lang/rfcs )
     c. Rust 核心库、标准库文档
        https://doc.rust-lang.org/core
        https://doc.rust-lang.org/std/
        // 查阅文档是必备技能

1.2. 安装开发环境
     a. 设置环境变量
          RUSTUP_DIST_SERVER=https://mirrors.ustc.edu.cn/rust-static
          RUSTUP_UPDATE_ROOT=https://mirrors.ustc.edu.cn/rust-static/rustup
     b. 使用 rustup 管理工具链
          rustup self update         // 更新 rustup 本身
          rustup self uninstall      // 卸载 rust 所有程序
          rustup update              // 更新 工具链
          // 亦可使用 rustup 轻松在 stable/beta/nightly 渠道中切换
          rustup install nightly     // 安装 nightly 版本编译工具链
          rustup default nightly     // 设置默认工具链是 nightly 版本
          rustup toolchain list      // List installed toolchains
     c. 查看相应版本
          rustc --version
          cargo --version
     d. 通过 包管理工具 cargo 轻松导入或发布开源库
          官方管理仓库( https://crate.io/ [推荐登录：浏览社区热门开源库])。大型项目往往
          依赖这些开源库，可于 Cargo 目录(如" E:\System\Cargo ")下创建名为" config "的
          文本文件其内容如下:
           [source.crates-io]
           registry = "https://github.com/rust-lang/crates.io-index"
           replace-with = 'ustc'
           [source.ustc]
           registry = "git://mirrors.ustc.edu.cn/crates.io-index"
           // 如此在编译需要依赖 crates.io 的项目时不会由于网络问题导致依赖库下载失败
    e. RLS(Rust language Server)：官方提供的标准化编辑器增强工具。
        (0). RLS 亦是开源的，其项目地址" https://github.com/rust-lang-nursery/rls "
        (1). RLS 是 RUST 语言后台运行服务器，为 IDE、编辑器和其它工具提供有关 Rust 程序的
             有关信息实现复杂功能。支持诸如"代码自动提示、跳转到定义、显示函数名、符号搜索、
             重新格式化、重命名及重构"等功能
        (2). 安装 RLS
             rustup update              // 更新 rustup 到最新
             rustup update nightly      // 更新 rust 编译器到最新的 nightly 版本
             // 安装 RLS
             rustup component add rls --toolchain nightly
             rustup component add rust-analysis --toolchain nightly
             rustup component add rust-src --toolchain nightly

1.5 Format 格式详解(p8[*])
    println!("{a} {b} {b}", a = "x", b = "y");  // 命名参数
    // Rust 设计使用"宏"做标准输出源于更好的错误检查(因"函数[如 println ]不具备"字符串
       格式化的静态检查功能[若出现不匹配情况只能是运行时错误])。 "宏"最终还是调用标准库
       "std::io"模块内函数完成(若开发者能精确控制标准输出操作亦可直接调用标准库来完成)。

*/