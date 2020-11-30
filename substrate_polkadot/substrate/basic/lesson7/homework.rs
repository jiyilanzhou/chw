
/*
第七课作业：
第一题：列出3个常用的宏、3个常用的存储数据结构；
    a. 常用宏：
        decl_storage!、decl_event!、decl_error!、decl_module!、construct_runtime!
    b. 常用存储数据结构：
        (0). Rust 原生数据类型及衍生(单值类型)：
            数值、布尔、枚举、结构体、集合、定点小数、定长哈希、大整数、内置自定义类型等
            如" u32、U256、U512、Percent、Permill、Perbill、Monment、AccountId "等
        (1). 原生类型构成的映射类型：
            简单映射：map 键值对(单值类型皆可用作 key 或 value )
                     如" hasher: blake2_128_concat, twox_64_concat, identity "
            双键映射: double_map 类型(使用两个 key 来索引 value )
    第二题：实现存证模块的功能，包括：
        创建存证
        撤销存证

    第三题：为存证模块添加新的功能，
        转移存证，接收两个参数，一个是内容的哈希值，另一个是存证的接收账户地址。

参考作业 ：" https://github.com/chw683/substrate-node-template  "


*/