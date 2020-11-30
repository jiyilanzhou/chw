
/*
0. 基于 Template 开发 ERC20 (ERC20 主要用于发行资产)
    a. ERC20 规范 : "  https://eips.ethereum.org/EIPS/eip-20 "
    b. 编程实现一个 ERC20 的 Pallet

1. 编程实现 ERC20
    a. 在" substrate-node-template\pallets "下建立 erc20 目录
    b. 编辑 erc20 模块 (参阅"substrate-node-template\pallets\erc20")
    c. 最后将"erc20" pallet 追加至" substrate-node-template\runtime "

2. 将编辑好的 pallet 追加至" substrate-node-template\runtime "模块
    a. 首先在将 pallet 追加到 workspace 的 members 中(根目录 Cargo.toml)
            [workspace]
            members = [
                'node',
                'pallets/template',
                'pallets/erc20',
                'runtime',
            ]
    b. 其次在" substrate-node-template\runtime\Cargo.toml "内追加依赖
       (仿照已添加的 template 依赖:  [dependencies.template]
                                    default-features = false
                                    package = 'pallet-template'
                                    path = '../pallets/template'
                                    version = '2.0.0-rc2'
       追加自定义 pallet 的依赖如:  [dependencies.erc20]
                                   default-features = false
                                   package = 'pallet-erc20'
                                   path = '../pallets/erc20'
                                   version = '2.0.0-r
       并在文件末"[features]"项 std 下追加如：
                        [features]
                        default = ['std']
                        std = [
                              ...
                            'template/std',
                            'erc20/std',
                        ]
    c. 紧接着在" substrate-node-template\runtime\src\lib.rs "为 Runtime
       配置相应实现。
       (仿照已添加 templage 的 Runtime 实现：
                    impl template::Trait for Runtime {
                        type Event = Event;
                    }
       追加自定义 pallet 的 Runtime 实现
                    impl erc20::Trait for Runtime {
                        type Event = Event;
                        type TokenBalance = u64;
                    }

    d. 最后在" substrate-node-template\runtime\src\lib.rs 的构建运行时宏
       模块即: construct_runtime! "内追加如:
            // 将自定义 pallet 引用作用域并向外暴露
            pub use erc20;
            // 将自定义 pallet 追加到" construct_runtime! "宏
            construct_runtime!(
                pub enum Runtime where
                    Block = Block,
                    NodeBlock = opaque::Block,
                    UncheckedExtrinsic = UncheckedExtrinsic
                {
                    // ...
                    TemplateModule: template::{Module, Call, Storage, Event<T>},
                    // 追加自定义 pallet 模块(先将 erc20 模块引入作用域单项)
               		Erc20: erc20::{Module, Call, Storage, Event<T>},
                }
            );
      e. 编辑完毕回退至项目" substrate-node-template "下执行" cargo build --release "

3. 具体编辑 erc20 模块(添加依赖于 erc20/Cargo.toml)
    a. 定义 Trait
        pub trait Trait: system::Trait {
            type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
            // ...
        }
    b. 按需定义结构体及其实现
        pub struct SN { //... }
        impl<T: Trait> Module<T> { //... }
    c. 存储
        decl_storage! { //... }
    d. 事件
        decl_event! { //... }
    e. 错误处理
        decl_error! { //... }
    f. 核心业务
        decl_module! { //... }

 */