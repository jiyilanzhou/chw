
/*
0. DeFi : decentralized finance 去中心化金融
    a. Defi 常被称为"分布式金融"或"开放金融"(因本质上目前还未存在完全的"去中心化"金融)。大多数都是不同程度
       的中心化和去中心化的结合。不过 DeFi 已经约定俗成，更适合传播
    b. 与传统金融相比 DeFi 通过区块链技术实现去中介化，减少中间角色从而降低中间环节所带来的巨额成本。
       (0). 示例：如拥有一笔 ETH 财产且希望用它来作为抵押物获得贷款，通过去中心化金融来实现则很简单。如此
            在融资效率和简洁性上就比传统金融要优秀很多。只需归还借款并支付约定利息或手续费就能赎回自己原来
            的 ETH，即获得了贷款又有机会享受币价增值带来的收益。而整个过程将没有任何的人为参与，完全通过
            代码运行来实现
       (1). 可见在 DeFi 体系中，传统金融领域中的特权机构被代码和智能合约所取代，随着去中心化金融的逐步落地，
            未来普通大众将有机会享受到自己金融资产所蕴含的金融价值，金融业务使用者也有机会用更低的成本享受
            到服务，整个金融体系的运行效率被极大地提升，而成本则被大大降低
       (2). 从更高维度来看 DeFi 主要愿景是将所有资产通证化，最终在全球形成一个无国界的开放金融系统。在这里
            一切操作由智能合约代码运行，没有暗箱、没有隐私信息被审查或利用等的操作。一切数据公开透明，无需
            相互信任，也无需任何的准入门槛，所有人可以以任意的颗粒度进行公平的交易
    c. 在区块链技术的支撑下这一切已经在不断的实现过程中，有理由对 DeFi 的未来更加充满期待。同时也正是由于
       现阶段去中心化金融的体量与传统金融相比极其微小，发展空间巨大，因此对于投资者而言雏形已现的 DeFi 未来
       有机会为自己带来巨大的投资回报

1. DeFi 主体功能
    a. 传统金融(中心化金融)的核心功能是借贷，延伸有支付、期货、期权、保证金、投资管理等方面
    b. DeFi(开放金融)是一种无需许可的金融生态。其功能特性发挥，首先必然要围绕传统金融核心功能(即借贷)进行
       (0). 于是借贷方面 DeFi 蓬勃发展：如 MakerDAO[以太坊上的央行]、Compound[自动化商行]、Dharma[点对点
            借贷]等，它们类似于传统金融的银行，是加密的 DeFi 银行。其中 MakerDAO、Compound 当前锁定的资产
            远超其它 DeFi 项目)。 // 可参阅" https://defipulse.com/ "
       (1). 在交易支付方面出现的 DeFi 有：Kyer、Uniswap、Bancor 等相当于传统金融的纽交所、上交所、纳斯达克
       (2). 在金融衍生品、投资管理方面出现的有：Augur(二元期权)、dYdY(保证金交易)、Melonport5 等相当于
            传统金融的基金、证券公司

2. DeFi 关键作用
    a. 超越传统金融的服务：部分特定群体需要掌握自身财产与金融服务，这是 DeFi 被需要的关键。因为 DeFi 具有
       去中介、无须许可、透明，其能充分满足这种群体的掌控自己资产的欲望
    b. 发挥与传统金融雷同的服务：并非所有人都具有管理资产的意愿或能力，从"专业的事交给专业的人做"而言,DeFi
       与传统金融并行存在
    c. 发挥资金托管服务作用成为传统金融的补充：进行大额交易时常有资金托管方，如进行购房时，未过户前一般会将
       资金放在托管银行，过户完成由银行将放款划给售房者，其目的主要是为了保护买卖双方的权益。
       (但在"币圈"经常会发生交易所、钱包跑路情况导致钱、币不翼而飞，究其根源是币圈缺乏资金托管服务，这对
       币圈而言是一大刚需，但目前很少有传统银行愿意做或敢于提供担保)
    d. DeFi 世界与现实世界独立存在：无需担保及资料提供，且在 DeFi 里的借贷抵押不会影响现实世界中的信誉，
       不会因数字货币资产的抵押而影响到住房贷款、消费贷款额度等

3. DeFi目前的问题和瓶颈
    a. 很多业内人士都在为 DeFi 的未来感到兴奋。但同时发展初期的现状是大多数用户对其认知程度很低，用户的数量
       也还很少。时至今日除了 MarkerDAO 和 REX 这样的明星项目外，其它项目目前的流量和沉淀资金都非常少，这
       其实也正是一个全新领域处于萌芽期的典型特征。
    b. DeFi 的发展用于底层公链的性能，另一方面去中心化的金融项目，相对于传统金融产品使用难度大很多，对用户
       的认知要求较高，这也会很大程度上影响 DeFi 的发展

*/

/*
3. 配置
    a. pull code:
        " https://github.com/bifrost-finance/bifrost "

    b. Install required tools:
       # 修改执行权限
       root@ubuntu:~/project/bifrost# chmod u+x ./scripts/init.sh
       # 安装必要工具
       root@ubuntu:~/project/bifrost# sudo ./scripts/init.sh
         错误0：
            /usr/bin/env: ‘bash\r’: No such file or directory
           # 解决方案：
             以 vim 编辑"  ./scripts/init.sh "文件: 进入到命令行模式修改当前文件格式为 unix 后保存退出
             即" : set ff=unix " / " :wq "(具体操作参见" 0_bash脚本提示.png ")
         错误1：
             root@ubuntu:~/project/bifrost# sudo ./scripts/init.sh
                *** Initializing WASM build environment
                ./scripts/init.sh: line 8: rustup: command not found
             # 查看内容
             root@ubuntu:~/project/bifrost# cat ./scripts/init.sh
                #!/usr/bin/env bash
                set -e
                echo "*** Initializing WASM build environment"
                if [ -z $CI_PROJECT_NAME ] ; then
                   rustup update nightly
                   rustup update stable
                fi
                rustup target add wasm32-unknown-unknown --toolchain nightly
                # Install wasm-gc. It's useful for stripping slimming down wasm binaries.
                command -v wasm-gc || \
                    cargo +nightly install --git https://github.com/alexcrichton/wasm-gc --force
            # 解决方案：更新
                 root@ubuntu:~/project/bifrost# rustup update

    c. Build all native code:
          cargo build

 */