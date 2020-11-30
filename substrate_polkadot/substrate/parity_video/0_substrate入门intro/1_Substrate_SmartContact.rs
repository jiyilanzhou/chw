
/*
0. Substrate VS Smart contact (自定义链上运行时与智能合约的区别)
   Substrate 能干的所有事情 Smart  contact 都可以做，如资产 ERC2.0、
   ERC7.1 及治理等。且智能合约开发门槛较低(如 python、solidity 较为
   容易入门的开发语言),那为何还需要 Substrate 来定义自身的链上运行时
   呢？(其原因之一即是 Substrate 提供更多功能如 frame\recovery 模块
   提供的可恢复功能)

1. " substrate\frame\recovery" 模块：
    a. recovery 言下之意即是在一定条件下可恢复。假设账户 A 不小心弄丢
       私钥，对于传统区块链则无法还原在相应地址上的资产而 Substrate 的
       " \frame\recovery\ "库在一定条件下则可或恢复使用新账户替代。其
       具体实现：比如 A 可请求组内 B、C、D 成员为其作证，通过后可调用
       " substrate\frame\recovery\src\lib.rs "内的" as_recovered "
       方法执行恢复或将原账户 A 的所有资产及权限转移到新开的账户。
    b. 假设链不支持可恢复功能，仍然采用传统密码学层面(如私钥分片多备份)
       的方式(弄丢的话可通过助记词拼接等找回)则是有风险的而且也还是一个
       比较中心化的操作。但如若链上原生就支持 Substrate 的可恢复功能则
       无论针对链还是用户都是非常有益的事情。

2. Substrate 可自定义链上运行时来进行底层操作而智能合约无法完成。
   如" substrate\frame\transaction-payment "模块可自定义使用货币种类
   及 Token 类型进行支付

 */