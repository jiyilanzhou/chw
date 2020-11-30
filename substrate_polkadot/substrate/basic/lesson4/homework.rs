
/*
0. 查看第三方文档库
    a. " Cargo.toml "下添加第三方库依赖
    b. 执行" cargo doc --open "
    c. 参照文档具体使用实例(或可尝试 github 上搜索、或百度 StructOpt 用法)

1. structopt
    一个通过结构体来解析命令行参数。可以说它对 clap 库进行补充

2. 子命令
    参阅" command_line crate "(提交的作业)

 */
use structopt::StructOpt;
#[derive(StructOpt, Debug)]
#[structopt(name = "example", about = "An example of StructOpt usage.")]
struct Opt {
    #[structopt(short, long, default_value = "42")]
    speed: f64,
    input: String,
    output: Option<String>,
}

fn main_0() {
    // 将参数解析为结构体
    let opt = Opt::from_args();

    /* 命令：
       a. " $ cargo run -- args "
            输出" Opt { speed: 42.0, input: "args", output: None } "
       b." $ cargo run -- -- -abc "
            输出" Opt { speed: 42.0, input: "-abc", output: None }
     */
    println!("{:?}", opt);
}





