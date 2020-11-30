
/*
0. Runtime Execution    运行时执行
    Substrate 运行时的执行由 Executive 模块协调。与 FRAME 中的其他模块不同，这不是运行时模块，而只是一个
    普通的 Rust 模块，该模块调用区块链中包含的各种运行时模块

1. Validating Transactions 验证交易

2. Executing a Block 执行块
    a. Initializing a Block  初始化块
    b. Executing Extrinsics     执行外部
    c. Finalizing a Block   完成块

 */