/*
0. 工具库
   具体依赖可从 crates.io 上寻找

1. 与序列化相关的
    a. https://crates.io/crates/serde
    b. 在"block_chain\utils\Cargo.toml"配置
        [dependencies]
        serde = "1.0.106"
        bincode = "1.2.1"
        // 添加依赖后可于相应 crate 下执行" cargo build "下载依赖

2. 编码

*/
use bincode;
use serde::{Deserialize, Serialize};

/* 参阅 bincode::serialize
   a. Terminal:
       Administrator@CHW MINGW64 /e/project/block_chain/util (master)
       $ cargo doc --open
   b. 文档源码
       Function bincode::serialize
       pub fn serialize<T: ?Sized>(value: &T) -> Result<Vec<u8>>
       where
           T: Serialize,
       [−]
       Serializes a serializable object into a Vec of bytes using the default
                                                                configuration.
*/
// 序列化  // " ?Sized "表示编译时未知大小
pub fn my_serialize<T: ?Sized>(value: &T) -> Vec<u8>
    where
        T: Serialize,
{
    let serialized = bincode::serialize(value).unwrap();
    serialized
}

/* 反序列化: 同理参阅 bincode::deserialize
   文档源码:
    Function bincode::deserialize
    pub fn deserialize<'a, T>(bytes: &'a [u8]) -> Result<T>
    where
        T: Deserialize<'a>,
    [−]
    Deserializes a slice of bytes into an instance of T using the default
                                                            configuration.

*/
// 传入引用，故需标注生命周期
pub fn my_deserialize<'a, T>(bytes: &'a [u8]) -> T
    where
        T: Deserialize<'a>,
{
    let deserialize = bincode::deserialize(bytes).unwrap();
    deserialize
}

// 导入依赖
use crypto::digest::Digest;
use crypto::sha3::Sha3;

// 获取哈希
pub fn get_hash(value: &[u8]) -> String {
    let mut hasher = Sha3::sha3_256();
    hasher.input(value);
    hasher.result_str()
}

// 测试
#[derive(Debug,Serialize,Deserialize,PartialOrd, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

#[cfg(test)]
mod tests {
    /* 绝对路径：
       使用需先于 lib.rs 中使用" pub "导出即" pub mod util; "
     */
    // 表示使用"根目录(crate)"下的"util"模块内的"Pointer"结构体
    use crate::code::Point;
    use crate::code::{my_serialize, my_deserialize};

    // 使用相对路径
    // use super::Point;
    // use super::{my_serialize, my_deserialize};

    #[test]
    fn util_works() {
        let point = Point {
            x: 1,
            y: 1,
        };
        let ser = my_serialize(&point);
        let deser: Point = my_deserialize(&ser);
        assert_eq!(deser, point);
    }
}
