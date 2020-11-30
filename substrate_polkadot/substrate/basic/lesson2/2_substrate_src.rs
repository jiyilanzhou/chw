
// issue[ˈɪʃuː]n.发行、出版，问题   // pallet[ˈpælət]n.画板,托盘
// riot[ˈraɪət]n.放纵,暴乱
/*
0. Substrate 源码解析：
   # 拉取代码
   $ git clone https://github.com/substrateCourse/substrate-node-template.git

1. 代码架构
    a. node
       " service.rs "封装了底层 core 的组件或服务模块
    b. pallets (实际开发版本多个模块[但"node-template"版本只有"template"模块])
       放置具体的 Runtime 模块(几乎包含区块链的链上具体逻辑如转账、升级、治理等)
    c. runtime : 封装所有 pallet 组成 runtime
        // 此时才规定其内 Currency 的关联类型为 blance 模块(可换为自定义实现模块)
        impl transaction_payment::Trait for Runtime {
            type Currency = balances::Module<Runtime>;
            type OnTransactionPayment = ();
            type TransactionByteFee = TransactionByteFee;
            type WeightToFee = IdentityFee<Balance>;
            type FeeMultiplierUpdate = ();
        }
        // 源于" runtime\src\lib.rs "源码
        /// Used for the module template in `./template.rs`
        // 此处才给 Event 赋于具体的类型
        impl template::Trait for Runtime {
            type Event = Event;
        }
        // 链上逻辑层:其" runtime\src\lib.rs "内" construct_runtime! "源码
        // Create the runtime by composing the FRAME pallets that were previously configured.
        construct_runtime!(
            pub enum Runtime where
                Block = Block,
                NodeBlock = opaque::Block,
                UncheckedExtrinsic = UncheckedExtrinsic
            {       // 选用的 pallet 模块(对应网页左侧栏的"链状态")
                System: system::{Module, Call, Config, Storage, Event<T>},
                RandomnessCollectiveFlip: randomness_collective_flip::{Module, Call, Storage},
                Timestamp: timestamp::{Module, Call, Storage, Inherent},
                Aura: aura::{Module, Config<T>, Inherent},
                Grandpa: grandpa::{Module, Call, Storage, Config, Event},
                Balances: balances::{Module, Call, Storage, Config<T>, Event<T>},
                TransactionPayment: transaction_payment::{Module, Storage},
                Sudo: sudo::{Module, Call, Config<T>, Storage, Event<T>},
                // Include the custom logic from the template pallet in the runtime.
                TemplateModule: template::{Module, Call, Storage, Event<T>},
            }
        );
    d. scripts

2. 如何参与开源项目(图"10_如何参与开源项目.png")
    a. 区块链世界几乎都是开源的(故衡量一个项目是否为区块链的硬性指标即是否开源)
    b. Watch & Star & Fork :
        Watch : 实时观察更新状态
        Star : 表示关注、支持(点赞)此项目(拥抱开源社区的基本素养)
        Fork : 当前时间点的项目拷贝(原项目后续更新则需通过其它方式同步)
    c. 时常关注 issues
       通过 PR 可追踪当前项目的最新进展(了解到目前正在处理的模块或可改进的地方)
    d. 开源社区
       (0). 务必参与到官方开源社区" https://www.substrate.io/ "-> community (网页底部) 选项
       (1). " Events(各类 Parity 的 event ) "及" Stack Overflow (可用于提问的标签页) "等。
       (2). " Riot Chat "：官方聊天室，很多国内外专家都在里面(可向他们请教以获取专业的及时回应)
    // " Polkadot "亦同样操作(可查看" https://github.com/paritytech/polkadot "源码)

 */