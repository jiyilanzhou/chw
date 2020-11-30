
/*
0. 远程编译
   (前提条件)远程配置

1. 登录远程机器配置密钥登录
    a. 本地生成密码
        # ssh-keygen
        root@ubuntu:~/project/bifrost# ssh-keygen
        # 查看并拷贝公钥
        root@ubuntu:~/project/bifrost# cat /root/.ssh/id_rsa.pub
    b. 登录远程
        命令" ssh bifrost@10.115.27.96 "
        键入密码" bifrost "
    c. 查看远程 authorized_keys 并追加本地公钥
        bifrost@bifrost-build-machine:~$ vim ~/.ssh/authorized_keys

2. 生成 cargo-remote 命令(" https://github.com/sgeisler/cargo-remote ")
    a. Linux 系统下拉取并编译
        git clone https://github.com/sgeisler/cargo-remote
        cargo install --path cargo-remote/
    b. MacOS 系统
        brew install rsync

3. 远程编译：
    a. 拉取项目"
    b. 进入" bifrost " 项目配置" .cargo-remote.toml "
        root@ubuntu:~/project/bifrost# cat .cargo-remote.toml
        remote = "bifrost@10.115.62.106"
    c. 进入" bifrost "进行远程编译
        cargo remote -e /home/bifrost/.profile -- build --release

4. scp : secure copy
    a. 从本地复制到远程(目录使用" -r ")
        root@ubuntu:~/project/bifrost# scp ./.cargo-remote.toml bifrost@10.115.27.96:/root/
    b. 从远程复制到本地
        root@ubuntu:~/project/bifrost# scp -r bifrost@10.115.27.96:/home/bifrost/remote-builds/5015764390489952359/target ./




 */