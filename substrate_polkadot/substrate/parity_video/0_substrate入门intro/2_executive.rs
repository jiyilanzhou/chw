
/*
0. " substrate\frame\executive "模块：
    a. 从其 Cargo.toml 内的命名" frame-xxx '可知其重要性比肩 system、support 等。
    b. "\frame\executive\src\lib.rs"文件内并没有像其它模块内的"宏"展开，而是整个
       运行时逻辑的入口如" pub fn execute_block(block: Block) {...} ＂其源码：
       	 pub fn execute_block(block: Block) {     // 其内定义区块链的执行逻辑
       	     // 初始化区块头
             Self::initialize_block(block.header());

             // any initial checks
             Self::initial_checks(&block);

             let signature_batching = sp_runtime::SignatureBatching::start();

             // execute extrinsics
             let (header, extrinsics) = block.deconstruct();
             Self::execute_extrinsics_with_book_keeping(extrinsics, *header.number());

             if !signature_batching.verify() {
                 panic!("Signature verification failed.");
             }

             // any final checks
             Self::final_checks(&header);
         }
       // 注：可自定义区块链执行逻辑([传统区块链无法实现]最大程度掌控 substrate )


 */