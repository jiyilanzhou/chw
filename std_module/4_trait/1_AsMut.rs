
/*
0. Trait std::convert::AsMut
   a. 源码
        // 文档:" https://doc.rust-lang.org/std/convert/trait.AsMut.html "
        Trait std::convert::AsMut
        [−]
        pub trait AsMut<T>
        where
            T: ?Sized,
        {
            fn as_mut(&mut self) -> &mut T;
        }
        [−]
        Used to do a cheap mutable-to-mutable reference conversion.

        This trait is similar to AsRef but used for converting between mutable references. If you
        need to do a costly conversion it is better to implement From with type &mut T or write a
        custom function.

        Note: This trait must not fail. If the conversion can fail, use a dedicated method which
        returns an Option<T> or a Result<T, E>.

    b.


 */