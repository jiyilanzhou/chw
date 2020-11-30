
// junius@parity.io  课程问题可通过邮箱与周俊联系
// alice['ælɪs]n.爱丽丝(女子名)   // telemetry[təˈlemətri]n.遥测(遥感)
// extrinsic[eksˈtrɪnsɪk]adj.外在的,外来的 // emit[iˈmɪt]v.发射,发出,发行
/*
0. 创建第一条区块链应用
    参考" https://substrate.dev/docs/en/tutorials/create-your-first-substrate-chain/ "

1. 运行单节点的区块链
    # 清除链上所有数据
    root@ubuntu:~/project/substrate# ./target/release/substrate purge-chain --dev
    # 启动
    root@ubuntu:~/project/substrate# ./target/release/substrate  --dev
    # 查看配置( build-spec 生成 spec.json 文件)
    root@ubuntu:~/project/substrate# ./target/release/substrate build-spec >> spec.json
    root@ubuntu:~/project/substrate# vim spec.json       // 此处使用 vim 快速查看

2. 设置日志
    export RUST_LOG=info    // 默认(可设置为" trace "或" debug ")

3. 启动参数
        参数                       参数说明
        --base-path                数据存放路径
        --chain                    指定使用的链的类型        // 如 dev / local 等
        --alice                    使用预先定义的密钥
        --port                     p2p通信的端口
        --ws-port                  websocket服务端口
        --rpc-port                 rpc服务端口
        --node-key                 指定libp2p使用的私钥
        --telemetry-url            统计数据提交的地址
        --validator                作为验证人加入网络
        --light                    运行轻客户端模式

4. 运行两个节点的区块链
    (参阅" https://substrate.dev/docs/en/tutorials/start-a-private-network/ ")
    a. 使用 local 模式来启动第二个节点，来组成一条链
       root@ubuntu:~/project/substrate# ./target/release/substrate --base-path /tmp/alice \
                                        --chain local --alice \
                                        --port 30333 --ws-port 9944 --rpc-port 9933 \
                                        --node-key 0000000000000000000000000000000000000000000000000000000000000001 \
                                        --telemetry-url 'ws://telemetry.polkadot.io:1024 0' \
                                        --validator
        // 注：本地单一结点启动不会产生区块
    b. 启动第二个节点
       root@ubuntu:~/project/substrate# ./target/release/substrate --base-path /tmp/bob \
                                        --chain local --bob \
                                        --port 30334 --ws-port 9945 --rpc-port 9934 \
                                        --telemetry-url 'ws://telemetry.polkadot.io:1024 0' \
                                        --validator

        // 注：启动第二个节点后开始产生区块

5.  Node Template 代码导读
    a. Node Template 代码分析
    b. Code base
        (参阅" https://github.com/SubstrateCourse/substrate-node-template ")
    c. 关于Node Template 知乎文章
        (参阅" https://zhuanlan.zhihu.com/p/123167097 ")


6. Node Template 源码分析
    a. node
        (0). " ...\node\src\service.rs "
             " new_full "方法； " new_light "方法(用于轻节点)
    b. pallets
        (0). " ...\template\src\lib.rs "
             pub trait Trait: system::Trait {
                // Add other types and constants required to configure this pallet.
                /// The overarching event type.
                // 自定义 Event 须可转为 <system::Trait>::Event
                type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
             }
             decl_storage! : 存储位置
             decl_event! :  事件处理
             decl_error! :  错误处理
             decl_module! {
                /// Just a dummy entry point.
                /// function that can be called by the external world as an extrinsics call
                /// takes a parameter of the type `AccountId`, stores it, and emits an event
                #[weight = 10_000]
                pub fn do_something(origin, something:u32) -> dispatch::DispatchResult {...}
             }
        (1). " ...\template\src\mock.rs "
        (2). " ...\template\src\test.rs "
            可使用" cargo test "测试看其是否通过
    c. runtimes
        (0). " ...\runtime\src\lib.rs "
            construct_runtime!(
                pub enum Runtime where
                    Block = Block,
                    NodeBlock = opaque::Block,
                    UncheckedExtrinsic = UncheckedExtrinsic
                {
                    System: system::{Module, Call, Config, Storage, Event<T>},
                    RandomnessCollectiveFlip: randomness_collective_flip::{Module, Call, Storage},
                    Timestamp: timestamp::{Module, Call, Storage, Inherent},
                    Aura: aura::{Module, Config<T>, Inherent(Timestamp)},
                    Grandpa: grandpa::{Module, Call, Storage, Config, Event},
                    Balances: balances::{Module, Call, Storage, Config<T>, Event<T>},
                    TransactionPayment: transaction_payment::{Module, Storage},
                    Sudo: sudo::{Module, Call, Config<T>, Storage, Event<T>},
                    // Used for the module template in `./template.rs`
                    // 用于开发的模块
                    TemplateModule: template::{Module, Call, Storage, Event<T>},
                }
            );

 */
