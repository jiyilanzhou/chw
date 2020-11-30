
/*
0. 在相应 pallet 内的 lib.rs 内定义 WeightInfo trait
   a. 如在" bifrost_poe/brml/poe/src/lib.rs "
       pub trait WeightInfo {
            // 权重函数名：建议功能函数名一致(见文知义)
            fn create_claim() -> Weight;
            fn revoke_claim() -> Weight;
            fn transfer_claim() -> Weight;
        }
        // 注：返回类型 Weight 源于 frame_support::weights::Weight
           故需于 poe/lib.rs 下追加如" use frame_support::{ ... weights::Weight ... }; "
   b. 为模块 Trait 增加关联类型
       pub trait Trait: frame_system::Trait {
            type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
            /// Set default weight
            type WeightInfo : WeightInfo;
       }
   c. 在 decl_module! 内的函数上方调用如：
       #[weight = T::WeightInfo::create_claim()]
       pub fn create_claim(origin, claim: Vec<u8>) -> dispatch::DispatchResult{
            // ...
       }

1. WeightInfo Trait 具体实现（注:" pallet "下的相应" default_weights "文件作用为【待理解】）
    a. 在 ../runtime/src/weights 下定义相应文件(如" pallet_poe.rs ")并在同目录下 mod.rs 中暴露(外部可调用)
        use frame_support::{traits::Get, weights::Weight};
        use sp_std::marker::PhantomData;
        pub struct WeightInfo<T>(PhantomData<T>);
        // 注" brml_poe "即是定义的 pallet 项目名即 poe 项目下 Cargo.toml 内" [package] 下的 name "
        impl<T: frame_system::Trait> brml_poe::WeightInfo for WeightInfo<T> {
            fn create_claim() -> Weight {
                (65949000 as Weight)
                    .saturating_add(T::DbWeight::get().reads(1 as Weight))
                    .saturating_add(T::DbWeight::get().writes(1 as Weight))
            }
            fn revoke_claim() -> Weight {
                (46665000 as Weight)
                    .saturating_add(T::DbWeight::get().reads(1 as Weight))
                    .saturating_add(T::DbWeight::get().writes(1 as Weight))
            }
            fn transfer_claim() -> Weight {
                (27086000 as Weight)
                    .saturating_add(T::DbWeight::get().reads(1 as Weight))
                    .saturating_add(T::DbWeight::get().writes(1 as Weight))
            }
        }

   b.  在" bin/node/runtime/src/lib.rs "为相应 pallet Trait 的 Runtime 实现添加相应的 WeightInof 关联类型
       (源于" 0.b "[因 pallet Trait 已增加相应关联类型，故实现其 Trait 则必须定义相应关联类型])如：
        impl brml_poe::Trait for Runtime {
            type Event = Event;
            // 调用 WeightInfo 的具体实现
            type WeightInfo = weights::pallet_poe::WeightInfo<Runtime>;
        }

2. 修改 biforst 项目与 substrate 不同的地方
    a. bifrost 增加 pallet_assets.rs 文件的 WeightInfo 实现
       substart 使用 #[weight = 0] 实现
    b. 修改 bifrost 的相应模块
        (0). assets
        (1). bridge-eos
        (2). bridge-iost
        (3). convert
        (4). oracle
        (5). proxy-validator
        (6). swap
        (7). voucher

3. 在远程创建创建 .yml 文件
   打开工程" https://github.com/bifrost-finance/bifrost " -> 切换到自己分支 -> Actions

 */