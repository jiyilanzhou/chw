
/*
0. Substarte 中的存储
    Substrate 模块可通过 decl_storage! 宏来声明数据存储

1. 存储类型 ： 基本存储项由名称和类型组成，常见存储类型：
    a. Value，单个值。
    b. Map，key-value的hash map。
    c. Linked Map，与Map相似，但是允许枚举存储的元素。
    d. Double Map，具有两个keys的map实现

2. 定义 Storage
    a. 可使用宏 decl_storage! 创建新 runtime 存储项
    b. 声明每种类型的存储项的示例
       decl_storage! {
            trait Store for Module<T: Trait> as Example {
                pub SomeValue: u64;
                pub SomeMap: map u64 => u64;
                pub SomeLinkedMap: linked_map u64 => u64;
                pub SomeDoubleMap: double_map u64, blake2_256(u64) => u64;
            }
        }
    c. 存储项的完整定义格式
       #vis #name get(fn #getter) config(#field_name) build(#closure): #type = #default;
            #vis：设置结构体的可见性。pub或不设置。
            #name：存储项的名称，用作存储中的前缀。
            get(fn #getter)：可选， Module 实现名称为 #getter 的函数。
            config(#field_name)：可选，定义可在GenesisConfig中包含该存储项。如果设置 get 则 field_name 为可选
            build(#closure)：可选，定义闭包，调用它可以覆盖存储项的值。
            #type：存储项的类型。
            #default：定义默认值，即无值时的返回值。

3. 定义示例及解读
        decl_storage! {
            trait Store for Module<T: Trait> as Example {
                Foo get(fn foo) config(): u32=12;
                Bar: map u32 => u32;
                pub Zed build(|config| vec![(0, 0)]): linked_map u32 => u32;
            }
        }
    a. 使用(pub) trait Store for Module<T: Trait> as Example 定义 Store trait，用来关联每一存储项到 Moudle,
        Example 称为模块前缀，必须唯一。as Example 用来设置模块的每个存储项前缀。
    b. 值类型：Foo，实现 StorageValue trait
        Twox128(module_prefix) ++ Twox128(storage_prefix)
    c. Map 类型：Bar，实现 StorageMap trait
        twox128(module_prefix) ++ twox128(storage_prefix) ++ hasher(encode(key))
    d. LinkedMap 类型：Zed，实现 StorageLinkedMap trait
    e. 默认值
       Substrate 允许定义未设置存储项时返回的默认值。该值实际并未存储在 runtime 存储中但 runtime 逻辑在执行期间可
       获取该值如" 为 map 中的所有项设置默认值 ":
       decl_storage! {
            trait Store for Module<T: Trait> as Example {
                pub SomeMap: map u64 => u64 = 1337;
            }
        }
    f. 访问 Storage : 可通过多种方式访问
        (0). 结构：Foo 或 Foo::<T> 取决于值类型是否通用
        (1). Storage Trait 结构：<Module <T> as Store>::Foo
        (2). 调用模块的 getter 进入结构：Module::<T>::foo()

4. Runtime Storage API
    a. Substrate 的 Support 模块提供了实用性工具，其可为 runtime 模块的存储项生成唯一确定的 keys。这些存储项放置在
       状态 trie 中，可通过 key 查询 trie 来访问
    b. 存储 API 中定义了 5 个 trait：
        StorageValue，单个值
        StorageMap，key-value 的 hash map。
        StorageLinkedMap，与 StorageMap 相似，但是允许枚举存储的元素。
        StorageDoubleMap，有两个 keys 的 map 实现。
        StoragePrefixedMap，用于唯一前缀存储所有值的 map

5. 具体示例 1 ：在区块链中创建一个存储值
   a. 框架
        use support::{decl_storage, decl_module};
        decl_storage! {
            trait Store for Module<T: Trait> as VerifiableCreds {
                // Declare storage and getter functions here
                // 存储
            }
        }

        decl_module! {
            pub struct Module<T: Trait> for enum Call where origin: T::Origin {
                // Declare public functions here
                // 公有函数:主用于区块链外部调用
            }
        }

   b. 具体存储：
        use support::{decl_storage, decl_module, StorageValue};
        decl_storage! {
            trait Store for Module<T: Trait> as VerifiableCreds {
                // 定义 u32 类型的值(需导入"support::StorageValue")：SubjectCount
                // 赋值于 SubjectCount 可用" <SubjectCount<T>>::put(x); "
                // 取值可用" let subject_count = <SubjectCount<T>>::get(); "
                SubjectCount: u32;
                // 存储映射 (Storage Map) ：使用 map 结构需导入 support::StorageMap 类型
                // 插入" <SomeValue<T>>::insert(key, value); "
                // 获取" let my_value = <SomeValue<T>>::get(key);"
                Subjects: map u32 => T::AccountId;
            }
        }

        decl_module! {
            pub struct Module<T: Trait> for enum Call where origin: T::Origin {
                fn set_value(origin, value: u32) -> Result {
                    let sender = ensure_signed(origin)?;
                    <SubjectCount<T>>::put(value);
                    Ok(())
                }
                fn create_subject(origin) -> Result {
                    let sender = ensure_signed(origin)?;
                    let sc = <SubjectCount<T>>::get();
                    // 更新
                    <SubjectCount<T>>::put(sc + 1);
                    <Subjects<T>>::insert(sc, sender);
                    Ok(())
                }
            }
        }

    c. 公开函数的首个参数永远都是 origin
        decl_moudle!{
            pub struct Module<T: Trait> for enum Call where origin: T::Origin {
                // 首项 origin，之后为参数，返回值为 Result
                fn foo(origin, bar: Bar, baz: Baz, ...) -> Result;
            }
        }
        // origin 包含这个函数调用者的信息。有三种可能性：
            (0). 公有的调用：由外界账户签名
            (1). 根调用（root call）：由 governance 治理系统调用
            (2). Inherent 调用：block authors（区块的提出者）和验证者调用
            注：可用 ensure_signed, ensure_root, ensure_inherent 对 origin 的发起者进行验证
       // Result : 执行成功的函数会返回OK(()) 否则程序会返回 Err()

6. 具体示例 2 ：
        use support::{decl_storage, decl_module, StorageValue};
        decl_storage! {
            trait Store for Module<T: Trait> as Example {
                // Your storage items

                Foo get(fn foo) config(): u32=12;

                /*  Foo: map hasher($hash) type => type;
                    其通过 StorageMap generator 实现" StorageMap、StoragePrefixedMap "Trait
                    $hash 代表 Hashable 特征中可用的哈希算法选择 。通常使用以下三个哈希之一:
                    (0). blake2_128_concat：默认的安全选择。若不确定或不在乎可使用
                    (1). twox_64_concat：不安全的哈希器，只有当不信任用户无法随意选择原像时才能安全使用
                    (2). identity：不是一个哈希器而是直接使用密钥材料(由于不进行散列或追加故最快但也是最不安全的)
                */
                Bar: map hasher(identity) u32 => u32;
                pub Zed build(|config| vec![(0, 0)]): map hasher(identity) u32 => u32;
            }

            /* GenesisConfig：
                  GenesisConfig 可定义一个用于存储初始化的可选结构，当至少一个存储字段需要默认初始化( get 和 config 或
                  build)时或可特别定义为：
             */
             add_extra_genesis {
                 config(genesis_field): GenesisFieldType;
                 config(genesis_field2): GenesisFieldType;
                 // ...
                 build(|_: &Self| {
                      // Modification of storage
                 })
            }
        }

 */