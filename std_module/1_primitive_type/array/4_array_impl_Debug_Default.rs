
/*
(数组常规操作)Trait Implementations
    文档:" https://doc.rust-lang.org/std/primitive.array.html "

5. 实现 Trait std::fmt::Debug
    a. 实现 Debug 源码
        #[stable(feature = "rust1", since = "1.0.0")]
        impl<T: fmt::Debug, const N: usize> fmt::Debug for [T; N]
        where
            [T; N]: LengthAtMost32,
        {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt::Debug::fmt(&&self[..], f)
            }
        }

    b. 相关 Debug Trait 源码(可参阅"4_trait\5_Debug.rs")：
        // 文档:" https://doc.rust-lang.org/std/fmt/trait.Debug.html "
        Trait std::fmt::Debug
        [−]
        pub trait Debug {
            fn fmt(&self, f: &mut Formatter) -> Result<(), Error>;
        }
        [−]
        ? formatting.

        Debug should format the output in a programmer-facing, debugging context.

        Generally speaking, you should just derive a Debug implementation.

        When used with the alternate format specifier #?, the output is pretty-printed.

        For more information on formatters, see the module-level documentation.

        This trait can be used with #[derive] if all fields implement Debug. When derived for
        structs, it will use the name of the struct, then {, then a comma-separated list of each
        field's name and Debug value, then }. For enums, it will use the name of the variant and, if
        applicable, (, then the Debug values of the fields, then ).

*/

fn main_0() {
    let arr = ['c', 'h', 'w'];
    println!("{:#?}", arr);
}

/*
6. 实现 Trait std::default::Default
    a. 实现 Default 源码
        // The Default impls cannot be generated using the array_impls! macro because
        // they require array literals.
        macro_rules! array_impl_default {
            {$n:expr, $t:ident $($ts:ident)*} => {
                #[stable(since = "1.4.0", feature = "array_default")]
                impl<T> Default for [T; $n] where T: Default {            // src_start --
                    fn default() -> [T; $n] {                             //            |
                        [$t::default(), $($ts::default()),*]              //            |
                    }                                                     //            |
                }                                                         //  src_end --
                array_impl_default!{($n - 1), $($ts)*}
            };
            {$n:expr,} => {
                #[stable(since = "1.4.0", feature = "array_default")]
                impl<T> Default for [T; $n] {
                    fn default() -> [T; $n] { [] }
                }
            };
        }

        array_impl_default! {32, T T T T T T T T T T T T T T T T T T T T T T T T T T T T T T T T}

    b. 相关 Default Trait 源码(可参阅"4_trait\6_Default.rs")：
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

*/

#[derive(Default, Debug)]
struct SomeOptions {
    foo: i32,
    bar: f32,
}

fn main() {
    /* Rust 为各种原生类型皆实现了默认值
          因" Default::default(); "为空参故需要指定接收类型
          " Default::default(); "实现原理：待理解[???]
     */
    let options: SomeOptions = Default::default();
    println!("{:?}", options); // Console:" SomeOptions { foo: 0, bar: 0.0 } "
    // 使用部分默认值
    let options = SomeOptions {
        foo: 42,
        /*[自]此处" Default::default(); "实现原理：
              因字段已指定原生类型故" Default::default(); "可为原生类型赋于默认值.
              问题：针对非原生类型如何使用" Default::default(); "自定义处理[???]
         */
        ..Default::default()
    };
    println!("{:?}", options);  // Console:" SomeOptions { foo: 42, bar: 0.0 } "
}