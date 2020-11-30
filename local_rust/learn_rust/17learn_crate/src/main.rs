
// Rust 语言中所有模块及其内函数皆默认为"私有"
mod factory {
    pub mod produce_refrigerator {
        pub fn produce_re() {
            println!("produce refrigerator!");
        }
    }

    mod produce_washing_machine {
        fn produce_washing() {
            println!("produce washing machine!");
        }
    }

}

fn main() {
    // 使用其须声明为公有" pub "否则报错
    factory::produce_refrigerator::produce_re();
    println!("Hello, world!");
}
