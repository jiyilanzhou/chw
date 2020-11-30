
/*
1. 执行" cargo run/build "命令默认生成" debug "版本：
    a. 会在项目下 src 同级目录生成" target/debug(调试) "目录
    b. 执行" cargo run "命令即是编译后执行" target/debug/projectName.exe "文件

2. 执行" cargo run/build --release "命令可生成" release "版本：
    a. 会在项目下 src 同级目录生成" targe/release(释放) "及" target/debug(调试) "目录
    b. 执行" cargo run --release "命令即是编译后执行"target/release/projectName.exe"

3. " Cargo.toml "文件
    a. 工程配置文件其" [dependencies] "下即是配置相应依赖如使用的外部库
    b. 编译优化配置如" Cargo.toml "内部分默认配置
        [profile.dev]           # " debug "版
         opt-level = 0          # 优化级别
        [profile.release]       # " release "版
         opt-level = 3          # 优化级别越高编译耗时越长

*/
fn main() {
    println!("Hello, world!");
}
