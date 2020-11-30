
/*
0.  编译 substrate (使用官方源安装[国内源暂未及时更新部分依赖包])
    # 切换为官方镜像源 (注释 #replace-with = 'ustc' 即可)
        root@ubuntu:~# cat .cargo/config
            [source.crates-io]
            registry = "https://github.com/rust-lang/crates.io-index"
            #replace-with = 'ustc'
            [source.ustc]
            registry = "git://mirrors.ustc.edu.cn/crates.io-index"
    a. 安装 Rust 环境
        curl https://getsubstrate.io -sSf | bash -s -- --fast
        // 已安装 Rust 环境的可忽略
    b. 添加依赖库
        (0). Ubuntu
            root@ubuntu:~# apt install -y cmake pkg-config libssl-dev git gcc build-essential git clang libclang-dev
            root@ubuntu:~# cmake --version
                cmake version 3.16.3
        (1). CentOS 添加依赖库
             # 安装 cmake
             [root@centos ~]# yum -y install cmake
             [root@centos ~]# cmake --version
                   cmake version 3.11.4
              # 安装 llvm、clang
              [root@centos ~]# yum install -y llvm-devel clang-devel
              [root@centos ~]# clang --version
                   clang version 9.0.1 (Red Hat 9.0.1-2.module_el8.2.0+309+0c7b6b03)
                      # 另一种安装 llvm 的方案(未使用)
                       yum list |grep llvm
                       yum install llvm-toolset.i686 llvm-toolset.x86_64
                       yum install libatomic
                       ldconfig
                       llvm --version
               # 安装 openssl
               [root@centos ~]# yum install openssl openssl-devel
               [root@centos ~]# openssl version
                    OpenSSL 1.1.1c FIPS  28 May 2019
    c. rust 编译链
        # 安装 nightly 编译链
        root@ubuntu:~# rustup update nightly
        # 对 nightly 编译链添加 wasm 编译 target
        root@ubuntu:~# rustup target add wasm32-unknown-unknown --toolchain nightly
        # (笔者建议)导出环境变量
        root@ubuntu:~# export WASM_BUILD_TYPE=release
    d. 编译
        # 拉取 substrate 项目
        git clone https://github.com/paritytech/substrate.git
        #  进入 substrate 编译(暂不支持 Windows )
        root@ubuntu:~# cd project/substrate/
        root@ubuntu:~/project/substrate# cargo build --release

1. 运行 Substrate
    a. 单结点启动(开发模式)
       [root@centos substrate]# ./target/release/substrate --dev
       # 删除存储目录
       [root@centos substrate]# ./target/release/substrate purge-chain --dev
    b. 多结点启动(指定链)
        # 清空
        rm -rf /tmp/alice
        rm -rf /tmp/bob
        # (多节点网络)仅启动单一结点(首结点)时不会出块(等待其余结点的加入形成多结点)
        ./target/release/substrate --alice --chain local --base-path /tmp/alice
        # 启动另一结点(启动 bob 节点: " --base-path "即链的数据库目录)
         ./target/release/substrate --bob --chain local --base-path /tmp/bob
    b. 客户端
       测试网络:" https://polkadot.js.org "
       其左侧栏" Extrinsics "可理解为由 web 触发的交易
       // extrinsic[eksˈtrɪnsɪk]adj.外在的,外来的

 */