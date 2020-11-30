use structopt::StructOpt;
use core::fmt;
use std::fmt::Formatter;
use crate::initial::Initialize;

#[derive(StructOpt)]
pub struct Cmd_Line {
    tree: Option<String>,
    #[structopt(subcommand)]
    sub_cmd: Command,
}

impl Initialize for Cmd_Line {
    type output = Cmd_Line;
    fn new() -> Self::output {
        Cmd_Line::from_args()
    }
}

// 自定义实现 Cmd_Line 的 Display 显示样式
impl fmt::Display for Cmd_Line {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self.sub_cmd {
            Command::Pound { acorns } => {
                write!(f, "{}\t{}", "Accepte CLI agrs is : Pound", acorns);
                Ok(())
            }
            Command::Sparkle { magicality, color } => {
                write!(f, "{}\t{}\t{}", "Accepte CLI agrs is : Sparkle", magicality, color);
                Ok(())
            }
            Command::Finish(work) => {
                write!(f, "{}\t{}", "Accepte CLI agrs is : Finish  Work", work.typ);
                Ok(())
            }
            _ => Ok(())
        }
    }
}

#[derive(StructOpt)]
enum Command {
    Pound {
        acorns: u32
    },
    Sparkle {
        magicality: u64,
        color: String,
    },
    Finish(Work),
}

#[derive(StructOpt)]
struct Work {
    typ: String,
}

pub fn ownship_transfer(cli: Cmd_Line) {
    println!("{}", cli);
}
