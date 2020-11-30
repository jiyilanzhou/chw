
/*
0. 猜数字(游戏)

1. 处理一次猜测(p19[*])
    " std::io::stdin().read_line(&mut variable) "

2. 使用变量来存储值
    a. 关联函数(associated function)：针对类型本身而非实例定义关联函数，在某些
      语言中亦被称为静态方法(static method)
    b. " std::io::stdin().read_line(&mut variable) "中" & "意味着当前参数是
       一个引用(用于在不同地方访问同一份数据而无需付出多余的拷贝开销)。
    c. (类比变量)引用默认也是"不可变的"，故需使用" &mut variable "声明可变引用
       而非" &variable 不可变引用[仅用于可读]"

3. 使用 Result 类型来处理可能失败的情况
    a. 在 Rust 中提供许多以 Result 命名的类型，它们通常是各个子模块中 Result
       泛型的特定版本，用于编码可能出现的错误处理信息。如" std::io::Result "
    b. Result 是一个枚举类型。枚举类型是由一系列固定的值组合而成，这些值被称作
       枚举的"变体".
    c. 未处理" Result "会出现编译警告" warning: unused 'std::result::Result'
        which must be used "。其意味着未对潜在的错误进行处理(p24[*])

4. 通过" Cargo.lock "文件确保项目构建可重现(p27[*])
    a. Cargo 提供一套机制来确定构建结果是可以重现的，在任何时候重新编译代码都会
       生成相同的产物：Cargo 会一直使用某个特定的版本，依赖直到手动指定其他版本。
    b. Cargo.lock 文件在首次使用 Cargo build 命令构建项目时生成。第一次构建时
       Cargo 会依次遍历项目声明的依赖及其对应的语义化版本，找到符合要求的具体
       版本号写入 Cargo.lock 文件。
    c. 随后再次构建项目时 Cargo 会优先检索 Cargo.lock，若文件已存在指明的具体
       依赖库则跳过搜寻版本号的过程，直接使用 Cargo.lock 文件中指明的版本。这
       使得项目拥有一个自动化、可重现的构建系统

5. 升级依赖包(p27[*])
    a. 当确实需要升级依赖包时 Cargo 提供了一个专用命令" update "会强制 Cargo
       忽略 Cargo.lock 文件并基于语义化版本规则[约定范围]重新计算所有依赖包中
       符合 Cargo.toml 声明的最新版本。若命令执行成功则 Cargo 会将更新后的版本
       号写入 Cargo.lock 文件以覆盖之前内容。
    b. 示例: 假设 Cargo.toml 之" [dependencies] "下配置 rand = "0.3.14 "，则
       使用" cargo update "命令(基于语义化版本规则，Cargo 在自动升级时只会寻找
       大于" 0.3.0 "并小于" 0.4.0 "的最新版本[若 rand 包发布了 0.3.15、0.4.0
       两个版本则更新时会选择" 0.3.15 "版本：可在 Cargo.lock 中查看)。若欲使用
       " 0.4.0 "版本必须在 Cargo.toml 文件中"[dependencies]"下手动地显示指定
       " rand = "0.4.0"。当下一次运行 cargo build 时 Cargo 就会自动更新注册表
       所有可用包的最新版本信息，并根据指定的新版本来重新评估项目对 rand 的需求

*/