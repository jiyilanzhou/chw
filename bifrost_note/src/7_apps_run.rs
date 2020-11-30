
/*
0. 运行" Polkadot app ":
    a. 拉取项目" ttps://github.com/polkadot-js/apps "
    b. 进入 apps 执行" sudo yarn install "及" yarn start "
    c. 前端使用" http://localhost:3000 "访问

1. 启动 bifrost 多结点进行 Staking 测试
    注：" --unsafe-ws-external 及 --rpc-cors all "只需在其中之一结点添加即可
    但欲使 apps 访问即" http://localhost:3000/?rpc=ws://10.115.27.96:9944#/settings/developer "由必须
    添加" --unsafe-ws-external 及 --rpc-cors all "

    # clear
rm -rf /tmp/alice/ \
/tmp/bob/ \
/tmp/charlie/ \
/tmp/dave/ \
/tmp/eve/ \
/tmp/ferdie/ \

    # alice
./target/release/bifrost-node \
--base-path /tmp/alice \
--chain=dev \
--alice \
--port 30333 \
--rpc-port 9933 \
--ws-port 9944 \
--unsafe-ws-external \
--rpc-cors all  \
--node-key 0000000000000000000000000000000000000000000000000000000000000001 \
--validator

    # bob
./target/release/bifrost-node \
--base-path /tmp/bob \
--chain=dev \
--bob \
--bootnodes /ip4/127.0.0.1/tcp/30333/p2p/12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp \
--port 30334 \
--rpc-port 9934 \
--ws-port 9945 \
--validator

    /* a. 欲使用本地终端远程访问即：
                 curl -H "Content-Type: application/json" -d '{"id":1, "jsonrpc":"2.0", "method":
                "author_rotateKeys", "params":[]}' http://localhost:9935
            "访问则还必须添加" --unsafe-rpc-external "
        b. 登录到远程直接访问(即启动结点无需添加" --unsafe-rpc-external "参数)
                curl -H "Content-Type: application/json" -d '{"id":1, "jsonrpc":"2.0", "method": "author_rotateKeys", "params":[]}' http://localhost:9935
    */

    # charlie
./target/release/bifrost-node \
--base-path /tmp/charlie \
--chain=dev \
--charlie \
--port 30335 \
--rpc-port 9935 \
--ws-port 9946 \
--bootnodes /ip4/127.0.0.1/tcp/30333/p2p/12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp \
--validator

    # dave
./target/release/bifrost-node \
--base-path /tmp/dave \
--chain=dev \
--dave \
--port 30336 \
--rpc-port 9936 \
--ws-port 9947 \
--bootnodes /ip4/127.0.0.1/tcp/30333/p2p/12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp \
--validator

    # eve
./target/release/bifrost-node \
--base-path /tmp/eve \
--chain=dev \
--eve \
--port 30337 \
--rpc-port 9937 \
--ws-port 9948 \
--bootnodes /ip4/127.0.0.1/tcp/30333/p2p/12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp \
--validator

    # ferdie
./target/release/bifrost-node \
--base-path /tmp/ferdie \
--chain=dev \
--ferdie \
--port 30338 \
--rpc-port 9938 \
--ws-port 9949 \
--bootnodes /ip4/127.0.0.1/tcp/30333/p2p/12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp \
--validator

    # apps 访问
   http://localhost:3000/?rpc=ws://10.115.27.96:9944#/settings/developer
   http://localhost:3000/?rpc=ws://10.115.27.96:9945#/settings/developer

 */

/*
0. 添加 validator 结点：
   a. Network -> Staking -> Account actions -> Validator
              -> 添加 stash account 及 controller account(此例均选择" CHARLIE ") -> next
   b. 登录远程访问 charlie 相应端口如：( charlie 对应的启动端口为" 9935 ")
      bifrost@bifrost-build-machine:~$ curl -H "Content-Type: application/json" -d '{"id":1, "jsonrpc":"2.0", "method": "author_rotateKeys", "params":[]}' http://localhost:9935
        {"jsonrpc":"2.0","result":"0x1fb53a78613d01e3dc9ed2fc87a9a283cb5543c5bc3cd60cfd0f2b42e5717f74929fe900f32d6fcdbbe36e229f8e115b8c52e5d4ff0db5bdf8cad658dcc521740a5f5cf2fcc016abced4fcdb298424c7d34f697556d7df5389370b124b9b052406f6b97387f82b4f042a4b27e4e724d4f6ca8f15644162aa8678057921490d0c","id":1}
        // 拷贝 result 的值即" 0x1fb53a78613d01e3dc9ed2fc87a9a283cb5543c5bc3cd60cfd0f2b42e5717f74929fe900f32d6fcdbbe36e229f8e115b8c52e5d4ff0db5bdf8cad658dcc521740a5f5cf2fcc016abced4fcdb298424c7d34f697556d7df5389370b124b9b052406f6b97387f82b4f042a4b27e4e724d4f6ca8f15644162aa8678057921490d0c "
        // 注：" result "值对应的 key 可在" /tmp/charlie/chains/dev/keystore "下查看
   c. 将拷贝的 result 拷贝至第一步中的 next 对话框内的" key from rotateKeys "并设置" reward commission percentage "
   d. 提交并回到" Network -> Staking -> Staking overview "界面等待" epoch "下一个时间结束查看 validators

1. 设置 validator 结点数量
    a. Developer -> Sudo -> Sudo access -> 选择" staking "模块并选择其下拉列表" setValidator(new) "选项
    b. 设置数量后提交并于网页端查看变化情况(如" waiting "数量即是准备出块的新加结点)
    c. 观察 Network -> Explorer -> finalize 及 best
           finalize 为最终出块的高度
           best 为最佳出块高度

2. 解决 finalize 卡住的问题
    a. Developer -> Sudo -> grandpa -> 选择 noteStalled(delay,best finalized block number)
    b. 填写" delay: BlockNumber "及" best_finalized_block_number: BlockNumber "

3. 修改 epoch 等待时间(开发时测试专用): " era "生效时长则为" epoch "的 6 倍
   在" bifrost\bin\node\runtime\src\constants.rs "文件内更新
   即" pub const EPOCH_DURATION_IN_BLOCKS: BlockNumber = 30 * MINUTES; "

 */

/*
    curl -H "Content-Type: application/json" -d '{"id":1, "jsonrpc":"2.0", "method": "author_rotateKeys", "params":[]}' http://localhost:9935


 */