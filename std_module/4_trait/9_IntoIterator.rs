
/*
0. Trait std::iter::IntoIterator
    a. 源码
        // 文档:" https://doc.rust-lang.org/std/iter/trait.IntoIterator.html "
        Trait std::iter::IntoIterator
        [−]
        pub trait IntoIterator
        where
            <Self::IntoIter as Iterator>::Item == Self::Item,
        {
            type Item;
            type IntoIter: Iterator;
            fn into_iter(self) -> Self::IntoIter;
        }
        [−]
        Conversion into an Iterator.

        By implementing IntoIterator for a type, you define how it will be converted to an iterator.
        This is common for types which describe a collection of some kind.

        One benefit of implementing IntoIterator is that your type will work with Rust's for loop
        syntax.

        See also: FromIterator.

        // Trait 相对 : " Trait std::iter::IntoIterator "与" Trait std::iter::FromIterator "

    b.

*/