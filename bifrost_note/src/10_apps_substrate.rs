
/*

    # clear
rm -rf /tmp/alice/ \
/tmp/bob/ \
/tmp/charlie/ \
/tmp/dave/ \
/tmp/eve/ \
/tmp/ferdie/ \

    # alice
./target/release/substrate \
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
./target/release/substrate \
--base-path /tmp/bob \
--chain=dev \
--bob \
--bootnodes /ip4/127.0.0.1/tcp/30333/p2p/12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp \
--port 30334 \
--rpc-port 9934 \
--ws-port 9945 \
--validator

    # charlie
./target/release/substrate \
--base-path /tmp/charlie \
--chain=dev \
--charlie \
--port 30335 \
--rpc-port 9935 \
--ws-port 9946 \
--bootnodes /ip4/127.0.0.1/tcp/30333/p2p/12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp \
--validator

    # dave
./target/release/substrate \
--base-path /tmp/dave \
--chain=dev \
--dave \
--port 30336 \
--rpc-port 9936 \
--ws-port 9947 \
--bootnodes /ip4/127.0.0.1/tcp/30333/p2p/12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp \
--validator

    # eve
./target/release/substrate \
--base-path /tmp/eve \
--chain=dev \
--eve \
--port 30337 \
--rpc-port 9937 \
--ws-port 9948 \
--bootnodes /ip4/127.0.0.1/tcp/30333/p2p/12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp \
--validator

    # ferdie
./target/release/substrate \
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
