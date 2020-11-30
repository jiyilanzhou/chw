
/*
Slice 声明
    a. Primitive Type slice 动态大小的连续序列
    b. 切片是一个内存块的视图，表示为一个指针和一个长度
    c. 切片亦分为可变或共享：共享切片类型为 &[T] 而可变切片类型为 &mut [T]
       (可更改可变切片指向的内存块[其中 T 表示元素类型])
    文档:" https://doc.rust-lang.org/std/primitive.slice.html "

Primitive Type slice
    Primitive Type slice
    [−]
    A dynamically-sized view into a contiguous sequence, [T]. Contiguous here means that elements
    are laid out so that every element is the same distance from its neighbors.

    See also the std::slice module.

    Slices are a view into a block of memory represented as a pointer and a length.

    // slicing a Vec
    let vec = vec![1, 2, 3];
    let int_slice = &vec[..];
    // coercing an array to a slice
    let str_slice: &[&str] = &["one", "two", "three"];
    Run

    Slices are either mutable or shared. The shared slice type is &[T], while the mutable slice
    type is &mut [T], where T represents the element type. For example, you can mutate the block of
    memory that a mutable slice points to:

    let mut x = [1, 2, 3];
    let x = &mut x[..]; // Take a full slice of `x`.
    x[1] = 7;
    assert_eq!(x, &[1, 7, 3]);

*/
fn main_0() {
    // 切片 Vec (集合或动态数组)
    let vec = vec![1, 2, 3];
    let int_slice = &vec[..];
    // 将数组强制为切片
    let str_slice: &[&str] = &["one", "two", "three"];
    // 更改可变切片指向的内存块
    let mut x = [1, 2, 3];
    let x = &mut x[..];
    x.len();
    x[1] = 7;
    assert_eq!(x, &[1, 7, 3]);
}

/*
Implementations :
    #[lang = "slice"]
    #[cfg(not(test))]
    impl<T> for &[T] {
          pub const fn len(&self) -> usize
          // ...
          pub fn partition_point<P>(&self, mut pred: P) -> usize
    }

0. pub const fn len(&self) -> usize
    [src][−]
    Returns the number of elements in the slice.

    /// Returns the number of elements in the slice.
    ///
    /// # Examples
    ///
    /// ```
    /// let a = [1, 2, 3];
    /// assert_eq!(a.len(), 3);
    /// ```
    #[stable(feature = "rust1", since = "1.0.0")]
    #[rustc_const_stable(feature = "const_slice_len", since = "1.32.0")]
    #[inline]
    // SAFETY: const sound because we transmute out the length field as a usize (which it must be)
    #[allow(unused_attributes)]
    #[allow_internal_unstable(const_fn_union)]
    pub const fn len(&self) -> usize {
        unsafe { crate::ptr::Repr { rust: self }.raw.len }
    }

    Examples
    let a = [1, 2, 3];
    assert_eq!(a.len(), 3);

1. pub const fn is_empty(&self) -> bool
    [src][−]
    Returns true if the slice has a length of 0.

    /// Returns `true` if the slice has a length of 0.
    ///
    /// # Examples
    ///
    /// ```
    /// let a = [1, 2, 3];
    /// assert!(!a.is_empty());
    /// ```
    #[stable(feature = "rust1", since = "1.0.0")]
    #[rustc_const_stable(feature = "const_slice_is_empty", since = "1.32.0")]
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }

    Examples
    let a = [1, 2, 3];
    assert!(!a.is_empty());

2. pub fn first(&self) -> Option<&T>
    [src][−]
    Returns the first element of the slice, or None if it is empty.

    /// Returns the first element of the slice, or `None` if it is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// let v = [10, 40, 30];
    /// assert_eq!(Some(&10), v.first());
    ///
    /// let w: &[i32] = &[];
    /// assert_eq!(None, w.first());
    /// ```
    #[stable(feature = "rust1", since = "1.0.0")]
    #[inline]
    pub fn first(&self) -> Option<&T> {
        if let [first, ..] = self { Some(first) } else { None }
    }

    Examples
    let v = [10, 40, 30];
    assert_eq!(Some(&10), v.first());
    let w: &[i32] = &[];
    assert_eq!(None, w.first());

3. pub fn first_mut(&mut self) -> Option<&mut T>
    [src][−]
    Returns a mutable pointer to the first element of the slice, or None if it is empty.

    /// Returns a mutable pointer to the first element of the slice, or `None` if it is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// let x = &mut [0, 1, 2];
    ///
    /// if let Some(first) = x.first_mut() {
    ///     *first = 5;
    /// }
    /// assert_eq!(x, &[5, 1, 2]);
    /// ```
    #[stable(feature = "rust1", since = "1.0.0")]
    #[inline]
    pub fn first_mut(&mut self) -> Option<&mut T> {
        if let [first, ..] = self { Some(first) } else { None }
    }

    Examples
    let x = &mut [0, 1, 2];
    if let Some(first) = x.first_mut() {
        *first = 5;
    }
    assert_eq!(x, &[5, 1, 2]);

4. pub fn split_first(&self) -> Option<(&T, &[T])>
    1.5.0
    [src]
    [−]
    Returns the first and all the rest of the elements of the slice, or None if it is empty.

    /// Returns the first and all the rest of the elements of the slice, or `None` if it is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// let x = &[0, 1, 2];
    ///
    /// if let Some((first, elements)) = x.split_first() {
    ///     assert_eq!(first, &0);
    ///     assert_eq!(elements, &[1, 2]);
    /// }
    /// ```
    #[stable(feature = "slice_splits", since = "1.5.0")]
    #[inline]
    pub fn split_first(&self) -> Option<(&T, &[T])> {
        // tail 使用 @ 绑定剩余元素(即切片)
        if let [first, tail @ ..] = self { Some((first, tail)) } else { None }
    }

    Examples
    let x = &[0, 1, 2];
    if let Some((first, elements)) = x.split_first() {
        assert_eq!(first, &0);
        assert_eq!(elements, &[1, 2]);
    }

*/
fn main_1() {
    let x = &[0, 1, 2];
    if let Some((first, elements)) = split_first(x) {
        assert_eq!(first, &0);
        assert_eq!(elements, &[1, 2]);
    }
}
pub fn split_first<T>(sli: &[T]) -> Option<(&T, &[T])> {
    // 模式解构典型应用("@"用于绑定)
    if let [first, tail @ ..] = sli {
        Some((first, tail))
    } else {
        None
    }
}

/*
5. pub fn split_first_mut(&mut self) -> Option<(&mut T, &mut [T])>
    1.5.0
    [src]
    [−]
    Returns the first and all the rest of the elements of the slice, or None if it is empty.

    /// Returns the first and all the rest of the elements of the slice, or `None` if it is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// let x = &mut [0, 1, 2];
    ///
    /// if let Some((first, elements)) = x.split_first_mut() {
    ///     *first = 3;
    ///     elements[0] = 4;
    ///     elements[1] = 5;
    /// }
    /// assert_eq!(x, &[3, 4, 5]);
    /// ```
    #[stable(feature = "slice_splits", since = "1.5.0")]
    #[inline]
    pub fn split_first_mut(&mut self) -> Option<(&mut T, &mut [T])> {
        if let [first, tail @ ..] = self { Some((first, tail)) } else { None }
    }

    Examples
    let x = &mut [0, 1, 2];
    // 返回可变主用于操作原切片
    if let Some((first, elements)) = x.split_first_mut() {
        *first = 3;
        elements[0] = 4;
        elements[1] = 5;
    }
    assert_eq!(x, &[3, 4, 5]);

6. pub fn split_last(&self) -> Option<(&T, &[T])>
    1.5.0
    [src]
    [−]
    Returns the last and all the rest of the elements of the slice, or None if it is empty.

    /// Returns the last and all the rest of the elements of the slice, or `None` if it is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// let x = &[0, 1, 2];
    ///
    /// if let Some((last, elements)) = x.split_last() {
    ///     assert_eq!(last, &2);
    ///     assert_eq!(elements, &[0, 1]);
    /// }
    /// ```
    #[stable(feature = "slice_splits", since = "1.5.0")]
    #[inline]
    pub fn split_last(&self) -> Option<(&T, &[T])> {
        if let [init @ .., last] = self { Some((last, init)) } else { None }
    }

    Examples
    let x = &[0, 1, 2];
    if let Some((last, elements)) = x.split_last() {
        assert_eq!(last, &2);
        assert_eq!(elements, &[0, 1]);
    }

7. pub fn split_last_mut(&mut self) -> Option<(&mut T, &mut [T])>
    1.5.0
    [src]
    [−]
    Returns the last and all the rest of the elements of the slice, or None if it is empty.

    /// Returns the last and all the rest of the elements of the slice, or `None` if it is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// let x = &mut [0, 1, 2];
    ///
    /// if let Some((last, elements)) = x.split_last_mut() {
    ///     *last = 3;
    ///     elements[0] = 4;
    ///     elements[1] = 5;
    /// }
    /// assert_eq!(x, &[4, 5, 3]);
    /// ```
    #[stable(feature = "slice_splits", since = "1.5.0")]
    #[inline]
    pub fn split_last_mut(&mut self) -> Option<(&mut T, &mut [T])> {
        if let [init @ .., last] = self { Some((last, init)) } else { None }
    }

    Examples
    let x = &mut [0, 1, 2];
    if let Some((last, elements)) = x.split_last_mut() {
        *last = 3;
        elements[0] = 4;
        elements[1] = 5;
    }
    assert_eq!(x, &[4, 5, 3]);

8. pub fn last(&self) -> Option<&T>
    [src][−]
    Returns the last element of the slice, or None if it is empty.

    /// Returns the last element of the slice, or `None` if it is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// let v = [10, 40, 30];
    /// assert_eq!(Some(&30), v.last());
    ///
    /// let w: &[i32] = &[];
    /// assert_eq!(None, w.last());
    /// ```
    #[stable(feature = "rust1", since = "1.0.0")]
    #[inline]
    pub fn last(&self) -> Option<&T> {
        if let [.., last] = self { Some(last) } else { None }
    }

    Examples
    let v = [10, 40, 30];
    assert_eq!(Some(&30), v.last());
    let w: &[i32] = &[];
    assert_eq!(None, w.last());

9. pub fn last_mut(&mut self) -> Option<&mut T>
    [src][−]
    Returns a mutable pointer to the last item in the slice.

    /// Returns a mutable pointer to the last item in the slice.
    ///
    /// # Examples
    ///
    /// ```
    /// let x = &mut [0, 1, 2];
    ///
    /// if let Some(last) = x.last_mut() {
    ///     *last = 10;
    /// }
    /// assert_eq!(x, &[0, 1, 10]);
    /// ```
    #[stable(feature = "rust1", since = "1.0.0")]
    #[inline]
    pub fn last_mut(&mut self) -> Option<&mut T> {
        if let [.., last] = self { Some(last) } else { None }
    }

    Examples
    let x = &mut [0, 1, 2];
    if let Some(last) = x.last_mut() {
        *last = 10;
    }
    assert_eq!(x, &[0, 1, 10]);

10. pub fn get<I>(&self, index: I) -> Option<&<I as SliceIndex<[T]>>::Output>
    where
        I: SliceIndex<[T]>,
    [src][−]
    Returns a reference to an element or subslice depending on the type of index.

        If given a position, returns a reference to the element at that position or None if out of
        bounds.
        If given a range, returns the subslice corresponding to that range, or None if out of bounds.

    /// Returns a reference to an element or subslice depending on the type of
    /// index.
    ///
    /// - If given a position, returns a reference to the element at that
    ///   position or `None` if out of bounds.
    /// - If given a range, returns the subslice corresponding to that range,
    ///   or `None` if out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// let v = [10, 40, 30];
    /// assert_eq!(Some(&40), v.get(1));
    /// assert_eq!(Some(&[10, 40][..]), v.get(0..2));
    /// assert_eq!(None, v.get(3));
    /// assert_eq!(None, v.get(0..4));
    /// ```
    #[stable(feature = "rust1", since = "1.0.0")]
    #[inline]
    pub fn get<I>(&self, index: I) -> Option<&I::Output>
    where
        I: SliceIndex<Self>,
    {
        index.get(self)
    }

    Examples
    let v = [10, 40, 30];
    assert_eq!(Some(&40), v.get(1));
    // " 0..2 "即为" 0..=1 ": get 参数索引亦可使用"范围表达式"匹配
    assert_eq!(Some(&[10, 40][..]), v.get(0..2));
    assert_eq!(None, v.get(3));
    assert_eq!(None, v.get(0..4));

11. pub fn get_mut<I>(
        &mut self,
        index: I
    ) -> Option<&mut <I as SliceIndex<[T]>>::Output>
    where
        I: SliceIndex<[T]>,
    [src][−]
    Returns a mutable reference to an element or subslice depending on the type of index (see get)
    or None if the index is out of bounds.

    /// Returns a mutable reference to an element or subslice depending on the
    /// type of index (see [`get`]) or `None` if the index is out of bounds.
    ///
    /// [`get`]: #method.get
    ///
    /// # Examples
    ///
    /// ```
    /// let x = &mut [0, 1, 2];
    ///
    /// if let Some(elem) = x.get_mut(1) {
    ///     *elem = 42;
    /// }
    /// assert_eq!(x, &[0, 42, 2]);
    /// ```
    #[stable(feature = "rust1", since = "1.0.0")]
    #[inline]
    pub fn get_mut<I>(&mut self, index: I) -> Option<&mut I::Output>
    where
        I: SliceIndex<Self>,
    {
        index.get_mut(self)
    }

    Examples
    let x = &mut [0, 1, 2];
    if let Some(elem) = x.get_mut(1) {
        *elem = 42;
    }
    assert_eq!(x, &[0, 42, 2]);

 */
fn main() {
    let v = [10, 40, 30];
    assert_eq!(Some(&40), v.get(1));
    // " 0..2 "即为" 0..=1 "
    assert_eq!(Some(&[10, 40][..]), v.get(0..=1));
    assert_eq!(None, v.get(3));
    assert_eq!(None, v.get(0..4));
}
