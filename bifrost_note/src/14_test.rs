
/*
0. 测试
    单元测试、集成测试和文档测试

1. 通过指定测试函数的名称作为 cargo test 的参数来运行特定集成测试
   # 进入到相应 crate 下"如 assets "
   # 方式 1 : 测试指定函数
    chw@chwdeMacBook-Pro assets % cargo test create_asset_should_work
   # 方式 2 ：测试指定函数(并输出测试函数中 println! 的打印内容)
    chw@chwdeMacBook-Pro assets % cargo test -- --nocapture create_asset_should_work
   # 方式 3 ：为运行特定集成测试文件中的所有测试可用" cargo test --test filename "命令(待实现)


2. shell 打开多个窗口
   gnome-terminal -x bash -c "cd ~;"



 */