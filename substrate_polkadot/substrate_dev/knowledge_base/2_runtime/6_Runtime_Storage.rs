
/*
0. Runtime Storage  运行时存储

1. Storage Items 存储项

2. Storage Value 存储值

3. Storage Maps 存储 Map

4. Hashing Algorithms 散列算法
    a. Cryptographic Hashing Algorithms 密码散列算法
    b. Transparent Hashing Algorithms 透明哈希算法

5. Declaring Storage Items  声明存储项
    a. Visibility       能见度
    b. Getter Methods   获取方法
    c. Default Values   默认值
    d. Genesis Configuration 创世配置
    // 注: 宏的 add_extra_genesis 扩展 decl_storage 允许定义一个范围， 可以在其中提供 config 和 build
           扩展，而无需将它们绑定到特定的存储项目

6. Accessing Storage Items 访问存储项
    用 Substrate 构建的区块链公开了一个远程过程调用（RPC）服务器，该服务器可用于查询区块链的运行时存储

7. Best Practices 最佳实践
    a. What to Store  存储什么
    b. Verify First, Write Last  首先验证，最后写入
    c. Create Bounds  创建边界



 */