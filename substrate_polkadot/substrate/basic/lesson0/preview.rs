
// preview[ˈpriːvjuː]n.预习,预览
// parity[ˈpærəti]n.平等 // substrate[ˈsʌbstreɪt]n.基质,底层
// polka[ˈpɒlkə]n.波卡  // dot[dɒt]v&n. (打)点
// stake[steɪk]n.赌注     // stack[stæk]n. 堆栈     // stock[stɒk]n.库存,股份
/*
0. Substrate 简介
    a. Substrate 是 Parity 创建的下一代区块链框架
    b. 可基于 Substrate 轻松开发出可升级、高效率、创新性的区块链项目

1. 为什么学习 Substrate
    a. 一个完整的区块链包含了多个模块(十五分钟可创建一个区块链应用)
       共识算法，p2p网络层，交易池，加密算法等等
    b. 创建一个新的区块链项目几种做法
        (0). 分叉现有项目
             弊端：继承原有项目的漏洞、Bug
        (1). 全新项目从零开始
             弊端：工作量大、开发周期长(难以快速迭代)
    c. Substrate 是下一代区块链开发框架
       由 Gavin Wood 带领的 Parity 团队开发，实现和集成了多个区块链项目所必须的模块。
       本身实现了一个在功能性上可以和以太坊比拟的链(可替换模块)
    d. 现已有多个基于 Substrate 的区块链项目正在开发中,可以预见未来基于 Substrate 的
       项目会越来越多(更早一步掌握和熟练 Substrate 开发则保证成功道路上比别人提前一步)
    e. Substrate 三大特性
        (0). 可升级：链本身能以无分叉(全新特性)形式升级，而传统的区块链项目(如 Btc 或
             Eth )则无法实现，其每一次重大升级都需要 一个软分叉或硬分叉且每一次分支都
             令人担心是否为永久性分叉(对社区及安全性造成影响)
        (1). 高效率 ：性能最大化(得益于共识机制)及轻结点的实现(移动端、物联网端、PC端
             安全同链以完全去中心化的方式交互)
        (2). 创新性：保持开发者的最高自由度(其中所有模块皆可定制)，配合相应的智能合约
             则只有想不到的没有做不到的。

2. Substrate 特性： 可升级
    a. 传统区块链项目，每次重大升级都需要硬分叉或者软分叉。很多新的区块链项目每半年左右都
       有执行一次硬分叉的要求，方便新功能的发布(从安全角度而言是一件好事,因为所有结点都会
       试运行一个新的版本；但对于运营而言则可能造成很多不必要的烦恼)。
    b. Substrate 提供了一个更好的，不需要分叉的升级方案
    c. Substrate 结点由两部分组成
       (0). Runtime：动态运行时，其包括链上所有逻辑(如一个区块是如何验证执行的及其区块头
                     又是如何生成的)
       (1). Client：p2p、db、cli、交易池，出块逻辑，Wasm 解释器
    d. 具体实现:
        Substrate Runtime Rust 代码 -> rustc -> Substrate Runtime Wasm 二进制文件
        -> 链上治理模块 -> 链上状态 (升级成功：结点同步区块[图"0/1_可升级.png"])

3. Substrate 特性：高效率     // PoS (Proof of Stake):股权证明机制
    a. Substrate 的设计从各个角度都是以最高的执行效率为出发点
    b. Rust 语言的选择可以在保证安全性的同时拥有最大自由度优化的可能性
    c. 内置的 PoS 共识机制避免了不必要的挖矿和高额度的 TPS
    d. 轻节点
    e. Patricia Merkle Tree 实现的 Trie DE
    f. 高效率，轻量级，去中心化，去信任化的方式，验证区块头和链上数据。使得移动端、物联网端
       和网页端的轻结点实现的可能性(如手机钱包就可以内置一个轻结点,用于验证链上所有用户交易
       数据，如此则一个全结点是没有能力伪造数据来欺骗这个轻结点的。所以手机钱包就可保护用户
       的资金安全，不需要无条件地相信所链接的全结点，避免双花的可能性，保证用户的资金安全)

4. Substrate特性：创新性
    a. 抽象的模块化接口
    b. Webassembly 解释器
    c. 不同开发语言实现
    d. 第三方库
    e. 定制模块
    f. 智能合约

5. Substrate 特性总结：
    a. Substrate 可以帮助开发出最好的区块链
    b. 抽象的模块化接口和 WebAssembly 解释器的设计
    c. 可升级
        避免了硬分叉的需求
    d. 高效率
        高额度 TPS
        轻节点
    e. 创新性
        只有想不到的没有不能通过 Substrate 实现的

// relay[rɪˈleɪ]n.中继,继电器,接力     // parallel[ˈpærəlel]n.平行,并列
6. Polkadot 波卡生态圈
    a. Polkadot 是 Parity 基于 Substrate 开发的下一代区块链项目(波卡链)
    b. Polkadot Poc-2 版本是 Substrate 的前身
       当时 Parity 团队在开发 Polkadot 时意识到可以将此项目设计为一个通用的区块链开发框架(
       而非仅仅是单一的区块链项目)，于是 Substrate 就诞生了(之后 Parity 基于 Substrate 又
       进行了新版本的 Polkadot 开发)
    c. 波卡链是一个中继链，为了实现跨链沟通
    d. 波卡链支持私有链，公链， oracles（预言机）等组件之间去信任化的交流
    e. 波卡生态圈包含了 relay chain 中继链， parachain 平行链， bridges 转接桥
       (0). Relay chain 中继链的作用是达成关于平行链和中继链状态的共识
       (1). Parachain 平行链是独立的区块链，拥有自己独立的链上数据和交易处理，但同时利用了
            中继链来保证安全性(解决传统区块链项目"扩展"的问题)
       (1). Bridges 跨链桥接(其实是平行链的一种)让波卡生态圈能够与其它原生不在波卡生态圈中
            的独立链进行交流，如比特币和以太坊(图"2_波卡生态圈.png")

7. Polkadot 波卡生态圈总结
    a. Polkadot 波卡生态圈可以实现真正的去中心化的互联网生态圈
    b. Relay chain 中继链：跨链交流
    c. Parachain 平行链：实现业务逻辑
    d. Bridges 转接桥：让其它链亦能加入波卡生态圈

8. 启动结点(参考网络部分资源[未操作])
    a. 环境
       安装 Rust 环境和相关工具
       curl https://getsubstrate.io -sSf | bash -s -- --fast
    b. 获取代码
        git clone https://github.com/paritytech/substrate
        cd substrate
        git checkout v1.0       // (Substrate不断更新,为避免不必要的麻烦使用 v1.0 版本)
    c. 初始化
        ./scripts/init.sh
    d. 编译
        ./scripts/build.sh
        cargo build / cargo build --release

 */


/*
0. What is Substrate?
    Substrate 是一个开源的、模块化的、可扩展的框架

1. Substrate 提供了如下所示的区块链核心组件
    a. 数据库层
    b. 网络传输层
    c. 共识引擎
    d. 交易队列
    e. Runtime 工具库
    // 每一层的组件都可扩展并且支持自定义的

2. The Substrate Runtime
    Runtime 是区块链的链上运行逻辑的集合。也就是状态转换函数
    (图"3_Runtime.png")

3. The Substrate 总体层次
    ------ technical freedom ----
                    Substrate core
              Substrate SRML  // 即 Runtime
      Substrate Node
    ------ development ease ------
    （图"5_总体层次.png"）

 */
