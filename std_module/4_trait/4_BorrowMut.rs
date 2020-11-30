
/*
0. Trait std::borrow::BorrowMut
    a. 源码
        // 文档:" https://doc.rust-lang.org/std/borrow/trait.BorrowMut.html "
        Trait std::borrow::BorrowMut
        [−]
        pub trait BorrowMut<Borrowed>: Borrow<Borrowed>
        where
            Borrowed: ?Sized,
        {
            fn borrow_mut(&mut self) -> &mut Borrowed;
        }
        [−]
        A trait for mutably borrowing data.

        As a companion to Borrow<T> this trait allows a type to borrow as an underlying type by
        providing a mutable reference. See Borrow<T> for more information on borrowing as another
        type.

    b.

*/