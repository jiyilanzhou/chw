
// equi[ˈiːkwɪ]n. 等     // equivalent[ɪˈkwɪvələnt]adj&n.等价(的)
/*
(数组常规操作)Trait Implementations
    文档:" https://doc.rust-lang.org/std/primitive.array.html "

3. 实现 Trait std::borrow::Borrow
    a. 实现 Borrow 源码
        #[stable(feature = "array_borrow", since = "1.4.0")]
        impl<T, const N: usize> Borrow<[T]> for [T; N]
        where
            [T; N]: LengthAtMost32,
        {
            fn borrow(&self) -> &[T] {
                self
            }
        }

    b. 相关 Borrow Trait 源码(可参阅"4_trait\3_Borrow.rs")：
        // 文档:" https://doc.rust-lang.org/std/borrow/trait.Borrow.html "
        Trait std::borrow::Borrow
        [−]
        pub trait Borrow<Borrowed>
        where
            Borrowed: ?Sized,
        {
            fn borrow(&self) -> &Borrowed;
        }
        [−]
        A trait for borrowing data.

        In Rust, it is common to provide different representations of a type for different use
        cases. For instance, storage location and management for a value can be specifically chosen
        as appropriate for a particular use via pointer types such as Box<T> or Rc<T>. Beyond these
        generic wrappers that can be used with any type, some types provide optional facets
        providing potentially costly functionality. An example for such a type is String which adds
        the ability to extend a string to the basic str. This requires keeping additional
        information unnecessary for a simple, immutable string.

        These types provide access to the underlying data through references to the type of that
        data. They are said to be ‘borrowed as’ that type. For instance, a Box<T> can be borrowed as
        T while a String can be borrowed as str.

        Types express that they can be borrowed as some type T by implementing Borrow<T>, providing
        a reference to a T in the trait’s borrow method. A type is free to borrow as several
        different types. If it wishes to mutably borrow as the type – allowing the underlying data
        to be modified, it can additionally implement BorrowMut<T>.

        Further, when providing implementations for additional traits, it needs to be considered
        whether they should behave identical to those of the underlying type as a consequence of
        acting as a representation of that underlying type. Generic code typically uses Borrow<T>
        when it relies on the identical behavior of these additional trait implementations. These
        traits will likely appear as additional trait bounds.

        In particular Eq, Ord and Hash must be equivalent for borrowed and owned values:
        x.borrow() == y.borrow() should give the same result as x == y.

        If generic code merely needs to work for all types that can provide a reference to related
        type T, it is often better to use AsRef<T> as more types can safely implement it.

 */

fn main_0() {
    // 声明
    let arr: [i32; 3] = [1, 2, 3];
    /* 编译错误：
        error[E0283]: type annotations needed
             let a = arr.borrow();
                     ----^^^^^^--
                     |   |
                     |   cannot infer type for array `[i32; 3]`
                     this method call resolves to `&Borrowed`
           = note: cannot satisfy `[i32; 3]: std::borrow::Borrow<_>`
     */
    // let a = arr.borrow();

    // 类型标注方式 1 ：值表达式
    use std::borrow::Borrow;
    /* [自]不能为自身类型指定类型[???]
       error[E0109]: type arguments are not allowed for this type
           let a = arr::<[i32;3]>.borrow();
                        ^^^^^^^ type argument not allowed
     */
    // let a = arr::<[i32;3]>.borrow();
    let a: &[i32] = arr.borrow();

    /* 类型标注方式 2 ：位置表达式
          " ::<T> "形式的标注类型即 turbofish 操作符
       例如：
            let x = "1";
            let a = x.parse::<i32>();
            println!("{:?}",a);     // Console:" Ok(1) "
       问题: 为何此种方式不可用于 borrow [因其为非泛型参数]
             源于" turbofish 操作符专用于泛型函数 "
             (参阅图"0_turbofish 操作符专用于泛型函数.png")
     */
    //let a = arr.borrow::<&[i32]>();

    for i in a {
        println!("{}", i);
    }
    println!("{:?}", a);    // 移动后不可再用

}

/*
4. 实现 Trait std::borrow::BorrowMut
    a. 实现 BorrowMut 源码
        #[stable(feature = "array_borrow", since = "1.4.0")]
        impl<T, const N: usize> BorrowMut<[T]> for [T; N]
        where
            [T; N]: LengthAtMost32,
        {
            fn borrow_mut(&mut self) -> &mut [T] {
                self
            }
        }

    b. 相关 BorrowMut Trait 源码(可参阅"4_trait\4_BorrowMut.rs")：
        // 文档:" https://doc.rust-lang.org/std/borrow/trait.BorrowMut.html "
        Trait std::borrow::BorrowMut
        [−]
        pub trait BorrowMut<Borrowed>: Borrow<Borrowed>
        where
            Borrowed: ?Sized,
        {
            fn borrow_mut(&mut self) -> &mut Borrowed;
        }
        [−]
        A trait for mutably borrowing data.

        As a companion to Borrow<T> this trait allows a type to borrow as an underlying type by
        providing a mutable reference. See Borrow<T> for more information on borrowing as another
        type.

 */

fn main() {
    // 声明
    let mut arr: [i32; 3] = [1, 2, 3];
    // 类型标注方式 1 ：值表达式
    use std::borrow::BorrowMut;
    let a: &mut [i32] = arr.borrow_mut();
    // 类型标注方式 2 ：位置表达式(同理：不可使用 furbofish 操作符)
    //let a = arr.borrow_mut::<&[i32]>();   // 飘红报错(因 borrow_mut 非泛型函数)

    for i in a {
        *i += 2
    }
    println!("{:?}", arr);  // Console:" [3, 4, 5] "

}