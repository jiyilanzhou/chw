
/*
0. 相应网站
    https://etherscan.io/tx/0x18b69adc661d212e18f1770b23f86ed1e1c5c288b5b837d625742debd486d6bc
    https://bifrost.subscan.io/extrinsic
    https://bifrost.subscan.io/tools/ss58_transform
    https://bifrost.subscan.io/runtime

1. 密码学
    密钥 -> private key -> pubkey ->(导出)通过某种算法-> 地址
    无论文件大小 -> Hash -> 可设置为具体长度的地址

2. Validator
   出块有奖励 -> 出块结点 (属于某个作者：抵押大量币成为结点) -> 其内部可设置奖励归属人的地址
   即使空交易亦出场：随之而来的区块头

3. Wasm / Native
   升级时只需更新编译好的 Wasm  —> 结点通过共识确定升级 -> 其余结点再次执行相应业务则对比 Native 及 线上 Wasm ->
   Native 同步线上 Wasm -> 执行本地 Native (效率高速度快)操作业务

4. 钱包
    钱包只是管理密钥的一个工具

5. IPO
    首次公开募股（Initial Public Offering）是指一家企业第一次将它的股份向公众出售。
    如波卡平行链卡槽拍卖：竞标的某个项目没有足够的币抵押，

6. ss58 与 base58 的区别
   谷歌搜索 ss58 address
   https://github.com/paritytech/substrate/wiki/External-Address-Format-(SS58)

 */