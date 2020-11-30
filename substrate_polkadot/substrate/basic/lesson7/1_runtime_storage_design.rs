/*
0. Substrate 存储
    a. 区块链存储的不同点和约束
    b. 存储单元的类型
    c. 存储的初始化

1. 区块链存储不同点及约束
    a. 区块链常用特性
       (0). 开源可审查，对等节点，引入延迟和随机来达到共识
       (1). 链式、增量地存储数据
    b. 区块链应用结点依赖高效键值对数据库
       (0). LevelDB
       (1). RocksDB ：parity substrate 依赖的数据库
    c. 区块链存储约束与限制
       (0). 大文件直接存储在链上的成本很高
       (1). 链式的区块存储结构不利于对历史数据的索引
       (2). 进行数值运算时不能使用浮点数

2. Substrate 存储单元的类型
    a. Rust 原生数据类型的子集(定义于核心库及 alloc 库)
       (0). 单值类型 : 如" 数值、布尔、枚举、结构体... "
            数值：u8,i8,u32,i32,u64,i64,u128,i128
            大整数：U128,U256,U512
            布尔：bool
            集合：Vec<T>, BTreeMap, BTreeSet
            定点小数：Percent,Permill,Perbill
            定长哈希：H128,H256,H512
            复杂类型：Option<T>,tuple,enum,struct
            内置自定义类型：Moment,AccountId
       (1). 原生类型构成的映射类型
            如" 简单映射 : map "及" 双键映射 : dublemap(两个 key 对应一个 value )"
    b. 示例：
        decl_storage! {
            trait Store for Module<T:  Trait> as TemplateModule {
                // 存储类型为" 单值类型 " : Option<u32>
                Something get(fn something):  Option<u32>;
            }
        }
    c. 宏展开
        trait Store {
            type Something;
        }
        impl<T: Trait + 'static> Store for Module<T> {
            type Something = Something;
        }
        impl<T: Trait + 'static> Module<T> {
            pub fn something() -> Option<u32> {
                < Something < > as self :: sp_api_hidden_includes_decl_storage :: hidden_include :: storage :: StorageValue < u32 > > :: get ( )
            }
        }

3. 单值类型
    a. 数值类型定义
        decl_storage! {
            trait Store for Module<T: Trait> as DataTypeModule {
                 // store unsigned integer, init to zero if not set
                 // 设置默认值为 10 (若未设置则默认为 0 )
                 MyUnsignedNumber get(fn unsigned_number): u8 = 10;
                 // also init to zero, can store negative number
                 MySignedNumber get(fn signed_number): i8;
            }
        }
    b. 数值类型 u8,i8,u32,i32,u64,i64,u128,i128 的使用：
           增：MyUnsignedNumber::put(number);
           查：MyUnsignedNumber::get();
           改：MyUnsignedNumber::mutate(|v| v + 1);
           删：MyUnsignedNumber::kill();
       // 参阅" frame_support::storage::StorageValue "
    c. 数值类型 u8,i8,u32,i32,u64,i64,u128,i128 的安全操作：
        (0). 返回 Result 类型：checked_add, checked_sub, checked_mul, checked_div
              // fail the transaction if error
              my_unsigned_num.checked_add(10)?;
        (1). 溢出返回饱和值：saturating_add,saturating_sub,saturating_mul
              // result is 255 for u8
              my_unsigned_num.saturating_add(10000);
    d. 大整数 U256,U512 类型定义：
        use sp_core::U256;
        decl_storage! {
            trait Store for Module<T: Trait> as DataTypeModule {
                 // init to 0
                MyBigInteger get(fn my_big_integer): U256;
            }
        }
        // 安全操作：checked_add,overflowing_mul... (更多 API 参考" sp_core::U256 "文档)
    e. bool 类型定义：
        decl_storage! {
            trait Store for Module<T: Trait> as DataTypeModule {
                 // init to false, store boolean value
                MyBool get(fn my_bool): bool;
            }
        }
    f. Vec<T> 类型定义：
        use sp_std::prelude::*;       // 引入 Vec 定义的路径
        decl_storage! {
            trait Store for Module<T: Trait> as DataTypeModule {
                // default to 0x00
                MyString get(fn my_string): Vec<u8>;
            }
        }
        // 操作：push,pop,iter... Vec 结构体
    g. Percent,Permill,Perbill (" 百/百万/十亿 "分之几)定点类型定义：
        (0). 定义( API 文档" sp_arithmetic::Permill ")
            use sr_primitives::Permill;
            decl_storage! {
                trait Store for Module<T: Trait> as DataTypeModule {
                    // fix point number, default to 0
                    MyPermill get(fn my_permill): Permill;
                }
            }
        (1). Percent,Permill,Perbill 定点类型操作
             构造 :
                 // 指定分子构造(百分之)
                 Permill::from_percent(value);
                 // 指定分子构造(百万分之)
                 Permill::from_parts(value);
                 // 指定分子分母构造
                 Permill::from_rational_approximation(p,q);
             计算 :
                 permill_one.saturating_mul(permill_two);
                 my_permill * 20000 as u32
    h. Moment 时间类型定义
        // Moment 源于 timestamp 模块
        pub trait Trait: system::Trait + timestamp::Trait {}
        decl_storage! {
            trait Store for Module<T: Trait> as DataTypeModule {
                 // Moment is type alias of u64
                MyTime get(fn my_time): T::Moment;
            }
        }
        // 获取链上时间：<timestamp::Module<T>>::get();
    i. AccountId 账户类型定义 : // AccountId 源于 system 模块
        decl_storage! {
            trait Store for Module<T: Trait> as DataTypeModule {
                MyAccountId get(fn my_account_id): T::AccountId;
            }
        }
        // 获取 AccountId: let sender = ensure_signed(origin)?;
    j. struct 类型定义( enum 类型定义类似但需自定义实现 Default 接口)
        #[derive(Clone, Encode, Decode, Eq, PartialEq, Debug, Default)]
        pub struct People {
            name: Vec<u8>,
            age: u8,
        }
        decl_storage! {
            trait Store for Module<T: Trait> as DataTypeModule {
                MyStruct get(fn my_struct): People;
            }
        }
    i. enum 类型定义
        #[derive(Copy, Clone, Encode, Decode, Eq, PartialEq, Debug)]
        pub enum Weekday {
            Monday,
            Tuesday,
            Wednesday,
            Other,
        }
        // 需要手动实现 Default trait
        impl Default for Weekday {
            fn default() -> Self {
                Weekday::Monday
            }
        }
        // 参阅" https://github.com/kaichaosun/play-substrate/blob/master/pallets/datatype/src/lib.rs#L32 "
           (内容已拷贝至" reference_code/0_lib.rs ")

4. 简单映射类型定义
    a. map 类型用于保存键值对，单值类型都可以用作 key 或 value
        decl_storage! {
            trait Store for Module<T: Trait> as DataTypeModule {
                // map 键为 u8 值为 Vec<u8> ,且需显式指明针对 key 的 hash 函数生成底层 tree
                MyMap get(fn my_map): map hasher(twox_64_concat) u8 => Vec<u8>;
            }
        }
        /* hasher 算法 : blake2_128_concat(加密安全的哈希算法), twox_64_concat, identity
           当输入源不可信时使用 blake2_128_concat (如用户输入)；当输入源可信时(如一些自增的
           有界 index )可使用非密码安全的哈希算法 twox_64_concat ；当本身输入就是密码安全的
           哈希时可使用 identity (避免二次哈希)
        */
    b. map 基本操作方法
        插入一个元素 ：MyMap::insert(key, value)；
        通过 key 获取 value ： MyMap::get(key);
        删除某个 key 对应的元素 ： MyMap::remove(key);
        覆盖或修改某个 key 对应的元素 ：
            MyMap::insert(key, new_value);
            MyMap::mutate(key, |old_value| old_value+1);
        // API 文档" Trait frame_support::storage::StorageMap "
                   " Trait frame_support::storage::IterableStorageMap "

5. 双键映射类型
    a. double_map 类型,使用两个 key 来索引 value，用于快速删除 key1 对应的任意记录亦可用于
       遍历 key1 对应的所有记录
         decl_storage! {
             trait Store for Module<T: Trait> as DataTypeModule {
                 // 针对 Key 需分别指定作用于 key1、key2 的 hasher 函数
                 MyDoubleMap get(fn my_double_map):
                     double_map hasher (blake2_128_concat) T::AccountId,
                                hasher(blake2_128_concat) u32 => Vec<u8>;
             }
         }
        // 然亦可针对可确信输入来源的 key1 或 key2 分别使用 hasher(twox_64_concat) 效率高些
    b. 双键映射类型操作
        插入一个元素 ：MyDoubleMap::<T>::insert(key1, key2, value);
        获取某一元素 ：MyDoubleMap::<T>::get(key1, key2);
        删除某一元素 ：MyDoubleMap::<T>::remove(key1, key2);
        删除 key1 对应的所有元素 ：MyDoubleMap::<T>::remove_prefix(key1);
        // API文档 " frame_support::storage::StorageDoubleMap "
                  " Trait frame_support::storage::IterableStorageDoubleMap "

6. 存储的初始化
    a. 创世区块的数据初始化三种方式：
        config()
        build(clousure)
        add_extra_genesis { … }
    b. 存储初始化演示
        (0). 源于" .../pallets/genesis-config/src/lib.rs "
            // This pallet's storage items.
            decl_storage! {
                // It is important to update your storage name so that your pallet's
                // storage items are isolated from other pallets.
                // ---------------------------------vvvvvvvvvvvvvv
                trait Store for Module<T: Trait> as GenesisConfigModule {
                    // genesis storage with config()
                    Something get(fn something) config(): Option<u32>;

                    SomethingTwo get(fn something_two) build(|config: &GenesisConfig<T>| {
                        Some(config.something_two + 1)
                    }): Option<u32>;
                    SomethingMap get(fn something_map): map hasher(blake2_128_concat) T::AccountId => u32;
                }
                add_extra_genesis {
                    config(something_two): u32;
                    config(some_account_value): Vec<(T::AccountId, u32)>;
                    build(|config: &GenesisConfig<T>| {
                        for (who, value) in config.some_account_value.iter() {
                            SomethingMap::<T>::insert(who, value);
                        }
                    })
                }
            }
        (1). // 对应 " .../node/src/chain_spec.rs "
                fn testnet_genesis(
                    wasm_binary: &[u8],
                    initial_authorities: Vec<(AuraId, GrandpaId)>,
                    root_key: AccountId,
                    endowed_accounts: Vec<AccountId>,
                    _enable_println: bool,
                ) -> GenesisConfig {
                    GenesisConfig {
                         // ...
                         genesis_config: Some(GenesisConfigModuleConfig {
                             something: 30,
                             something_two: 60,
                             some_account_value: endowed_accounts.iter().cloned().map(|k| (k, 1 << 6)).collect(),
                         })
                    }
                }
        参阅" https://github.com/kaichaosun/play-substrate/blob/master/pallets/genesis-config/src/lib.rs "
             (内容已拷贝至" reference_code/1_lib.rs ")

7. 最佳实践
    a. 最小化链上存储空间
        哈希值 : 针对大文件、视频记录其哈希值(而非整个文件大小)
        设置列表容量 : 当使用集合类型或结构体时，设置结构体大小及列表容量
    b. Verify First Write Last
        链上存储始终遵循" "

8. Other Tips
    可通过 pub 关键字设置存储单元针对其它模块的可见范围
    可自定义设置默认值如: MyUnsignedNumber get(fn unsigned_number): u8 = 10;
    在 frame 目录下查找对应的最新用法
    decl_storage 宏的说明文档


*/