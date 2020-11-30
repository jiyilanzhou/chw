
/*
(数组常规操作)Trait Implementations
    文档:" https://doc.rust-lang.org/std/primitive.array.html "

Primitive Type array
    [−]
    A fixed-size array, denoted [T; N], for the element type, T, and the non-negative compile-time
    constant size, N.

    There are two syntactic forms for creating an array:

    A list with each element, i.e., [x, y, z].
    A repeat expression [x; N], which produces an array with N copies of x. The type of x must be
    Copy.
    Arrays of sizes from 0 to 32 (inclusive) implement the following traits if the element type
    allows it:

    Debug
    IntoIterator (implemented for &[T; N] and &mut [T; N])
    PartialEq, PartialOrd, Eq, Ord
    Hash
    AsRef, AsMut
    Borrow, BorrowMut
    Default
    This limitation on the size N exists because Rust does not yet support code that is generic over
    the size of an array type. [Foo; 3] and [Bar; 3] are instances of same generic type [T; 3], but
    [Foo; 3] and [Foo; 5] are entirely different types. As a stopgap, trait implementations are
    statically generated up to size 32.

    Arrays of any size are Copy if the element type is Copy and Clone if the element type is Clone.
    This works because Copy and Clone traits are specially known to the compiler.

    Arrays coerce to slices ([T]), so a slice method may be called on an array. Indeed, this
    provides most of the API for working with arrays. Slices have a dynamic size and do not coerce
    to arrays.

    You can move elements out of an array with a slice pattern. If you want one element, see
    mem::replace.

    Examples
    let mut array: [i32; 3] = [0; 3];
    array[1] = 1;
    array[2] = 2;
    assert_eq!([1, 2], &array[1..]);
    // This loop prints: 0 1 2
    for x in &array {
        print!("{} ", x);
    }
    Run

    // An array itself is not iterable:
    ⓘ
    let array: [i32; 3] = [0; 3];
    for x in array { }
    // error: the trait bound `[i32; 3]: std::iter::Iterator` is not satisfied
    Run

    // The solution is to coerce the array to a slice by calling a slice method:
    for x in array.iter() { }
    Run

    // If the array has 32 or fewer elements (see above), you can also use the array reference's
    // IntoIterator implementation:
    for x in &array { }
    Run

    // You can use a slice pattern to move elements out of an array:
    fn move_away(_: String) { /* Do interesting things. */ }
    let [john, roa] = ["John".to_string(), "Roa".to_string()];
    move_away(john);
    move_away(roa);

0. 相关 Trait std::array::LengthAtMost32
    a. 相关 LengthAtMost32 Trait 源码(可参阅"4_trait\0_LengthAtMost32.rs")：
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

/*
1. 实现 Trait std::convert::AsMut
    a. 实现 AsMut 源码
        #[stable(feature = "rust1", since = "1.0.0")]
        impl<T, const N: usize> AsMut<[T]> for [T; N]
        where
            [T; N]: LengthAtMost32,
        {
            #[inline]
            fn as_mut(&mut self) -> &mut [T] {
                &mut self[..]
            }
        }

    b. 相关 AsMut Trait 源码(可参阅"4_trait\1_AsMut.rs")：
        // 文档:" https://doc.rust-lang.org/std/convert/trait.AsMut.html "
        Trait std::convert::AsMut
        [−]
        pub trait AsMut<T>
        where
            T: ?Sized,
        {
            fn as_mut(&mut self) -> &mut T;
        }
        [−]
        Used to do a cheap mutable-to-mutable reference conversion.

        This trait is similar to AsRef but used for converting between mutable references. If you
        need to do a costly conversion it is better to implement From with type &mut T or write a
        custom function.

        Note: This trait must not fail. If the conversion can fail, use a dedicated method which
        returns an Option<T> or a Result<T, E>.

 */

fn main_1() {
    // 声明
    let mut arr = [1, 2, 3];
    // 普通修改
    arr[0] = 0;
    /* 可变到可变引用的转换
        " let a = arr.as_mut() "相当于" let b = &mut arr; "或
        " let ref mut c = arr; "
     */
    // let a = arr.as_mut();
    // let a =&mut arr;
    let ref mut a = arr;
    (*a)[0] = 1;
    // 数组(可变)引用已实现 IntoIterator 故可直接遍历
    for i in a {
        *i += 3;
    }
    println!("{:?}", arr);   // Console:" [4, 5, 6] "
}

/*
2. 实现 Trait std::convert::AsRef
    a. 实现 AsRef 源码
        #[stable(feature = "rust1", since = "1.0.0")]
        impl<T, const N: usize> AsRef<[T]> for [T; N]
        where
            [T; N]: LengthAtMost32,
        {
            #[inline]
            fn as_ref(&self) -> &[T] {
                &self[..]
            }
        }

    b. 相关 AsRef Trait 源码(可参阅"4_trait\2_AsRef.rs")：
        // 文档:" https://doc.rust-lang.org/std/convert/trait.AsRef.html "
        Trait std::convert::AsRef
        [−]
        pub trait AsRef<T>
        where
            T: ?Sized,
        {
            fn as_ref(&self) -> &T;
        }
        [−]
        Used to do a cheap reference-to-reference conversion.

        This trait is similar to AsMut which is used for converting between mutable references. If
        you need to do a costly conversion it is better to implement From with type &T or write a
        custom function.

        AsRef has the same signature as Borrow, but Borrow is different in few aspects:

            Unlike AsRef, Borrow has a blanket impl for any T, and can be used to accept either a
            reference or a value.

            Borrow also requires that Hash, Eq and Ord for borrowed value are equivalent to those of
            the owned value. For this reason, if you want to borrow only a single field of a struct
            you can implement AsRef, but not Borrow.

        Note: This trait must not fail. If the conversion can fail, use a dedicated method which
        returns an Option<T> or a Result<T, E>.

    c. AsRef 与 Borrow 的区别[??]

*/

fn main() {
    // 声明
    let arr = [1, 2, 3];
    /* 可变到可变引用的转换
        " let a = arr.as_ref() "相当于" let b = &arr; "或
        " let ref c = arr; "
     */
    let a = arr.as_ref();
    //let a = &arr;
    //let ref a = arr;
    // 数组引用已实现 IntoIterator 故可直接遍历
    for i in a {
        print!("{}\t", i);   // Console:" 1	2 3 "
    }
}


/*
// 部分源码赏析：
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

 */

