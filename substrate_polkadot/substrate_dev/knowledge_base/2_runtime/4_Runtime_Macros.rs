
/*
0. Runtime Macros 运行时宏
   对每个模块实时查看相应的" API "文档

1. Introduction 介绍
    优化运行时开发，其意图是利用宏中的简洁语句来声明其意图

2. Macro Basics 宏基础
    大多数 Substrate 运行时宏都是使用声明性宏或类似于函数的宏定义的

3. Substrate Runtime Macros 运行时宏
    开发时使用的一些关键宏

4. decl_storage! 存储宏
    a. decl_storage! 的简要说明
       #vis #name get(fn #getter) config(#field_name) build(#closure): #type = #default;
       (0). 宏声明为 #name 定义的新结构体类型并实现 Storage Trait
       (1). 宏声明实现 Store Trait 以设置 pallet 具有存储空间，并由 decl_module! 内定义的 Module 结构体实现指定
            的 #getter 函数( Module 实现包含每个存储项的 getter 函数和元数据)
    b. 存储项定义包括(每个存储项都是以 #name 命名的结构体 struct )
        (0). 数据类型
            for StorageValue: rust-type
            for StorageMap: map hasher($hasher) rust_type => rust_type
            for StorageDoubleMap: doublemap hasher($hasher) rust_type,
                                            hasher($hasher) rust_type => rust_type
        (1). 定义的函数功能( its getter function )
        (2). 其键类型及其哈希方法( its key types and their hashing methods [if a map or double-map])
        (3). 存储名称( the name of the storage )
        (4). 默认值( its default value )
    c. 声明和实现 Store Trait
            // Store trait 为每个"存储名称( #name )"声明，因此每个存储名称成为 Store trait 内的关联类型如：
            trait Store {
                type #name;
                type #name1;
                ...
            }
    d. decl_storage! {
              trait Storage for Module<T: Trait> as MyModule {
                // ...
              }
              // 这些存储值可在 genesis 块中通过 add_extra_genesis 块初始化
              add_extra_genesis {
                build (|config| {
                  //...
                });
              }
        }

5. decl_event! 事件宏
    a. 定义运行时 pallet 触发的事件
    b. RawEvent<...all generic types used> 枚举类型定义为将宏中指定的所有事件作为其枚举变量。

6. decl_error! 错误宏
    a. 定义托盘可能在其可调度功能中返回的错误类型
    b. Error类型实现包含一个fn metadata()函数

7. decl_module! 模块宏
    a. 在托盘中定义可调度功能
    b. 包含托盘的 Module 结构和 Call 枚举类型。它使用用户定义的可调度调用将必要的逻辑组合到两种类型中

8. Construct_runtime!
    a. 构造 Substrate 运行时并将各种 pallet 集成到运行时
    b. Runtime 定义了 struct 类型以表示 Substrate 运行时

9. parameter_types!
    在运行时构造期间声明要分配给托盘可配置特征关联类型的参数类型

10. impl_runtime_apis!
    通过RuntimeApi and RuntimeApiImpl struct 类型为客户端生成 API 实现

11. app_crypto!
    指定要由货盘管理的加密密钥对及其签名算法

12. impl_outer_origin！
    为 Origin 运行时构造结构类型

13. impl_outer_event！
    为Event运行时构造结构类型

14. impl_outer_dispatch！
    实现元分派模块以分派给其它分派器

 */