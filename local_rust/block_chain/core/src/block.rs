
// 按需导入第三方依赖包
use serde::{Serialize,Deserialize};
// 区块结构：区块头  // 使用需先在" core/Cargo.toml 引入" serde "
#[derive(Serialize,Deserialize,Debug,PartialOrd, PartialEq)]
pub struct BlockHeader {
    // time stamp : 时间戳
    pub time: i64,
    // transactions merkele hash : 所有交易数据的默克尔哈希
    pub tx_hash: String,
    // hash of the previous block : 前一个区块哈希
    pub pre_hash: String,
}

// 区块结构：区块(内含区块头及区块体)
#[derive(Debug)]
pub struct Block {
    // 区块头
    pub header: BlockHeader,
    // hash of the block header :当前区块哈希(即获取当前区块头的哈希)
    pub hash: String,
    /* transactions data ：交易数据(一般为"一组交易列表"[此例简化
       为一个交易，故使用 String 表示])
    */
    pub data: String,
}

// 将第三方依赖按需导入作用域
use chrono::prelude::*;
// 将本地依赖按需导入作用域
use util::code;

impl Block {
    // 设置 hash 则涉及修改 block 对象故使用 &mut self
    fn set_hash(&mut self) {
        // 序列化: my_serialize 接收的参数需可序列化
        // 故需将 BlockHeader 实现 Serialize trait
        let header = code::my_serialize(&(self.header));
        self.hash = code::get_hash(&header[..]);
    }
    // 关联函数：创建区块
    pub fn new_block(data: String, pre_hash: String) -> Block {
        // 对传入的交易数据序列化后获取其 hash
        let transactions = code::my_serialize(&data);
        let tx_hash = code::get_hash(&transactions[..]);
        // 实例化
        let time = Utc::now().timestamp();
        let mut block = Block {
            header: BlockHeader {
                time,
                tx_hash,
                pre_hash,
            },
            hash: "".to_string(),
            data,
        };

        block.set_hash();
        block
    }
}
