
/*
0. Vec 源码分析
   " Vec::new() "未分配内存空间(即 capacity 为 0 )，可用 with_capacity 指定
   预留空间构造 Vec 亦可对已有的 Vec 调用 reserver 方法扩展预留空间

20.1 内存申请

20.2 内存扩容

20.3 内存释放

20.3.1 Vec 的析构函数(p222)

20.3.2 Drop Check

20.4 不安全的边界

20.5 自定义解引用

20.6 迭代器(p228)

20.7 panic safety

*/