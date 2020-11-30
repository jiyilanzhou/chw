
/*
(数组常规操作)Trait Implementations
    文档:" https://doc.rust-lang.org/std/primitive.array.html "

9. 实现 Trait std::iter::IntoIterator
    a. 实现 IntoIterator 源码( "  for &'a [T; N] " )
        #[stable(feature = "rust1", since = "1.0.0")]
        impl<'a, T, const N: usize> IntoIterator for &'a [T; N]
        where
            [T; N]: LengthAtMost32,
        {
            type Item = &'a T;
            type IntoIter = Iter<'a, T>;

            fn into_iter(self) -> Iter<'a, T> {
                self.iter()
            }
        }

    b. 实现 IntoIterator 源码( " for &'a mut [T; N] " )
        #[stable(feature = "rust1", since = "1.0.0")]
        impl<'a, T, const N: usize> IntoIterator for &'a mut [T; N]
        where
            [T; N]: LengthAtMost32,
        {
            type Item = &'a mut T;
            type IntoIter = IterMut<'a, T>;

            fn into_iter(self) -> IterMut<'a, T> {
                self.iter_mut()
            }
        }

    c. 相关 IntoIterator Trait 源码(可参阅"4_trait\9_IntoIterator.rs")：
        // 文档:" https://doc.rust-lang.org/std/iter/trait.IntoIterator.html "
        Trait std::iter::IntoIterator
        [−]
        pub trait IntoIterator
        where
            <Self::IntoIter as Iterator>::Item == Self::Item,
        {
            type Item;
            type IntoIter: Iterator;
            fn into_iter(self) -> Self::IntoIter;
        }
        [−]
        Conversion into an Iterator.

        By implementing IntoIterator for a type, you define how it will be converted to an iterator.
        This is common for types which describe a collection of some kind.

        One benefit of implementing IntoIterator is that your type will work with Rust's for loop
        syntax.

        See also: FromIterator.

        // Trait 相对 : " Trait std::iter::IntoIterator "与" Trait std::iter::FromIterator "

*/

fn main() {
    let mut array: [i32; 3] = [0; 3];
    //  使用切片本身实现的 iter_mut 实现修改
    for  i in array.iter_mut() {
        print!("{}\t", i);       // Console:" 0	 0	0 "
        *i += 1
    }
    // 使用切片实现的 IntoIterator 之 into_iter 实现修改
    let mut arr = [0; 3];
    for i in (&mut arr[..]).into_iter(){
        *i+=1;
    }
    println!("{:?}",arr);

    // 使用数组引用的 IntoIterator 实现修改
    for i in (&mut array).into_iter() {
        print!("{}\t", i);       // Console:" 1	1 1 "
        *i += 1
    }
    // 使用数组引用的 IntoIterator 实现读取
    for i in (&array).into_iter() {
        print!("{}\t",i);       // Console:" 2 2 2 "
    }
}

/*
10. 实现 Trait std::array::LengthAtMost32
    a. 实现 LengthAtMost32 源码
        /// Implemented for lengths where trait impls are allowed on arrays in core/std
        #[rustc_on_unimplemented(message = "arrays only have std trait implementations for lengths
        0..=32")]
        #[unstable(
            feature = "const_generic_impls_guard",
            issue = "none",
            reason = "will never be stable, just a temporary step until const generics are stable"
        )]
        pub trait LengthAtMost32 {}

        macro_rules! array_impls {
            ($($N:literal)+) => {
                $(
                    #[unstable(feature = "const_generic_impls_guard", issue = "none")]
                    impl<T> LengthAtMost32 for [T; $N] {}
                )+
            }
        }

        array_impls! {
             0  1  2  3  4  5  6  7  8  9
            10 11 12 13 14 15 16 17 18 19
            20 21 22 23 24 25 26 27 28 29
            30 31 32
        }

    b. 相关 LengthAtMost32 Trait 源码(可参阅"4_trait\0_LengthAtMost32.rs")：
        // 文档:" https://doc.rust-lang.org/std/array/trait.LengthAtMost32.html "
        Trait std::array::LengthAtMost32
        [−]
        pub trait LengthAtMost32 { }
        🔬 This is a nightly-only experimental API. (const_generic_impls_guard)
        [−]
        Implemented for lengths where trait impls are allowed on arrays in core/std

*/
fn main_0() {
    // pub trait LengthAtMost32 {}          // 空 trait
}