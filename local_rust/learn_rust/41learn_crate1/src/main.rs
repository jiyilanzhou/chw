
/*
使用外部库：
  a. " Cargo.toml "内" [dependencies] "可配置依赖的外部包如
        [dependencies]
        rust-crypto = "0.2"
   b. 按需使用 use 导入外部包如
        use crypto::digest::Digest;
        use crypto::sha3::Sha3;

*/
use crypto::digest::Digest;
use crypto::sha3::Sha3;

fn main() {
    let mut hasher = Sha3::sha3_256();
    hasher.input_str("Hello world!");
    let result = hasher.result_str();
    println!("result: {}", result);
}