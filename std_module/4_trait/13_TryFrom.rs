
/*
0. Trait std::convert::TryFrom
    a. 源码
        // 文档:" https://doc.rust-lang.org/std/convert/trait.TryFrom.html "
        Trait std::convert::TryFrom
        [−]
        pub trait TryFrom<T> {
            type Error;
            fn try_from(value: T) -> Result<Self, Self::Error>;
        }
        [−]
        Simple and safe type conversions that may fail in a controlled way under some
        circumstances. It is the reciprocal of TryInto.

        This is useful when you are doing a type conversion that may trivially succeed
        but may also need special handling. For example, there is no way to convert an
        i64 into an i32 using the From trait, because an i64 may contain a value that
        an i32 cannot represent and so the conversion would lose data. This might be
        handled by truncating the i64 to an i32 (essentially giving the i64's value
        modulo i32::MAX) or by simply returning i32::MAX, or by some other method. The
        From trait is intended for perfect conversions, so the TryFrom trait informs
        the programmer when a type conversion could go bad and lets them decide how to
        handle it.

        // Trait 相对 : " Trait std::convert::TryFrom "与" Trait std::convert::TryInto "

    b.

*/