
/*
0. 文件读取

*/

fn main() {
    // 读取文件
    // let path = "input.txt"; // 运行时错误:"系统找不到指定的文件"(待解决[?])
    let path = "E:\\project\\online_rust\\error_file\\input";
    // 方式 1 ：使用 file
    let mut content = String::new();
    //std::fs::File::open(path).unwrap().read_to_string(&mut content).unwrap();

    // 方式 2：使用 fs
    let content = std::fs::read_to_string(path);
    println!("{:?}", content);

}