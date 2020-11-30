
/*
0. ssh 配置（需要使用 root 权限）

1. 创建ssh客户端的密钥
    chw@chwdeMacBook-Pro ~ % sudo ssh-keygen
        然后根据提示，一路选择对应的路径，选择默认配置就行，
        直接敲回车选择默认配置。一直到出现下面的乱码一样的内容，表示配置成功

2. 查看公钥
    chw@chwdeMacBook-Pro ~ % cat /Users/chw/.ssh/id_rsa.pub

3. 密钥配置完成后，需要将公钥复制到你需要免密登陆的服务器
    chw@chwdeMacBook-Pro ~ % ssh-copy-id bifrost@10.115.27.96

4. mac 安装 yarn
    brew install yarn

5. runtime 模块引入 pallet
    文件" substrate/bin/node/runtime/Cargo.toml "追加如：
    pallet-poe = { version = "2.0.0", default-features = false, path = "../../node-template/pallets/poe" }

6. 修改版本号
    substrate/bin/node/runtime/src/lib.rs        // 搜索 spec_version
    # 进入" substrate "进行远程编译
      cargo remote -e /home/bifrost/.profile -- build --release
    # 远程拷贝至本地
      scp -r bifrost@10.115.27.96:/home/bifrost/remote-builds/18332495490393433006/target/release/wbuild/node-runtime/node_runtime.compact.wasm ./


 */