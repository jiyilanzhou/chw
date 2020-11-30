
// upgrade[ˈʌpɡreɪd]v&n.升级      // voucher[ˈvaʊtʃə(r)]n. 收据、凭证
/*
0. Bifrost 升级步骤：

1. 准备工作: 下载编译运行
    a. 拉取：" https://github.com/bifrost-finance/bifrost "
    b. 编译：进入" bifrost "编译" cargo build "
    c. 运行(进入" bifrost "运行): 启动单结点 或 启动多结点
       (0). 启动单结点：
            # 先清空之前数据
            " ./target/debug/bifrost-node purge-chain --dev "
            # 启动
            " ./target/debug/bifrost-node --dev "
       (1). 启动多结点:
            # 先清空数据" rm -rf /tmp/alice/ "及" rm -rf /tmp/bob/ "
            启动 alice 结点：
                " ./target/debug/bifrost-node --alice --chain local --base-path /tmp/alice "
            另一窗口启动 bob 结点：
               " ./target/debug/bifrost-node --bob --chain local --base-path /tmp/bob "

2. 添加 pallet (以" proctice "为例)
   a. 添加 practice 模块
   b. 将" bifrost\bin\node\runtime\src\lib.rs "内" spec_version: 14, "更新为" spec_version: 15, "
   c. 编译：进入" bifrost "编译" cargo build "(编译后不可再运行[否则就不是在线升级而是重新启动了])

3. (在线升级)访问前端 polkadot.js.org
    a. 若新添加的 pallet 中有新增的类型则需在" Settings -> Developer "下追加相应类型
    b. 在线升级(适用于单结点与多结点)：
       (0). 进入" Developer -> Sudo "模块 -> 勾选 " file upload "(最好也勾选" with weight override "并
            将其值设置为" 1 ")并使用 F12 打开调试 console 观察升级信息
       (1). 将再次编译好的" bifrost/target/debug/wbuild/node-runtime "下" node_runtime.compact.wasm "
            文件拖拽到" Developer -> Sudo -> file upload "窗口 -> 点击" Submit Sudo Unchecked "
    c. 提交后观察浏览器 console (若一切正常则升级完毕即可看到左上角的版本号已更新):刷新浏览器即可测试功能

4. Native 升级
    a. 单结点 native 升级：
       (0). 停止原 bifrost-node 的运行
       (1). 启动新编译生成的 bifrost-node 看出块是否能衔接以及功能是否正常执行
    b. 双结点 native 升级(以" alice、bob "两结点为例)
       (0). 先进行在线升级(即第三步[同理在浏览器使用 F12 打开调试 console 观察升级停息]),升级完毕刷新
            浏览器测试新增模块功能
       (1). 停止原 bifrost-node 之一结点如停止 bob 结点的运行
       (2). 启动新编译生成的 bifrost-node 之 bob 结点

 */

/*
0. 连接远程
    连接" ssh bifrost@172.29.12.24 "
    输入密码:" bifrost "

1. scp 拷贝

2.  # 启动
    root@ubuntu:~/project/substrate# ./target/release/substrate  --dev
    # 查看配置( build-spec 生成 spec.json 文件)
    root@ubuntu:~/project/substrate# ./target/release/substrate build-spec >> spec.json
    root@ubuntu:~/project/substrate# vim spec.json       // 此处使用 vim 快速查看

3. 启动参数
        参数                       参数说明
        --base-path                数据存放路径
        --chain                    指定使用的链的类型        // 如 dev / local 等
        --alice                    使用预先定义的密钥
        --port                     p2p通信的端口
        --ws-port                  websocket服务端口
        --rpc-port                 rpc服务端口
        --node-key                 指定libp2p使用的私钥
        --telemetry-url            统计数据提交的地址
        --validator                作为验证人加入网络
        --light                    运行轻客户端模式

 */

/* Bifrost 多结点升级:

0. 准备工作: 下载编译运行
    a. 拉取：" https://github.com/bifrost-finance/bifrost "
    b. 编译：进入" bifrost "编译" cargo build --release "
    c. 运行(进入" bifrost "项目启动多结点):
        # 先清空数据
            rm -rf /tmp/alice/
            rm -rf /tmp/bob/
            rm -rf /tmp/charlie/
            rm -rf /tmp/dave
        # 启动 alice 结点：
             ./target/debug/bifrost-node --alice --chain local --base-path /tmp/alice
        # 新窗口启动 bob 结点：
             ./target/debug/bifrost-node --bob --chain local --base-path /tmp/bob
        # 新窗口启动 charlie 结点：
             ./target/debug/bifrost-node --charlie --chain local --base-path /tmp/charlie
        # 新窗口启动 dave 结点：
             ./target/debug/bifrost-node --dave --chain local --base-path /tmp/dave

1. 添加 pallet (以 practice pallet 为例)
   a. 编辑并添加 practice 模块
   b. 将" bifrost\bin\node\runtime\src\lib.rs "内" spec_version: 14, "更新为" spec_version: 15, "
   c. 编译：进入" bifrost "编译" cargo build --release "

3. (在线升级)访问前端" https://polkadot.js.org/apps/ "
    a. 若新添加的 pallet 中有新增的类型则需在" Settings -> Developer "下追加相应类型
    b. 在线升级：
       (0). 进入" Developer -> Sudo "模块 -> 勾选 " file upload "(最好也勾选" with weight override "并
            将其值设置为" 1 ")并使用 F12 打开调试 console 观察升级信息
       (1). 将再次编译好的" bifrost/target/debug/wbuild/node-runtime "下" node_runtime.compact.wasm "
            文件拖拽到" Developer -> Sudo -> file upload "窗口 -> 点击" Submit Sudo Unchecked "
    c. 提交后观察浏览器 console (若一切正常则升级完毕即可看到左上角的版本号已更新):刷新浏览器即可测试功能

4. Native 升级
    a. 单结点 native 升级：
       (0). 停止原 bifrost-node 的运行
       (1). 启动新编译生成的 bifrost-node 看出块是否能衔接以及功能是否正常执行
    b. 双结点 native 升级(以" alice、bob "两结点为例)
       (0). 先进行在线升级(即第三步[同理在浏览器使用 F12 打开调试 console 观察升级停息]),升级完毕刷新
            浏览器测试新增模块功能
       (1). 停止原 bifrost-node 之一结点如停止 bob 结点的运行
       (2). 启动新编译生成的 bifrost-node 之 bob 结点

# 编辑并添加 pallet 模块(此例添加 practice )
# 将" bifrost\bin\node\runtime\src\lib.rs "内" spec_version "的版本号加一
 */

/*

# clear
rm -rf /tmp/alice/ \
/tmp/bob/ \
/tmp/charlie/ \
/tmp/dave/

====================================================================================================
                                            # DEV
# alice
./target/release/bifrost-node --alice \
--chain=dev \
--base-path /tmp/alice \
--port 30333 \
--validator

# bob
./target/release/bifrost-node --bob \
--chain=dev \
--base-path /tmp/bob \
--port 30334 \
--validator

# charlie
./target/release/bifrost-node --charlie \
--chain=dev \
--base-path /tmp/charlie \
--port 30335 \
--validator

# dave
./target/release/bifrost-node --dave \
--chain=dev \
--base-path /tmp/dave \
--port 30335 \
--validator

====================================================================================================
                                            # LOCAL
# clear
rm -rf /tmp/alice/ \
/tmp/bob/ \
/tmp/charlie/ \
/tmp/dave/

# alice
./target/release/bifrost-node --alice \
--chain local \
--base-path /tmp/alice \
--port 30333 \
--validator

# bob
./target/release/bifrost-node --bob \
--chain local \
--base-path /tmp/bob \
--port 30334 \
--validator

# charlie
./target/release/bifrost-node --charlie \
--chain local \
--base-path /tmp/charlie \
--port 30335 \
--validator

# dave
./target/release/bifrost-node --dave \
--chain local \
--base-path /tmp/dave \
--port 30335 \
--validator

====================================================================================================

# 没有 key 故无法启动
./target/release/bifrost-node --alice --chain bifrost --base-path /tmp/alice/
./target/release/bifrost-node --bob --chain bifrost --base-path /tmp/bob/

====================================================================================================
# 查看修改的代码
cat bin/node/cli/src/chain_spec.rs | grep authority_keys_from_seed

 */

/* Bifrost 多结点升级(此例以两个结点为例):

0. 准备工作: 下载编译运行
    a. 拉取：" https://github.com/bifrost-finance/bifrost "
    b. 编译：进入" bifrost "编译" cargo build --release "
    c. 运行(进入" bifrost "项目启动多结点):
        # 先清空数据
            rm -rf /tmp/alice/
            rm -rf /tmp/bob/
            rm -rf /tmp/charlie/
            rm -rf /tmp/dave
        # 启动 alice 结点：
             ./target/debug/bifrost-node --alice --chain local --base-path /tmp/alice
        # 新窗口启动 bob 结点：
             ./target/debug/bifrost-node --bob --chain local --base-path /tmp/bob
        # 新窗口启动 charlie 结点：
             ./target/debug/bifrost-node --charlie --chain local --base-path /tmp/charlie
        # 新窗口启动 dave 结点：
             ./target/debug/bifrost-node --dave --chain local --base-path /tmp/dave

1. 添加 pallet (以 practice pallet 为例)
   a. 编辑并添加 practice 模块
   b. 将" bifrost\bin\node\runtime\src\lib.rs "内" spec_version: 14, "更新为" spec_version: 15, "
   c. 编译：进入" bifrost "编译" cargo build --release "

2. (在线升级)访问前端" https://polkadot.js.org/apps/ "
    a. 若新添加的 pallet 中有新增的类型则需在" Settings -> Developer "下追加相应类型
    b. 在线升级：
       (0). 进入" Developer -> Sudo "模块 -> 勾选 " file upload "(最好也勾选" with weight override "并
            将其值设置为" 1 ")并使用 F12 打开调试 console 观察升级信息
       (1). 将再次编译好的" bifrost/target/debug/wbuild/node-runtime "下" node_runtime.compact.wasm "
            文件拖拽到" Developer -> Sudo -> file upload "窗口 -> 点击" Submit Sudo Unchecked "
    c. 提交后观察浏览器 console (若一切正常则升级完毕即可看到左上角的版本号已更新):刷新浏览器即可测试功能

3. Native 升级
    a. 单结点 native 升级：
       (0). 停止原 bifrost-node 的运行
       (1). 启动新编译生成的 bifrost-node 看出块是否能衔接以及功能是否正常执行
    b. 双结点 native 升级(以" alice、bob "两结点为例)
       (0). 先进行在线升级(即第三步[同理在浏览器使用 F12 打开调试 console 观察升级停息]),升级完毕刷新
            浏览器测试新增模块功能
       (1). 停止原 bifrost-node 之一结点如停止 bob 结点的运行
       (2). 启动新编译生成的 bifrost-node 之 bob 结点


 */