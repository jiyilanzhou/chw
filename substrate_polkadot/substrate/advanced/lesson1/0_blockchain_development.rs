
// autonomous[ɔːˈtɒnəməs]adj. 自治的        // staking[ˈsteɪkɪŋ]n.定桩,权益质押
// collate[kəˈleɪt]v.整理,校对               // collator[kɑ'letɚ]n.整理器
// democracy[dɪˈmɒkrəsi]n.民主               // seminar[ˈsemɪnɑː(r)]n.研讨会
/*
0. 区块链发展

1. 公链(开源)
    Defi : 去中心化金融
    DAOs ：去中心化自治组织　Decentralized Autonomous Organizations
    Staking : 权益质押
    ZK : 隐私保护
    Oracle : 链下通信
    Crypto : 加密
    Consensus : 共识
    TEE : 可信执行环境( Trusted Execution Environment )

2. 联盟链
    一般是公司内部针对某种业务场景开发

3. 波卡(把不同的区块链连接到一起)
    a. 愿景：一个跨链协作的未来
    b. 为什么跨链是未来
        ● 区块链可以解决信任问题(因其不可篡改)
        ● 专业化可以解决可扩展问题( TPS 是区块链的天然瓶颈)
        ● 可交互,在可扩展的前提下，解决更广泛的信任问题，是一个更真实的世界。
    c. 波卡可解决下面三个问题
       (0). 跨链协作
       (1). 可扩展性
       (2). 共享安全
       // 而传统区块链根据不同场景在"去中心化程度、安全性及可扩展性"间作权衡取舍(图"1_不可能三角.png")

4. Polkadot 组件(图"2_波卡拓扑结构.png")
    a. Relay Chain 中继链 ( Validator 是在 Relaychain 上)
    b. Parachain 平行链( Collator 跟随着 Parachain )
       ( 一般由 collator 整理出块信息交由 validator 盖章后最终加入到 Relaychain 中)
    c. 每个区块都有自己的整理节点(Collators)
    d. 一级 Parachain 又可作为二级 Parachain 的 Relay Chain (故 Polkadot 理论上可无限拓展)

5. 可交互的联盟链(图"")
    a. 联盟链是趋势、公链是未来
    b. Substrate 是可嫁接联盟链与公链的框架

 */

/*
6. Substrate 介绍
    a. Substrate 是从 Polkadot 提取的构造区块链的框架
    b. 开源、模块化程度高、可扩展性强及自主可控

7. 区块链包含哪些组件
    a. 区块链节点需要
        ● 数据库
        ● 点对点网络
        ● 共识算法
        ● 交易处理
        ● 状态转换函数(Runtime)：决定区块链的整个状态
        ● 其它特别的函数：零知识证明(zk),分片
    b. 当前联盟链及 Substrate 的定制化程度
        ● 当前联盟链定制化程序(图"4_当前联盟链定制程度.png")
        ● Substrate 定制化程序(图"5_Substrate 定制程度.png")
    c. Substrate 具体包含的内容(图"6_Substrate 内容.png")
    d. Polkadot : 基于 Substrate 之上建立的模块(图"7_Polkadot 基于 Substrate 之上建立的模块.png")
    e. SRML ： Substrate Runtime Module Library (图"8_状态转换函数.png")

8. 升级治理
    a. 升级(图"9_不分叉链上升级.png")
    b. 治理(图"10_Runtime 升级的治理方案.png")
        Sudo 模块(使用 sudo 进行链上升级)或 Democracy 模块(通过民主投票升级)等
       (链上治理:是一种智能合约对协议进行更新的治理模式)
    c. 为何需要链上升级
        ● 修复重要的安全漏洞
        ● 改变核心规则
        ● 添加新功能
        ● 修复链上状态
        ➢ 硬分叉需要的协作成本极高，且易升级失败(没有明确的治理策略和升级时间点)
        // Note: 使用 Wasm 升级过程无需节点直接参与。如果不使用 Wasm 则整个网络都需要执行升级操作
    d. Substrate 与企业系统无缝集成(图"11_Substrate 与企业系统无缝集成.png"[传统区块链技术无法集成])
       " Off-chain Workers "提供链上链下直接交互(如链上区块链与链下数据库交互，即区块链内部可直接读写链下
       传统数据库或者一些 API 接口[故可以和现代企业系统无缝集成]；也可在链下进行一些密集型计算，如此既不
       影响出块速度亦能完成一些密集型工作)。" Off-chain Workers "达到的一个效果就是虽然不能保证链下数据是
       真实可信的但可将请求链下数据的动作记录于链上，这样至少可保证请求链下数据的动作受到区块链的约束,如此
       极大推动区块链与传统业务的融合。故 Substrate 的 Off-chain Workers 会极大丰富区块链开发的应用场景。
       (此功能在任何其它公链或联盟链中是没有的，为 Substrate 特有，这也是 Substrate 适合联盟链的原因之一)
    e. Substrate 是公链技术、生态和联盟链之间的桥梁
       # Build a Hub for Developers 官方文档
         " https://substrate.io "(Tutorials、Videos、Recipes、Docs、Playground、Sample、Community)
    f. 加入 Seminar :
        https://www.substrate.io/ -> Seminar -> Add to Google Calendar


 */