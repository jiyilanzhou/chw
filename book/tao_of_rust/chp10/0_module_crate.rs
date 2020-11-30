
// badge[bædʒ]n.徽章，标记
/*
0. 模块化编程

10.1 包管理
    TOML 语言：规范简单、主义明显、阅读性高("Cargo.toml"正是元数据文件之一)
    (" https://github.com/toml-lang/toml ")

10.1.2 使用第三方包
    a. 在 Cargo.toml 文件 [dependencies] 下添加依赖如:
        [dependencies]
        linked-list = "0.0.3"         # Cargo.toml 中使用的是连字符" - "
         # 而 extern crate 声明包的名称" linded_list "使用的是下划线" _ "
         // 注：为统一包名称，Cargo 默认会把连字符" - "转换成下划线" _ "(因
                " lindek-list "和" linked_list "到底是否为同一个包存在歧义)
    b. 按需使用" extern crate "[2018版本可省略]引入第三方包
        extern crate linked_list    # 按需引入使用的是下划线" _ "
    c. 使用正则表达式 regex 包
    d. 惰性静态初始化 lazy_static 包
    e. 指定第三方包的依赖关系
       Rust 包使用的是语义化版本号(SemVer),基本格式为" X.Y.Z "(即分别对应"
       主、次、修订版本号"[参阅" https://semver.org "])

10.1.3 Cargo.toml 文件格式(表示配置)
    [package]
    [badges]
    [workspace]
    [dependencies]
    [features]
    [lib]
    [test]
    [profile]

10.1.4 自定义 Cargo
    a. [registry] : 代表 crate.io 的相关配置
    b. Cargo 配置文件的层级关系
    c. 自定义 Cargo 子命令
    // 注：提交代码前执行" cargo fmt "即可统一团队编码风格

10.2 模块系统

10.3 从零开始实现一个完整功能包(实现".csv"文件内容解析替换)：(p344[*])
    a. 使用 Cargo 创建新项目
    b. 使用 structopt (第三方包) 解析命令行参数
    c. 定义统一的错误类型
    d. 读取 CSV 文件
    e. 替换 CSV 文件中的内容
    f. 进一步完善包(增加"单元、文档、集成及基准"测试)

*/