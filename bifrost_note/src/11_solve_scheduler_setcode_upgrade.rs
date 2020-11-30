
/*
0. 对比 bifrost 及 substrate
   # 拉取项目
   git clone https://github.com/paritytech/substrate.git
   # 查看 tag
   git tag
   # 切换到相应 tag
   git checkout v2.0.0
   # 查看日志
   git log

1. 对比 lib.rs
    /Users/chw/project/bifrost/bin/node/runtime/src/lib.rs
    /Users/chw/project/substrate/bin/node/runtime/src/lib.rs
    # 存在不同：
     diff /Users/chw/project/bifrost/bin/node/runtime/src/lib.rs  /Users/chw/project/substrate/bin/node/runtime/src/lib.rs


2. 先测试 substrate
    # clear
rm -rf /tmp/alice/ \
/tmp/bob/ \
/tmp/charlie/ \
/tmp/dave/ \
/tmp/eve/ \
/tmp/ferdie/

    # alice
./target/release/bifrost \
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
./target/release/bifrost \
--base-path /tmp/bob \
--chain=dev \
--bob \
--bootnodes /ip4/127.0.0.1/tcp/30333/p2p/12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp \
--port 30334 \
--rpc-port 9934 \
--ws-port 9945 \
--validator

    # charlie
./target/release/bifrost \
--base-path /tmp/charlie \
--chain=dev \
--charlie \
--port 30335 \
--rpc-port 9935 \
--ws-port 9946 \
--bootnodes /ip4/127.0.0.1/tcp/30333/p2p/12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp \
--validator

    # dave
./target/release/bifrost \
--base-path /tmp/dave \
--chain=dev \
--dave \
--port 30336 \
--rpc-port 9936 \
--ws-port 9947 \
--bootnodes /ip4/127.0.0.1/tcp/30333/p2p/12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp \
--validator

 */
