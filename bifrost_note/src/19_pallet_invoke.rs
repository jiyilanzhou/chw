
/*
0. 模块间的相互调用
    首先在" ../primitives/src/lib.rs "提供相应接口,后在 Runtime 或本模块的测试模块中指定具体实现

1. 关联的类型" ../primitives/src/lib.rs "
    pub trait AssetTrait<AssetId, AccountId, Balance, Cost, Income> {
        // --snip--
        // 可全局搜索或点击左侧 向下 (实现)箭头查看实现
        fn token_exists(token_symbol: TokenSymbol) -> bool;
        // --snip--
    }
    // 默认实现([自]主用于 develop )
    impl<AssetId, AccountId, Balance, Cost, Income> AssetTrait<AssetId, AccountId, Balance, Cost, Income> for ()
        where AssetId: Default, AccountId: Default, Balance: Default, Cost: Default, Income: Default
    {
        // --snip--
        fn token_exists(_: TokenSymbol) -> bool { Default::default() }
        // --snip--
    }

2. rebate pallet 调用 assets pallet
    a. 在 rebate 模块的" ../brml/rebate/src/lib.rs "声明
         Trait 内定义 type 关联类型
         use node_primitives::{AssetTrait,...};
         pub trait Trait: frame_system::Trait {
             // --snip--
             // 关联外部 Trait : 点击 AssetTrait 可跳转至 primitives/src/lib.rs
             type AssetTrait: AssetTrait<Self::AssetId, Self::AccountId, Self::Balance, Self::Cost, Self::Income>;
             // --snip--
         }
    b. 在 rebate 模块的使用
        use node_primitives::{AssetTrait,...};
        decl_module! {
            pub struct Module<T: Trait> for enum Call where origin: T::Origin {
                fn to_vtoken(
                    origin,
                    vtoken_symbol: TokenSymbol,
                    #[compact] token_amount: T::Balance,
                    referer: Option<T::AccountId>
                ) {
                    // --snip--
                    // 使用(但调用的是哪个具体实现呢[参见"1.c":运行时具体调用])
                    ensure!(T::AssetTrait::token_exists(token_symbol), Error::<T>::TokenNotExist);
                    // --snip--
                }
            }
        }
    // 注：rebate 模块使用 Assets 模块的实现故测试开发应在" .../rebate/Cargo.toml "中引入依赖如
          [dev-dependencies]
          assets = { package = "brml-assets", path = "../assets" }

3. 运行时具体调用(" .../runtime/src/lib.rs ")
    use node_primitives::{AccountIndex, Balance,... }
    impl brml_rebate::Trait for Runtime {
        type Event = Event;
        // --snip--
        // 指定 AssetTrait 的具体实现 : 此处 Assets 源于下述 " construct_runtime!( ... ); "
        // 此处可用" type AssetTrait = (); "因在 node_primitives 已为"()"默认实现 AssetTrait (见"1.b")
        type AssetTrait = Assets;
    }
    construct_runtime!(
            // --snip--
        pub enum Runtime where
            Block = Block,
            NodeBlock = node_primitives::Block,
            UncheckedExtrinsic = UncheckedExtrinsic
        {
            // --snip--
            Assets: brml_assets::{Module, Call, Storage, Event<T>, Config<T>},
            Rebate: brml_rebate::{Module, Call, Storage, Event, Config<T>},
        }
    );

 */