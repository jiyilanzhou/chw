
/*
0. Trait std::cmp::PartialEq
    a. 源码
        // 文档:" https://doc.rust-lang.org/std/cmp/trait.PartialEq.html "
        Trait std::cmp::PartialEq
        [−]
            [−]
            #[lang = "eq"]
            pub trait PartialEq<Rhs = Self>
            where
                Rhs: ?Sized,
            {
            [−]
            #[must_use]
            fn eq(&self, other: &Rhs) -> bool;

            [−]
            #[must_use]
            fn ne(&self, other: &Rhs) -> bool { ... }
        }
        [−]
        Trait for equality comparisons which are partial equivalence relations.

        This trait allows for partial equality, for types that do not have a full equivalence
        relation. For example, in floating point numbers NaN != NaN, so floating point types
        implement PartialEq but not Eq.

        Formally, the equality must be (for all a, b and c):

            symmetric: a == b implies b == a; and
            transitive: a == b and b == c implies a == c.

        Note that these requirements mean that the trait itself must be implemented symmetrically
        and transitively: if T: PartialEq<U> and U: PartialEq<V> then U: PartialEq<T> and
        T: PartialEq<V>.

    b.

*/