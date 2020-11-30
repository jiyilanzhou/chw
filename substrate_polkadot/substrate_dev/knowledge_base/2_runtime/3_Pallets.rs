
/*
0. Pallets 货盘(调色板,托盘)

1. What is a Pallet 调色板定义
    每个货盘都有自己的离散逻辑，这些逻辑可以修改区块链状态转换功能的特征和功能

2. Skeleton of a Pallet 托盘骨架
    一个 Substrate pallet 由 5 个主要部分组成
    // 1. Imports and Dependencies
    // The pallet supports the use of any Rust library which compiles
    // with the `no_std` flag.
    use support::{decl_module, decl_event, decl_storage, ...}

    // 2. Runtime Configuration Trait
    // All of the runtime types and consts go in here. If the pallet
    // is dependent on specific other pallets, then their configuration traits
    // should be added to the inherited traits list.
    pub trait Trait: system::Trait { ... }

    // 3. Runtime Events
    // Events are a simple means of reporting specific conditions and circumstances
    // that have happened that users, Dapps and/or chain explorers would find
    // interesting and otherwise difficult to detect.
    decl_event!{ ... }

    // 4. Runtime Storage
    // This allows for type-safe usage of the Substrate storage database, so you can
    // keep things around between blocks.
    decl_storage! { ... }

    // 5. The Pallet Declaration
    // This defines the `Module` struct that is ultimately exported from this pallet.
    // It defines the callable functions that this pallet exposes and orchestrates
    // actions this pallet takes throughout block execution.
    decl_module! { ... }

3. Example Module 示例模块(这是一个最小的工作托盘，它仅允许用户存储 u64 值)

    use support::{decl_module, decl_event, decl_storage, StorageValue, StorageMap};
    use system::ensure_signed;

    pub trait Trait: system::Trait {
        // The traits the `Event` type used in this pallet has.
        type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
    }

    decl_event!{
        pub enum Event<T> where
            AccountId = <T as system::Trait>::AccountId,
        {
            // An event which is emitted when `set_value` is called.
            // Contains information about the user who called the function
            // and the value they called with.
            ValueSet(AccountId, u64),
        }
    }

    decl_storage! {
        trait Store for Module<T: Trait> as Example {
            // The last value passed to `set_value`.
            // Used as an example of a `StorageValue`.
            pub LastValue get(fn last_value): u64;
            // The value each user has put into `set_value`.
            // Used as an example of a `StorageMap`.
            pub UserValue get(fn user_value): map T::AccountId => u64;
        }
    }

    decl_module! {
        pub struct Module<T: Trait> for enum Call where origin: T::Origin {
            // A default function for depositing events in our runtime
            fn deposit_event() = default;

            // The only callable function of our runtime module.
            // Allows a user to set a `u64` value into the runtime storage.
            pub fn set_value(origin, value: u64) {
                let sender = ensure_signed(origin)?;
                LastValue::put(value);
                UserValue::<T>::insert(&sender, value);
                Self::deposit_event(RawEvent::ValueSet(sender, value));
            }
        }
    }








 */