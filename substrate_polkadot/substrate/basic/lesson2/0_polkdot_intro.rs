
// pallet[ˈpælət]n.画板,托盘           // tutorial[tjuːˈtɔːriəl]n.教导(课程)
// recipe[ˈresəpi]n.食谱              // consensus[kənˈsensəs]n.共识(一致)
// collate[kəˈleɪt]v.整理,校对        // collator[kɑ'letɚ]n.整理器
// seminar[ˈsemɪnɑː(r)]n.研讨会       // sample[ˈsɑːmpl]n.样本
// playground[ˈpleɪɡraʊnd]n.运动场,操场
// TPS:Transactions Per Second(每秒传输的事物处理个数[系统吞吐量])
/*
0. Polkadot 介绍
    a. 互联网区块链未来 : 跨链
    b. 区块链发展现状：公链与联盟链背向而行(几乎无交集[图"0_区块链发展现状.png"])
        (0). 公链：维护成本高、升级困难、开发复杂
             公链生态：Defi(去中心化金融)、Crypto(加密)、Staking(链上治理)及
                      Consensus(共识)等。因其开源
        (1). 联盟链(CA准入机制[如 hyperledger] )：扩展性低、功能单一、生态单调
             联盟链生态：业务(目前大多公司仅因业务需要开发联盟链且仅内部使用)
        (2). 又因为公链与联盟链间使用的工具、框架不同导致两者几乎无交集
    c. 波卡欲将不同的区块链连接到一起，其愿景是"一个跨链协作的未来"
    d. 为什么跨链是未来
        (0). 区块链可以解决信任问题(源于不可篡改特性)
        (1). 专业化解决可扩展问题(因 TPS 于区块链而言又是天然的瓶颈)
             波卡"relay chain"及 parachain(专注领域内问题)"专业化解决可扩展问题
        (2). 可交互在可扩展前提下，可解决更广泛的信任问题，是一个更真实的世界
        // 波卡可专业化解决" 跨链协作、可扩展性 及 共享安全 "三个问题，其尝试打通
           整个区块链生态沟通协作(如将"BTC、ETH 及其它联盟链等"连接到一个生态内)
    e. 传统区块链：只能根据不同场景在"可扩展性、安全性及去中心化程度间"做不同取舍
        (图"1_传统区块链不可能三角关系.png")
    f. 波卡网络拓扑结构(图"2_Polkadot拓扑结构.png")
        (0). 区块链整理结点(Collators)跟随平行链(Parachain)
                // 每一个 Parachain 都有自己的 Collators
        (1). 区块验证结点(Validators)位于中继链(Relay Chain)
             // 如 Collators 结点整理出下一个区块样例后交予中继链上 Validators
                结点盖章及打包，通过验证后加入中继链
             // Relay Chain 及 Parachain 皆是基于 Substrate 开发，故理论上一级
                Parachain 又可作为二级 Parachain 的 Relay Chain (如此可无限扩展)
        (2). 桥(Bridge)
    g. 目前联盟链是趋势，公链是未来。而 Substrate 是嫁接联盟链与公链的框架。
       ( Substrate 既适用于公链又能完全满足联盟链需求)

1. Substrate
    a. Substrate (源于 Polkadot 提取)用于构造区块链的框架
    b. 区块链结点一般包含的组件(图"3_区块链构造组件.png")
        (0). 数据库
        (1). 点对点网络
        (2). 共识算法
        (3). 交易处理
        (4). 状态转换函数(Runtime)
             上层需要一个状态转换函数，在 Substrate 框架中这一层统称为 Runtime
        (5). 其余可能包含的特别函数：零知识证明(ZK)、分片...
    c. Substrate 特点
        (0). 可扩展性
        (1). 模块化
        (2). 开源
        (3). 自主可控
    d. Substrate 具体包含(图"4_Substrate模块内容.png")
        (0). 核心模块
        (1). 基本逻辑
        (2). p2p 网络
        (3). 共识机制
        (4). 链上治理
    e. Polkadot 基于 Substrate 建立的模块(图"5_Polkadot基于Substrate建立的模块.png")
       波卡(Polkadot)选取基于 Substrate Runtime 的模块如"Parachain、Collators"等结点。
       波卡是完全架构于 Substrate 之上故理解 Substrate 则完全可看懂 Polkadot 逻辑
    f. SRML : Substrate Runtime Module Library (图"6_运行时模块库.png")
       Runtime (又称状态转换函数)即链上逻辑，其具体组件目前在 Substrate 不再称之为 SRML
       而是将其统称为" 托盘(pallet) "。几乎所有逻辑上的功能皆可在 SRML 层实现，目前包含
       的 Runtime 组件(即"托盘[pallet]")大约有 100 个左右。故 Substrate 对于大多数用户
       而言是开箱即用(一般功能只需选择组件封装为 Runtime 就可以)，若欲自定义模块也只需
       遵循 Substrate Runtime 层面提供的范式亦可轻松实现
    g. Substrate 支持链上无分叉升级(图"7_无分叉升级.png")
       (0). Substrate Runtime 分为两种：Wasm runtime (from chain[即链上 Runtime 逻辑])
            及 Native runtime (from client[即本地客户端 Runtime 逻辑])
       (1). 当运行 Runtime 逻辑时会先判断 Native runtime 与 Wasm runtime 版本是否一致，
            若一致优先执行本地运行时(Native runtime)逻辑，因执行效率会高一些；若不一致则
            执行链上运行时(Wasm runtime)逻辑，从而避免客户端未能即时升级带来的影响，因为
            有了判断则保证所有客户端执行的 Runtime 逻辑即为链上最新版本的 Runtime 逻辑。
            所以升级时就不会发生分叉现象。
    h. Runtime 升级管理
        (0). Runtime 代码可以通过链上治理访问
        (1). Sudo 模块
        (2). Democracy 模块
        (3). 自定义的模块和逻辑
        (4). Runtime 升级是可选的
    i. 为什么需要链上升级(区块链升级十分困难[因区块链中 Bug 的代价是非常昂贵的])
         ● 修复重要的安全漏洞
         ● 改变核心规则
         ● 添加新功能
         ● 修复链上状态
         ➢ 硬分叉需要的协作成本极高，且易升级失败
         ➢ 没有明确的治理策略和升级时间点
         // 传统区块链升级成本非常高，没有明确的升级策略及升级时间点，升级难以把控。
            而 Substrate 使用 Wasm 升级，其过程无需结点的直接参与。若不使用 Wasm
            如传统区块链则整个网络都需要执行升级操作(其中间的协调及沟通成本非常高)。
    j. Substrate 与企业系统无缝集成
       (0). 其源于 Substrate 之 Off-chain Workers (链下工作机)功能，可与企业无缝
            集成。传统区块链与数据库MySql、Oracle、Hadoop等无法结合但确是所期望的。
       (1). Substrate 之 Off-chain Workers 则允许链上链下直接交互，即在区块链内部
            可直接读写传统数据库或相应的 API 接口，这样可在链下计算较为密集的工作，
            如此既可完成密集计算工作又不影响链上出块速度。Off-chain Workers 是传统
            的公链或者联盟链中所不具备的(这亦是 Substrate 适合联盟链开发的原因之一)。
       (2). Substrate 总体而言均适合公链及联盟链开发。故 Substrate 是公链技术、生态
            和联盟链之间的桥梁。通过 Substrate 官网可分享最先进的区块链技术成果，是
            区块链求知欲强烈开发及研究者的福音(因传统公链与联盟链背向而驰)
       (3). Substrate 官网入门学习资源参考阵营图
            ➢ Tutorials :
            ➢ Recipes : 碎片化小资料
            ➢ Videos : 由 Parity  在 YouTube 上每周二定期举行 Seminar (其可通过于
                        "https://substrate.io"官网最下方点击" Seminar "链接后加入
                        到" Google Calendar "[每周二可加入线上聊天室])
            ➢ Docs : 学习文档
            ➢ Playground : 线上运行
            ➢ Samples : samll Demo
       (4). Substrate 中文学习资源(基于官方入门资源后再学习)
            ➢ BiliBili :
                 https://space.bilibili.com/67358318
            ➢ 知乎 :
                 https://zhuanlan.zhihu.com/substrate
                 https://zhuanlan.zhihu.com/v2web3
            ➢ 微信公众号 ：
                 polkadot中文平台
                 Polkaworld
                 Polkabase

*/