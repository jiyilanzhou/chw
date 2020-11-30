
/*
0. Substrate Node Template :
      Node Template : 麻雀虽小五脏俱全
      (具体可查看"substrate-node-template\pallets\template\src\lib.rs"文件)

1. 模块定义概览(" decl "即 Declare )
    use support::{decl_module, decl_storage, decl_event,...};
    pub trait Trait: system::Trait {...}
    decl_storage! {...}         // 声明 Runtime 存储的部分
    decl_event! {...}           // 声明事件
    decl_error! {...}           // 声明错误
    decl_module! {...}          // 声明用户可调用的 Runtime 模块
    impl<T: Trait> Module<T> {...}      // 当前模块 impl

2. 引入和定义关联类型
    pub trait Trait: system::Trait {
        // 关联类型可先暂且理解为"抽象"类型
        type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
    }
    ...and it inherits from system::Trait:  // 每个 pallet 都会声明一个 Trait
    // From `system` pallet
    pub trait Trait: 'static + Eq + Clone {
        /* 每一个关联类型后的冒号跟随 Trait 约束，定义的关联类型用于与其余 pallet
           产生交互。如定义的 BolckNumber 使其受制于"可加、可减"约束，那么后续在
           封闭整个 Runtime 时可赋于 BlockNumber 具体类型( 如 u32、u64 等满足其
           BlockNumber 冒号后的 Trait 约束即可)，只需在构造 Runtime 时声明其具体
           类型即可。这也是 Substrate 高度模块化体现之一，即编辑每一个模块，比如
           线上拍卖功能需使用到转账，但并不直接调用具体的实现方法(如 balance 模块
           下的 transfer 方法)，此时可声明 balance 关联类型，后续市场可使用货币(
           如美元、人民币、游戏币等[随时替换为具体类型且替换时无需修改当前 pallet
           的任何一行代码即几乎没有替换成本])支付
        */
        type Origin: …
        type Call: …
        type Index: …
        type BlockNumber: ...
    }

3. 定义存储
    decl_storage! {             // 宏源码待理解[?]
        trait Store for Module<T: Trait> as TemplateModule {
            // Here we are declaring a StorageValue, `Something` as an Option<u32>
            // `get(fn something)` defines a getter function
            // Getter called with `Self::thing()`
            Something get(fn something): Option<u32>;   // 用法待理解[?]
            // Here we are declaring a StorageMap `SomeMap` from an AccountId to
            // a Hash.
            // Getter called with `Self::some_map(account_id)`
            SomeMap get(fn some_map): map hasher(identity) T::AccountId => u32;
        }
    }

4. 定义事件
    decl_event!(
        pub enum Event<T> where AccountId = <T as system::Trait>::AccountId {
            /// Event `SomethingStored` is declared with a parameter of the type `u32`
            and `AccountId`
            SomethingStored(u32, AccountId),
        }
    );

5. 定义可调用函数 ： decl_module! 可理解为用户编写的 function
    decl_module! {
        pub struct Module<T: Trait> for enum Call where origin: T::Origin {
            fn deposit_event<T>() = default; // The default deposit_event definition
            pub fn do_something(origin, something: u32) -> Result {
                let sender = ensure_signed(origin)?; // Check for transaction
                <Something::put(something); // Put a value into a StorageValue
                Self::deposit_event(RawEvent::SomethingStored(something, who)); // Emit Event
                Ok(()) // Return Ok at the end of a function
            }
        }
    }

6. 定义公共和私有函数
    impl<T: Trait> Module<T> {
        fn mint(to: T::AccountId, id: T::Hash) -> Result {...}
        fn transfer(from: T::AccountId, to: T::AccountId, id: T::Hash) -> Result {
             ...
        }
    }
    // 如果定义为' pub '，其它模块亦可调用

 */
