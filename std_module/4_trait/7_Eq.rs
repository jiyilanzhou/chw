
/*
0. Trait std::cmp::Eq
    a. 源码
        // 文档:" https://doc.rust-lang.org/std/cmp/trait.Eq.html "
        Trait std::cmp::Eq
        [−]
        pub trait Eq: PartialEq<Self> { }
        [−]
        Trait for equality comparisons which are equivalence relations.

        This means, that in addition to a == b and a != b being strict inverses, the equality must
        be (for all a, b and c):

        reflexive: a == a;
        symmetric: a == b implies b == a; and
        transitive: a == b and b == c implies a == c.
        This property cannot be checked by the compiler, and therefore Eq implies PartialEq, and has
        no extra methods.

    b.

*/