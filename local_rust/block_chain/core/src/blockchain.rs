use crate::block;

// 区块链结构
pub struct BlockChain {
    // 区块链即是区块集合
    pub blocks: Vec<block::Block>,
}

impl BlockChain {
    // 添加区块
    pub fn add_block(&mut self, data: String) {
        // 获取前一个区块的哈希
        let pre_block = &self.blocks[self.blocks.len() - 1];
        // 创建新区块
        let new_block = block::Block::new_block(data, pre_block.hash.clone());
        // 将区块装入容器(此例使用 vec 容器[一般使用"链表"实现])
        self.blocks.push(new_block);
    }

    // 创世区块(仅内部调用故不可用 pub 修饰)
    fn new_genesis_block() -> block::Block {
        block::Block::new_block("This is genesis block".to_string(), String::from(""))
    }

    // 创建区块链
    pub fn new_blockchain() -> BlockChain {
        BlockChain{
            blocks: vec![BlockChain::new_genesis_block()],
        }
    }
}