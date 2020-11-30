
/*
(数组常规操作)Trait Implementations
    文档:" https://doc.rust-lang.org/std/primitive.array.html "

7. 实现 Trait std::cmp::Eq
    a. 实现 Eq 源码
        // NOTE: some less important impls are omitted to reduce code bloat
        // __impl_slice_eq2! { [A; $N], &'b [B; $N] }
        // __impl_slice_eq2! { [A; $N], &'b mut [B; $N] }

        #[stable(feature = "rust1", since = "1.0.0")]
        impl<T: Eq, const N: usize> Eq for [T; N] where [T; N]: LengthAtMost32 {}

    b. 相关 Eq Trait 源码(可参阅"4_trait\7_Eq.rs")：
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

*/

fn main_0() {
    enum BookFormat { Paperback, Hardback, Ebook }
    struct Book {
        isbn: i32,
        format: BookFormat,
    }
    impl PartialEq for Book {
        fn eq(&self, other: &Self) -> bool {
            self.isbn == other.isbn
        }
    }
    impl Eq for Book {}
}

/*
8. 实现 Trait std::hash::Hash
    a. 实现 Hash 源码
        #[stable(feature = "rust1", since = "1.0.0")]
        impl<T: Hash, const N: usize> Hash for [T; N]
        where
            [T; N]: LengthAtMost32,
        {
            fn hash<H: hash::Hasher>(&self, state: &mut H) {
                Hash::hash(&self[..], state)
            }
        }

    b. 相关 Hash 源码(可参阅"4_trait\8_Hash.rs")：
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

*/
use std::hash::{Hash, Hasher};
use std::convert::TryInto;

fn main_1() {
    // 方式 1 : 使用默认实现
    #[derive(Hash)]
    struct Rustacean {
        name: String,
        country: String,
    }

    // 方式 2 ：自定义实现
    use std::hash::{Hash, Hasher};
    struct Person {
        id: u32,
        name: String,
        phone: u64,
    }
    impl Hash for Person {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.id.hash(state);
            self.phone.hash(state);
        }
    }

}