
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;
extern crate cc;

fn main() {
    // 获取并绑定输出目录
    let out_dir = env::var("OUT_DIR").unwrap();
    // 生成文件
    let dest_path = Path::new(&out_dir).join("say_hello.rs");
    let mut f = File::create(&dest_path).unwrap();
    // 写入数据
    f.write_all(b"
        fn say_hello() -> &'static str {
            \"hello\"
        }
    ").unwrap();

    // 使用 cc
    cc::Build::new().file("src/hello.c").compile("hello")
}


