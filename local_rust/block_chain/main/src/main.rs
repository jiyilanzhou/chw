// peer[pɪə(r)]n.同级,对等
/*
0. 区块链介绍
    a. peer to peer : (对等/同级)点对点,端对端
    b. Bitcoin : A Peer-to-Peer Electronic Cash System(即"比特币[一种
       去中心化的、共享的、防篡改的分布式账本]：一种点对点的电子现金系统)

1. 依赖
   编译时会检测" crate/Cargo.toml "并下载依赖

*/
// 导入 blockchain (先于" crate/Cargo.toml "内引入本地依赖)
use core::blockchain;

fn main() {
    // 创建区块链
    let mut bc = blockchain::BlockChain::new_blockchain();
    // 模拟挖矿：
    println!("----- start mining ... ");
    // 探矿耗时 3 秒
    std::thread::sleep(std::time::Duration::from_secs(3));
    // 添加区块
    bc.add_block(" a -> b : 6 BTC".to_string());
    bc.add_block(String::from(" c -> d : 8 ETH"));
    // 遍历区块
    for b in bc.blocks {
        println!("--------------------------------------");
        println!("{:#?}\n", b);
    }
}
