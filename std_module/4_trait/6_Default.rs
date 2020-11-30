/*
0. Trait std::default::Default
    a. 源码
        // 文档:" https://doc.rust-lang.org/std/default/trait.Default.html "
        Trait std::default::Default
        [−]
        pub trait Default {
            fn default() -> Self;
        }
        [−]
        A trait for giving a type a useful default value.

        Sometimes, you want to fall back to some kind of default value, and don't particularly care
        what it is. This comes up often with structs that define a set of options:
            struct SomeOptions {
                foo: i32,
                bar: f32,
            }
            Run

        How can we define some default values? You can use Default:
            #[derive(Default)]
            struct SomeOptions {
                foo: i32,
                bar: f32,
            }
            fn main() {
                let options: SomeOptions = Default::default();
            }
            Run

        Now, you get all of the default values. Rust implements Default for various primitives types.
        If you want to override a particular option, but still retain the other defaults:
            fn main() {
                let options = SomeOptions { foo: 42, ..Default::default() };
            }
            Run

    b.

*/