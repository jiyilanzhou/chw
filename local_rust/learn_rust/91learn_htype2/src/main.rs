
/*
2、从不返回的 never type (特殊类型["!"):
    Rust 的特殊类型("!")，在类型理论术语中被称为 empty type (因其无值）
    但 Rust 更倾向于称之为" never type "(用于函数不返回时充当返回值)如:
        fn bar() -> ! {
            loop{}
        }

*/

/* 示例：源于《Rust程序设计语言》中第二章"猜猜看"游戏
        配置依赖于"... \91learn_htype2\Cargo.toml"如
           [dependencies]
           rand = "0.6.0"

*/
use std::io;
use std::cmp::Ordering;
use rand::Rng;
fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1, 101);
    loop {
        println!("Please input your guess.");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            /* never type (" ! ") :
               a. 当 Rust 检测 guess 的类型时查看分支(前者是 u32 值
                  而后者是 ! 值["continue"返回值])
               b. 比对分支决定 guess 的类型是 u32 (因 ! 并没有一个值)
            */
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

/* // " panic! "亦返回 never type (" ! "):
    impl<T> Option<T> { // Option<T> 上的 unwrap 函数代码
        pub fn unwrap(self) -> T {
            match self {
                Some(val) => val,
                None => panic!("called `Option::unwrap()` on a `None` value"),
            }
        }
    }

*/