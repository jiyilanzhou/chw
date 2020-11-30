
// clamp[klæmp]v&n.夹(钳)；螺丝钳
/*
(数组常规操作)Trait Implementations
    文档:" https://doc.rust-lang.org/std/primitive.array.html "

11. 实现 Trait std::cmp::Ord
    a. 实现 Ord 源码
        /// Implements comparison of arrays lexicographically.
        #[stable(feature = "rust1", since = "1.0.0")]
        impl<T: Ord, const N: usize> Ord for [T; N]
        where
            [T; N]: LengthAtMost32,
        {
            #[inline]
            fn cmp(&self, other: &[T; N]) -> Ordering {
                Ord::cmp(&&self[..], &&other[..])
            }
        }

    b. 相关 Ord 源码(可参阅"4_trait\10_Ord.rs")：
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

*/
fn main_0() {
    use std::cmp::Ordering;

    #[derive(Eq)]
    struct Person {
        id: u32,
        name: String,
        height: u32,
    }

    impl Ord for Person {
        fn cmp(&self, other: &Self) -> Ordering {
            self.height.cmp(&other.height)
        }
    }

    impl PartialOrd for Person {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    impl PartialEq for Person {
        fn eq(&self, other: &Self) -> bool {
            self.height == other.height
        }
    }
}


/*
12. 实现 Trait std::cmp::PartialEq
    a. 实现 PartialEq 源码 (" PartialEq<&'b [B]> for [A; N] ")
        #[stable(feature = "rust1", since = "1.0.0")]
        impl<'b, A, B, const N: usize> PartialEq<&'b [B]> for [A; N]
        where
            A: PartialEq<B>,
            [A; N]: LengthAtMost32,
        {
            #[inline]
            fn eq(&self, other: &&'b [B]) -> bool {
                self[..] == other[..]
            }
            #[inline]
            fn ne(&self, other: &&'b [B]) -> bool {
                self[..] != other[..]
            }
        }

    b. 实现 PartialEq 源码 (" PartialEq<&'b mut [B]> for [A; N] ")
        #[stable(feature = "rust1", since = "1.0.0")]
        impl<'b, A, B, const N: usize> PartialEq<&'b mut [B]> for [A; N]
        where
            A: PartialEq<B>,
            [A; N]: LengthAtMost32,
        {
            #[inline]
            fn eq(&self, other: &&'b mut [B]) -> bool {
                self[..] == other[..]
            }
            #[inline]
            fn ne(&self, other: &&'b mut [B]) -> bool {
                self[..] != other[..]
            }
        }

    c. 实现 PartialEq 源码 (" PartialEq<[B; N]> for [A; N] ")
        #[stable(feature = "rust1", since = "1.0.0")]
        impl<A, B, const N: usize> PartialEq<[B; N]> for [A; N]
        where
            A: PartialEq<B>,
            [A; N]: LengthAtMost32,
            [B; N]: LengthAtMost32,
        {
            #[inline]
            fn eq(&self, other: &[B; N]) -> bool {
                self[..] == other[..]
            }
            #[inline]
            fn ne(&self, other: &[B; N]) -> bool {
                self[..] != other[..]
            }
        }

    d. 实现 PartialEq 源码 (" PartialEq<[B]> for [A; N] ")
        #[stable(feature = "rust1", since = "1.0.0")]
        impl<A, B, const N: usize> PartialEq<[B]> for [A; N]
        where
            A: PartialEq<B>,
            [A; N]: LengthAtMost32,
        {
            #[inline]
            fn eq(&self, other: &[B]) -> bool {
                self[..] == other[..]
            }
            #[inline]
            fn ne(&self, other: &[B]) -> bool {
                self[..] != other[..]
            }
        }

    e. 相关 PartialEq Trait 源码(可参阅"4_trait\11_PartialEq.rs")：
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

*/
fn main_1() {
    enum BookFormat {
        Paperback,
        Hardback,
        Ebook,
    }
    struct Book {
        isbn: i32,
        format: BookFormat,
    }
    impl PartialEq for Book {
        fn eq(&self, other: &Self) -> bool {
            self.isbn == other.isbn
        }
    }
    let b1 = Book { isbn: 3, format: BookFormat::Paperback };
    let b2 = Book { isbn: 3, format: BookFormat::Ebook };
    let b3 = Book { isbn: 10, format: BookFormat::Paperback };
    assert!(b1 == b2);
    assert!(b1 != b3);
}