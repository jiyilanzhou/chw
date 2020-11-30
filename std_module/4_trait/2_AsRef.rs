
/*
0. Trait std::convert::AsRef
    a. 源码
        // 文档:" https://doc.rust-lang.org/std/convert/trait.AsRef.html "
        Trait std::convert::AsRef
        [−]
        pub trait AsRef<T>
        where
            T: ?Sized,
        {
            fn as_ref(&self) -> &T;
        }
        [−]
        Used to do a cheap reference-to-reference conversion.

        This trait is similar to AsMut which is used for converting between mutable references. If
        you need to do a costly conversion it is better to implement From with type &T or write a
        custom function.

        AsRef has the same signature as Borrow, but Borrow is different in few aspects:

            Unlike AsRef, Borrow has a blanket impl for any T, and can be used to accept either a
            reference or a value.

            Borrow also requires that Hash, Eq and Ord for borrowed value are equivalent to those of
            the owned value. For this reason, if you want to borrow only a single field of a struct
            you can implement AsRef, but not Borrow.

        Note: This trait must not fail. If the conversion can fail, use a dedicated method which
        returns an Option<T> or a Result<T, E>.

    b.


 */