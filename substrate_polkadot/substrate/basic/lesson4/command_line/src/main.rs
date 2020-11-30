
mod command;
mod initial;
use command::{cli,Cmd_Line};
use initial::Initialize;

fn main() {
    // 将命令行参数转换为结构体对象
    let cmd = Cmd_Line::new();
    cli::ownship_transfer(cmd);
    // println!("{}", cmd); // 所有权已转移，不可再用

    /* 执行效果：
         0. 终端输入" $ cargo run -- CLI pound 50 "
            输出" Accepte CLI agrs is : Pound    50 "
         1. 终端输入" $ cargo run -- CLI sparkle 3 12 "
            输出"" Accepte CLI agrs is : Sparkle   3   12  "
         2. 终端输入" $ cargo run -- CLI finish 130 "
            输出" Accepte CLI agrs is : Finish  Work   130 "
     */
}
