
/*
0. 加载 GitZip 插件(下载部分文件)：
    方案一：加载已下载好的插件
        谷歌浏览器标签页(即地址栏) -> 输入" chrome://extensions/ "->  拖拽已下载好的插件
    方案二： 网上应用商店(推荐使用)
        谷歌浏览器标签页(即地址栏) -> 输入"  https://chrome.google.com/webstore/ "或直接
        点击浏览器工具栏 Apps 图标 -> 搜索" GitZip " -> 加载即可

1. GitZip 插件使用

2.直接使用 git 拉取(推荐使用)
    a. 创建目录->进入目录->" git init "
    b. 首次拉取使用" git clone https://github.com/SubstrateCourse/slides.git "
    c. 关联" git remote add origin  https://github.com/SubstrateCourse/slides.git "
    d. 查看关联" git remote -v "
        $ git remote -v
          origin  https://github.com/SubstrateCourse/slides.git (fetch)
          origin  https://github.com/SubstrateCourse/slides.git (push)
    e. 再次拉取使用" git pull "

 */

/*
0. Substrate 为什么使用宏
    a. 简化 Runtime 开发，Substrate 使用宏建立一套 DSL (Domain Specific Language)，
       设计合理的 DSL 可以：
           ● 很好的被用户理解
           ● 代码更加简洁，提升效率
           ● 解放应用开发者，只需实现业务组件
    b. 修改 rust 镜像源
        更改 $HOME/.cargo/config 为以下内容:
        [source.crates-io]
        registry = "https://github.com/rust-lang/crates.io-index"
        replace-with = 'ustc'
        [source.ustc]
        registry = "git://mirrors.ustc.edu.cn/crates.io-index"
    b. 安装 cargo-expand
        cargo install cargo-expand

 */