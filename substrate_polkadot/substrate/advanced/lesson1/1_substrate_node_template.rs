// emit[iˈmɪt]v.发射,发出,发行
// spec[spek]n.投机,规则          // specific[spəˈsɪfɪk]adj&n.特殊/定(的),具体
/*
0. Substrate Node Template
    Substrate Node Template 是 Substrate 最小的一个高度抽取可用 Demo

1. 模块定义概览
    use support::{decl_module, decl_storage, decl_event,...};
    pub trait Trait: system::Trait {...}
    decl_storage! {...}
    decl_event! {...}
    decl_error! {...}
    decl_module! {...}                       // 定义用户可调用的功能
    impl<T: Trait> Module<T> {...}           // 为当前模块或其它模块提供的功能函数

2. 引入和定义关联类型
    # it inherits from system::Trait:
    pub trait Trait: system::Trait {
        // 在 Runtime 中再声明满足约束的具体类型(亦是 substrate 高度模块化之一体现)
        type Event: From<Event<Self>> + Into<<Self  as system::Trait>::Event>;
        type Call: …
        type Index: …
        type BlockNumber: ...
    }
    // 定义的关联类型用于与其它模块交互

3. 定义存储
    decl_storage! {
        trait Store for Module<T: Trait> as TemplateModule {
            // Here we are declaring a StorageValue, `Something` as an Option<u32>
            // `get(fn something)` defines a getter function
            // Getter called with `Self::something()`
            Something get(fn something): Option<u32>;
            // Here we are declaring a StorageMap `SomeMap` from an AccountId to a Hash.
            // Getter called with `Self::some_map(account_id)`
            SomeMap get(fn some_map): map hasher(identity) T::AccountId => u32;
        }
    }

4. 定义事件: 使用" () / {} "均可(是否其它的声明宏也是一样呢[???])
    decl_event!(
      pub enum Event<T> where AccountId = <T as system::Trait>::AccountId {
          /// Event `SomethingStored` is declared with a parameter of the type `u32` and `AccountId`
          SomethingStored(u32, AccountId),
      }
   );

5. 定义可调用函数
    decl_module! {
        pub struct Module<T: Trait> for enum Call where origin: T::Origin {
            fn deposit_event<T>() = default; // The default deposit_event definition
            pub fn do_something(origin, something: u32) -> Result {
                let sender = ensure_signed(origin)?;    // Check for transaction
                <Something::put(something);     // Put a value into a StorageValue
                Self::deposit_event(RawEvent::SomethingStored(something, who));     // Emit Event
                Ok(())          // Return Ok at the end of a function
           }
        }
     }

6. 定义公共和私有函数
    impl<T: Trait> Module<T> {
        fn mint(to: T::AccountId, id: T::Hash) -> Result {...}
        fn transfer(from: T::AccountId, to: T::AccountId, id: T::Hash) -> Result { ... }
    }
    // 若定义为 pub 则其它模块亦可调用









 */