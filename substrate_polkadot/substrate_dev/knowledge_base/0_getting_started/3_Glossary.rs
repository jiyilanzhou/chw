
/*
0. Glossary 词汇表
    a. Author 作者
        负责创建块的节点，参与者或身份
    b. Aggregation 聚合
        将多个运行时模块的相似类型组合为一个enum具有各种变体的单个类型，以允许表示每个
        模块的相似类型。当前有五种这样的数据类型：
            Log （可扩展的标题项）
            Event （某种特定状态转换的指示）
            Call （函子，允许使用一组参数调用已发布的函数）
            Origin （函数调用的来源）
            Metadata （允许对上述内容进行自省的信息）
    c. Authority 权威
        权威是共同管理网络共识的行为者，关键人物或身份
    d. Aura 光环(又名"权威回合")
        具有非即时确定性的确定性共识协议
    e. Aurand + GRANDPA  欧兰 + 格兰帕
        一种混合共识方案
    f. BFT : Byzantine Fault Tolerance 拜占庭容错
        分布式计算机系统面对一定比例的有缺陷的节点或参与者时仍可保持运行的能力。
    g. Consensus
        能够确保一组不一定彼此信任的参与者达成共识的算法
    h. Cropto Primitives 加密基元
        签名方案和哈希算法。这些用于"区块链、状态、共识 及 交易身份验证"
    i. countil 议会
    j. Digest 文摘
    k. Equivocating 模棱两可
    l. Ethash
        工作量证明（PoW）共识算法中使用的许多工作量证明之一
    m. Evnets
        为了链下世界的利益，记录某种特定状态转换的一种方式
    n. Excutor
        在给定的运行时中使用一组外部性执行函数调用的方法
    o. Extrinsic 外在
    p. Existential Deposit 现有条款
    q. Finality 终结性
        达成进展共识的一部分不可逆
    r. Genesis Configuration 创世配置
    s. GRANDPA
        基于 GHOST 的递归祖先衍生协议
    t. Hybrid Consensus Protocol 混合共识协议
    u. Keystore 密钥库
    v. Libp2p 对答网络库
    w. Nominated Proof-of-Stake (NPoS) 提名权益证明
    x. OAS3
        OpenAPI 规范 3（OAS3），正式称为 Swagger API 规范，是用于以 JSON 或 YAML 格式
        编写 Swagger 文件的规范，可用于生成 API 参考文档
    y. State Transition Function (STF) 状态转换功能
    z. Transaction Era  交易时代

*/