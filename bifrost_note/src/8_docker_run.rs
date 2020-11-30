
/*
0. 安装 docker    (参阅" https://docs.docker.com/engine/install/ubuntu/ ")
    a. # 查看版本
docker --version
    b. # 若安装先卸载
sudo apt-get remove docker docker-engine docker.io containerd runc
    c. # 更新列表
sudo apt-get update
    d. # 安装依赖
sudo apt-get install \
apt-transport-https \
ca-certificates \
curl \
gnupg-agent \
software-properties-common
    e. # 添加  Docker 官方 GPG key
curl -fsSL https://download.docker.com/linux/ubuntu/gpg | sudo apt-key add -
    f. # 校验
sudo apt-key fingerprint 0EBFCD88
    h. # 使用稳定版仓库
sudo add-apt-repository \
"deb [arch=amd64] https://download.docker.com/linux/ubuntu \
$(lsb_release -cs) \
stable"
    i. # 再次更新列表
sudo apt-get update
    j. # 安装 docker 引擎
sudo apt-get install docker-ce docker-ce-cli containerd.io
    k. # 安装指定版本的引擎
apt-cache madison docker-ce
    l. # 安装 docker
sudo apt-get install docker-ce -y
    m. # 验证安装
docker images
    n. # 设置 docker 加速器(可选项):于" /etc/docker/ "目录生成daemon.json
curl -sSL https://get.daocloud.io/daotools/set_mirror.sh | sh -s http://f1361db2.m.daocloud.io
    o. cat 查看 daemon.json 文件
cat /etc/docker/daemon.json
    p. 重启 docker 服务
sudo systemctl restart docker


1. 安装 yarn
    a. # 查看版本
yarn --version
    b. # 若安装先卸载
sudo apt remove cmdtest
sudo apt remove yarn
    c. #  debian 方式安装
curl -sS https://dl.yarnpkg.com/debian/pubkey.gpg | sudo apt-key add -
echo "deb https://dl.yarnpkg.com/debian/ stable main" | sudo tee /etc/apt/sources.list.d/yarn.list
    d. # 更新列表并安装
sudo apt update
sudo apt install yarn
    e. # 验证
yarn --version

*/

/*
2. 拉取镜像
    root@ubuntu:~# docker pull bifrostnetwork/bifrost:asgard-v0.5.0
    root@ubuntu:~# docker images
        REPOSITORY               TAG                 IMAGE ID            CREATED             SIZE
        bifrostnetwork/bifrost   asgard-v0.5.0       2e67f8e67561        4 days ago          213MB

3. 启动多个结点(容器外启动)
# alice
docker run -p 9988:9944 -p 30333:30333 -p 9934:9933 \
--entrypoint '/usr/local/bin/bifrost-node' \
bifrostnetwork/bifrost:asgard-v0.5.0 \
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
docker run \
--entrypoint '/usr/local/bin/bifrost-node' \
bifrostnetwork/bifrost:asgard-v0.5.0 \
--base-path /tmp/bob \
--chain=dev \
--bob \
--bootnodes /ip4/127.0.0.1/tcp/30333/p2p/12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp \
--port 30334 \
--rpc-port 9934 \
--ws-port 9945 \
--validator

# charlie
docker run \
--entrypoint '/usr/local/bin/bifrost-node' \
bifrostnetwork/bifrost:asgard-v0.5.0 \
--base-path /tmp/charlie \
--chain=dev \
--charlie \
--port 30335 \
--rpc-port 9935 \
--ws-port 9946 \
--bootnodes /ip4/127.0.0.1/tcp/30333/p2p/12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp \
--validator

# dave
docker run \
--entrypoint '/usr/local/bin/bifrost-node' \
bifrostnetwork/bifrost:asgard-v0.5.0 \
--base-path /tmp/dave \
--chain=dev \
--dave \
--port 30336 \
--rpc-port 9936 \
--ws-port 9947 \
--bootnodes /ip4/127.0.0.1/tcp/30333/p2p/12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp \
--validator

# eve
docker run \
--entrypoint '/usr/local/bin/bifrost-node' \
bifrostnetwork/bifrost:asgard-v0.5.0 \
--base-path /tmp/eve \
--chain=dev \
--eve \
--port 30337 \
--rpc-port 9937 \
--ws-port 9948 \
--bootnodes /ip4/127.0.0.1/tcp/30333/p2p/12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp \
--validator

# ferdie
docker run \
--entrypoint '/usr/local/bin/bifrost-node' \
bifrostnetwork/bifrost:asgard-v0.5.0 \
--base-path /tmp/ferdie \
--chain=dev \
--ferdie \
--port 30338 \
--rpc-port 9938 \
--ws-port 9949 \
--bootnodes /ip4/127.0.0.1/tcp/30333/p2p/12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp \
--validator

 */

/*
docker ps 查看进程

docker stop  CONTAINER_ID

 */

/*
0. 通过 dev.json 启动
    a. 远程拷贝" /home/bifrost/jdeng/duplicated-trx/bifrost/dev.json "文件

1. 更新 charlie

./bifrost-node \
--base-path /tmp/charlie \
--chain dev18.json \
--charlie \
--port 30335 \
--rpc-port 9935 \
--ws-port 9946 \
--bootnodes /ip4/127.0.0.1/tcp/30333/p2p/12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp \
--validator

build-spec --chain dev > dev18.json

存在问题: 通过 docker 启动的 bifrost 结点后，本地启动如何加入原有链中[???]


 */

