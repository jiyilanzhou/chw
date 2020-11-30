
// bond[bɒnd]v&n.债券,粘合(结合)  // (对比) bind[baɪnd]v.绑定,连结
// bonded['bɒndɪd]adj.黏合的,抵押的
// preliminary[prɪˈlɪmɪnəri]adj&n.初步(的),预备(的)       // stash[stæʃ]v&n.存放,藏匿
/*
0. Run a Validator (Polkadot)           运行验证器

1. Preliminaries                        预赛
    How many DOT do I need?             需要几个 DOT？

2. Initial Set-up            最初设置
    a. Requirements          要求
    b. Install Rust          安装
    c. Install & Configure Network Time Protocol (NTP) Client      安装和配置网络时间协议（NTP）客户端
    d. Building and Installing the polkadot Binary                 生成和安装polkadot二进制文件
    e. Synchronize Chain Data                                      同步链数据

3. Bond DOT                         债券点
    a. Stash account                储藏帐户
    b. Controller account           控制器帐户
    c. Value bonded                 保证金价值
    d. Payment destination          付款目的地

4. Set Session Keys                             设置会话密钥
    a. Generating the Session Keys              生成会话密钥
        Option 1: PolkadotJS-APPS               选项1：PolkadotJS-APPS
        Option 2: CLI                           选项2：CLI
    b. Submitting the setKeys Transaction       提交 setKeys 交易

5. Validate     验证

6. FAQ          常见问题
    a. Why am I unable to synchronize the chain with 0 peers?   为何无法与 0 个对等方同步链？
    b. How do I clear all my chain data?                        如何清除所有的链数据？

7. VPS List         VPS 清单

8. Using Docker     使用 docker

 */