
/*
0. Trait std::borrow::Borrow
    a. 源码
        // 文档:" https://doc.rust-lang.org/std/borrow/trait.Borrow.html "
        Trait std::borrow::Borrow
        [−]
        pub trait Borrow<Borrowed>
        where
            Borrowed: ?Sized,
        {
            fn borrow(&self) -> &Borrowed;
        }
        [−]
        A trait for borrowing data.

        In Rust, it is common to provide different representations of a type for different use
        cases. For instance, storage location and management for a value can be specifically chosen
        as appropriate for a particular use via pointer types such as Box<T> or Rc<T>. Beyond these
        generic wrappers that can be used with any type, some types provide optional facets
        providing potentially costly functionality. An example for such a type is String which adds
        the ability to extend a string to the basic str. This requires keeping additional
        information unnecessary for a simple, immutable string.

        These types provide access to the underlying data through references to the type of that
        data. They are said to be ‘borrowed as’ that type. For instance, a Box<T> can be borrowed as
        T while a String can be borrowed as str.

        Types express that they can be borrowed as some type T by implementing Borrow<T>, providing
        a reference to a T in the trait’s borrow method. A type is free to borrow as several
        different types. If it wishes to mutably borrow as the type – allowing the underlying data
        to be modified, it can additionally implement BorrowMut<T>.

        Further, when providing implementations for additional traits, it needs to be considered
        whether they should behave identical to those of the underlying type as a consequence of
        acting as a representation of that underlying type. Generic code typically uses Borrow<T>
        when it relies on the identical behavior of these additional trait implementations. These
        traits will likely appear as additional trait bounds.

        In particular Eq, Ord and Hash must be equivalent for borrowed and owned values:
        x.borrow() == y.borrow() should give the same result as x == y.

        If generic code merely needs to work for all types that can provide a reference to related
        type T, it is often better to use AsRef<T> as more types can safely implement it.

    b.




 */