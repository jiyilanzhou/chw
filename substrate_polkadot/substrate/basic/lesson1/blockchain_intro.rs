
// nonce[nɒns]n.(目前)新鲜值  // Nonce 最新值(区块链中常用作"随机数")
// trap[træp]n.陷阱   // trapdoor[ˈtræpdɔː(r)]n.活板门
/*
0. 什么是区块链
    a. 传统的记账方式
        所有的账本都存在银行的中心化服务器
    b. 区块链的记账方式
        每个人都可以保存一一份账本
        伴随的问题

1. 伴随的问题(图"0_区块链记账伴随的问题.png")
    a. 谁可以向账本里写入内容？
       任何人
    b. 如何校验身份信息(证明身份)？
       数字签名算法(公私钥)
           Sign(secret_key,message)->Signature(签名)
           Verify(public_key,signature[签名],message)->true/false
    c. 即使复制数字签名信息亦可防伪造(图"3_防伪造[即使复制数字签名].png")
       因为每一次签名时都会携带从零自增的 nonce 值(而复制的签名 nonce 值
       未曾改变故发送交易时由于同已存在的 nonce 数值冲突从而导致交易无效)
    d. 如何维护账本的一致性？(如何保证所有人都维护同一份账本)
       共识算法
    e. 什么是 Hash (图"4_Hash.png")
       Hash 是一个 Trapdoor function (活板门方法)
    f. 什么是工作量证明 PoW (Proof Of Work)
       (图"5_PoW.png")
    g. 区块及区块链
       (图"7_区块.png / 8_区块链.png")
    h. 分叉(如 BTC 以 6 个区块[ 1 小时]为一个确认周期：降低分叉概率)
       篡改分叉一般得不偿失，故通常以最长链为基准续写
       (图"9_确认周期[BTC确认周期为6个区块].png")

2. 小结：  // Nonce 最新值(区块链中常用作"随机数")
    a. 账本 = 交易历史
    b. 每个人都可以向账本里添加内容
    c. 用私钥签名，公钥向他人证明。因此私钥绝对不可以泄露但公钥可示别人
    d. 每个人的交易里包含一个自增长数字（nonce），用于防止别人复制攻击
    e. 哈希函数：将任何文件转换成 256bit 的" 01 "串(不可逆)
    f. 工作量证明：不停地算出符合要求的哈希值
    g. 产生分叉怎么办? => 等待最长链出现

3. 课程大纲及课后作业
    a. 图"10_课程大纲及课后作业.png"
    b. 比特币白皮书参考网页" http://www.btcpapers.com/ "
    c. 课后作业：" BTC白皮书-观后感.docx "

 */