
/*
0. T Trait
    a. 源码
        // 文档:" https://doc.rust-lang.org/std/cmp/trait.PartialOrd.html "
        Trait std::cmp::PartialOrd
        [−]
            [−]
            #[lang = "partial_ord"]
            pub trait PartialOrd<Rhs = Self>: PartialEq<Rhs>
            where
                Rhs: ?Sized,
            {
            [−]
            #[must_use]
            fn partial_cmp(&self, other: &Rhs) -> Option<Ordering>;

            [−]
            #[must_use]
            fn lt(&self, other: &Rhs) -> bool { ... }
            [−]
            #[must_use]
            fn le(&self, other: &Rhs) -> bool { ... }
            [−]
            #[must_use]
            fn gt(&self, other: &Rhs) -> bool { ... }
            [−]
            #[must_use]
            fn ge(&self, other: &Rhs) -> bool { ... }
        }
        [−]
        Trait for values that can be compared for a sort-order.

        The comparison must satisfy, for all a, b and c:

            asymmetry: if a < b then !(a > b), as well as a > b implying !(a < b); and
            transitivity: a < b and b < c implies a < c. The same must hold for both == and >.

        Note that these requirements mean that the trait itself must be implemented symmetrically
        and transitively: if T: PartialOrd<U> and U: PartialOrd<V> then U: PartialOrd<T> and
        T: PartialOrd<V>.

    b.

*/
