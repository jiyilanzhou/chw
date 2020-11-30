
/*
0.  Trait std::hash::Hash
    a. 源码
        // 文档:" https://doc.rust-lang.org/std/hash/trait.Hash.html "
        Trait std::hash::Hash
        [−]
        pub trait Hash {
            fn hash<H>(&self, state: &mut H)
            where
                H: Hasher;

            fn hash_slice<H>(data: &[Self], state: &mut H)
            where
                H: Hasher,
            { ... }
        }
        [−]
        A hashable type.

        Types implementing Hash are able to be hashed with an instance of Hasher.

    b.

*/