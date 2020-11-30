
fn main() {
    // 读文件 ： 读取到 Vec<u8>
    let content = std::fs::read("test/input_file");
    println!(" content = {:?}", content);
    // 读文件 ： 读取到 String
    let content = std::fs::read_to_string("test/input_file");
    println!(" content = {:?}", content);
    // 读目录
    visit_dir(std::path::Path::new("test")).unwrap()

}

// 读取目录
fn visit_dir(dir: &std::path::Path) -> std::io::Result<()> {
    if dir.is_dir() {
        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dir(&path);
            } else {
                let content = std::fs::read_to_string(path).unwrap();
                println!("The File Content = \n {}", content);
            }
        }
    }
    Ok(())
}