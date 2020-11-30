
/*
0. 读取文件
    a. 读取单一文件
    b. 读取目录

*/

use std::{fs, path::Path};

fn main() {
    // " fs::read "将内容读入" Vec<u8> "容器
    // 文件位置 : 基于项目而非"main.rs"
    let context = fs::read("file").unwrap();
    //println!("{:#?}",context); // 输出字节
    println!("{:#?}", String::from_utf8_lossy(&context));

    // fs::read_to_string 直接读取到 String
    println!("{:?}", fs::read_to_string("./file").unwrap());

    // 读取 目录
    visit_dir(Path::new("./src")).unwrap();
}

// 读取目录
fn visit_dir(dir: &Path) -> std::io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dir(&path);
            } else {
                let c = fs::read_to_string(path);
                println!("file = {:?}", c);
            }
        }
    }
    Ok(())
}