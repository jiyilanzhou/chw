
/*
0. gRPC
    a. gRPC 是由 google 开发的一款开源的远程过程调用(RPC)系统
       (参阅" http://doc.oschina.net/grpc ")
    b. gRPC 默认使用 protocol buffers (结构数据序列化机制)

1.gRPC 允许定义四类服务方法(参阅" http://doc.oschina.net/grpc?t=58009 ")
    a. 单项 RPC
    b. 服务端流式 RPC
    c. 客户端流式 RPC
    d. 双向流式 RPC

2. 使用
    a. 先安装 protocol buffer 编译器
        $ sudo apt install protobuf-compiler
    b. 创建 protocol buffer 文件
       如" crate/foobar.proto "(与 src 同级)
    c. 使用示例参照
        " https://crates.io/ " -> 搜索" grpc " -> (一般点选首个搜索结果)如
        " grpc v0.8.1 " -> Repository -> (其仓库位置一般为 github 仓库)如
        " https://github.com/stepancheg/grpc-rust " -> " grpc-examples "
        (参照示例仿写)
    d. 编译完毕进入" crate/target/debug "下分别执行" server "及" client "
        root@ubuntu:~/crate # ./target/debug/server
        root@ubuntu:~/crate # ./target/debug/client

*/
