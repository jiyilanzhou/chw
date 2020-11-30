
/*
0. (数组常规操作)Trait Implementations
    文档:" https://doc.rust-lang.org/std/primitive.array.html "
13. 实现 Trait std::cmp::PartialOrd
    a. 实现 PartialOrd 源码
        #[stable(feature = "rust1", since = "1.0.0")]
        impl<T: PartialOrd, const N: usize> PartialOrd for [T; N]
        where
            [T; N]: LengthAtMost32,
        {
            #[inline]
            fn partial_cmp(&self, other: &[T; N]) -> Option<Ordering> {
                PartialOrd::partial_cmp(&&self[..], &&other[..])
            }
            #[inline]
            fn lt(&self, other: &[T; N]) -> bool {
                PartialOrd::lt(&&self[..], &&other[..])
            }
            #[inline]
            fn le(&self, other: &[T; N]) -> bool {
                PartialOrd::le(&&self[..], &&other[..])
            }
            #[inline]
            fn ge(&self, other: &[T; N]) -> bool {
                PartialOrd::ge(&&self[..], &&other[..])
            }
            #[inline]
            fn gt(&self, other: &[T; N]) -> bool {
                PartialOrd::gt(&&self[..], &&other[..])
            }
        }

    b. 相关 PartialOrd 源码(可参阅"4_trait\12_PartialOrd.rs")：
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

*/
fn main_0() {
    use std::cmp::Ordering;

    struct Person {
        id: u32,
        name: String,
        height: f64,
    }

    impl PartialOrd for Person {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            self.height.partial_cmp(&other.height)
        }
    }

    impl PartialEq for Person {
        fn eq(&self, other: &Self) -> bool {
            self.height == other.height
        }
    }
}

/*
14. 实现 Trait std::convert::TryFrom
    a. 实现 TryFrom 源码(" TryFrom<&[T]> for [T; N] ")
        #[stable(feature = "try_from", since = "1.34.0")]
        impl<T, const N: usize> TryFrom<&[T]> for [T; N]
        where
            T: Copy,
            [T; N]: LengthAtMost32,
        {
            type Error = TryFromSliceError;

            fn try_from(slice: &[T]) -> Result<[T; N], TryFromSliceError> {
                <&Self>::try_from(slice).map(|r| *r)
            }
        }

    b. 实现 TryFrom 源码(" TryFrom<&'a [T]> for &'a [T; N] ")
        #[stable(feature = "try_from", since = "1.34.0")]
        impl<'a, T, const N: usize> TryFrom<&'a [T]> for &'a [T; N]
        where
            [T; N]: LengthAtMost32,
        {
            type Error = TryFromSliceError;

            fn try_from(slice: &[T]) -> Result<&[T; N], TryFromSliceError> {
                if slice.len() == N {
                    let ptr = slice.as_ptr() as *const [T; N];
                    // SAFETY: ok because we just checked that the length fits
                    unsafe { Ok(&*ptr) }
                } else {
                    Err(TryFromSliceError(()))
                }
            }
        }

    c. 实现 TryFrom 源码(" TryFrom<&'a mut [T]> for &'a mut [T; N] ")
        #[stable(feature = "try_from", since = "1.34.0")]
        impl<'a, T, const N: usize> TryFrom<&'a mut [T]> for &'a mut [T; N]
        where
            [T; N]: LengthAtMost32,
        {
            type Error = TryFromSliceError;

            fn try_from(slice: &mut [T]) -> Result<&mut [T; N], TryFromSliceError> {
                if slice.len() == N {
                    let ptr = slice.as_mut_ptr() as *mut [T; N];
                    // SAFETY: ok because we just checked that the length fits
                    unsafe { Ok(&mut *ptr) }
                } else {
                    Err(TryFromSliceError(()))
                }
            }
        }

    b. 相关 TryFrom 源码(可参阅"4_trait\13_TryFrom.rs")：
        // 文档:" https://doc.rust-lang.org/std/convert/trait.TryFrom.html "
        Trait std::convert::TryFrom
        [−]
        pub trait TryFrom<T> {
            type Error;
            fn try_from(value: T) -> Result<Self, Self::Error>;
        }
        [−]
        Simple and safe type conversions that may fail in a controlled way under some
        circumstances. It is the reciprocal of TryInto.

        This is useful when you are doing a type conversion that may trivially succeed
        but may also need special handling. For example, there is no way to convert an
        i64 into an i32 using the From trait, because an i64 may contain a value that
        an i32 cannot represent and so the conversion would lose data. This might be
        handled by truncating the i64 to an i32 (essentially giving the i64's value
        modulo i32::MAX) or by simply returning i32::MAX, or by some other method. The
        From trait is intended for perfect conversions, so the TryFrom trait informs
        the programmer when a type conversion could go bad and lets them decide how to
        handle it.

        // Trait 相对 : " Trait std::convert::TryFrom "与" Trait std::convert::TryInto "

*/

fn main() {
    /* 源码：
        #[stable(feature = "try_from", since = "1.34.0")]
        impl<T, const N: usize> TryFrom<&[T]> for [T; N]
            where
                T: Copy,
        {
            type Error = TryFromSliceError;

            fn try_from(slice: &[T]) -> Result<[T; N], TryFromSliceError> {
                /* [自]此处" <&Self> "应是强制为切片后从而调用切片" try_from "
                      函数而非数组本身递归调用(待查证[??])
                */
                <&Self>::try_from(slice).map(|r| *r)
            }
        }
        // 问题："<&Self>"是什么用法[??] 以及如何调用
     */
    // 调用 trait 方法需将相应 Trait 引入作用域(" try_from "为关联函数[并非方法])
    use std::convert::TryFrom;
    let sli = [0;3];
    // " try_from "为关联函数[并非方法]
    // let arr = TryFrom::try_from();
    //let arr = TryFrom::try_from(sli); // try_from 如何显式指定类型[??]
    println!("{:?}",arr);
    // let arr = <&sli>::try_from(); // 如何使用 turbofish 字符串[??]
    // println!("{:?}",arr);

}