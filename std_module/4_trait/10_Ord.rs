
/*
0. Trait std::cmp::Ord
    a. 源码
        // 文档:" https://doc.rust-lang.org/std/cmp/trait.Ord.html "
        Trait std::cmp::Ord
        [−]
        pub trait Ord: Eq + PartialOrd<Self> {
            [−]
            #[must_use]
                fn cmp(&self, other: &Self) -> Ordering;

            [−]
            #[must_use]
                fn max(self, other: Self) -> Self { ... }
            [−]
            #[must_use]
                fn min(self, other: Self) -> Self { ... }
            [−]
            #[must_use]
                fn clamp(self, min: Self, max: Self) -> Self { ... }
        }

        [−]
        Trait for types that form a total order.

        An order is a total order if it is (for all a, b and c):

            total and asymmetric: exactly one of a < b, a == b or a > b is true; and
            transitive, a < b and b < c implies a < c. The same must hold for both == and >.

    b.

*/
