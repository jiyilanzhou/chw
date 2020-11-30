
/*
0. serde (反)序列化
    文档" https://crates.io/crates/serde "
         -> (深入文档)Documentation -> " https://serde.rs/ "

1. 序列化与反序列化场景
    简言之序列化是用于处理对象流(将对象内容进行流化)的机制，对象流读写操作
    会引发一些问题而序列化机制正是用来解决这些问题(网络传输对象须进行流化)

2. serde 相关
    a. serde crate 是 Serde 生态的核心
    b. serde_derive crate 提供必要的工具(如为结构体、枚举提供序列化与反序列化)
    c. serde 使用过程宏来派生 Serialize 和 Deserialize
    d. serde 只提供框架，具体序列化和反序列化操作还需依赖具体包如 serde_json 等

*/

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct ServerConfig {
    workers: u64,
    ignore: bool,
    auth_server: Option<String>,
}

fn main() {
    // 初始化
    let config = ServerConfig {
        workers: 100,
        ignore: false,
        auth_server: Some("auth.server.io".to_string()),
    };

    // Json 序列化与反序列化
    {
        println!("json:");
        let serialized = serde_json::to_string(&config).unwrap();
        println!("serialized: {}", serialized);
        let deserialized: ServerConfig = serde_json::from_str(&serialized).unwrap();
        println!("deserialized: {:#?}", deserialized);
    }

    // yaml 序列化与反序列化
    {
        println!("yaml:");
        let serialized = serde_yaml::to_string(&config).unwrap();
        println!("serialized: {}", serialized);
        let deserialized: ServerConfig = serde_yaml::from_str(&serialized).unwrap();
        println!("deserialized: {:#?}", deserialized);
    }

}

