
/*
0. Trait std::fmt::Debug
    a. 源码
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

    b.

 */