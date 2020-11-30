
#![feature(slice_ptr_range)]
/*
Primitive Type slice ：
    文档:" https://doc.rust-lang.org/std/primitive.slice.html "

12. pub unsafe fn get_unchecked<I>(
        &self,
        index: I
    ) -> &<I as SliceIndex<[T]>>::Output
    where
        I: SliceIndex<[T]>,
    [src][−]
    Returns a reference to an element or subslice, without doing bounds checking.

    This is generally not recommended, use with caution! Calling this method with an out-of-bounds
    index is undefined behavior even if the resulting reference is not used. For a safe alternative
    see get.

    /// Returns a reference to an element or subslice, without doing bounds
    /// checking.
    ///
    /// This is generally not recommended, use with caution!
    /// Calling this method with an out-of-bounds index is *[undefined behavior]*
    /// even if the resulting reference is not used.
    /// For a safe alternative see [`get`].
    ///
    /// [`get`]: #method.get
    /// [undefined behavior]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
    ///
    /// # Examples
    ///
    /// ```
    /// let x = &[1, 2, 4];
    ///
    /// unsafe {
    ///     assert_eq!(x.get_unchecked(1), &2);
    /// }
    /// ```
    #[stable(feature = "rust1", since = "1.0.0")]
    #[inline]
    pub unsafe fn get_unchecked<I>(&self, index: I) -> &I::Output
    where
        I: SliceIndex<Self>,
    {
        // " uncheck "主用于快速访问
        // SAFETY: the caller must uphold the safety requirements for `get_unchecked`.
        unsafe { index.get_unchecked(self) }
    }

    Examples
    let x = &[1, 2, 4];
    unsafe {
        assert_eq!(x.get_unchecked(1), &2);
    }

13. pub unsafe fn get_unchecked_mut<I>(
        &mut self,
        index: I
    ) -> &mut <I as SliceIndex<[T]>>::Output
    where
        I: SliceIndex<[T]>,
    [src][−]
    Returns a mutable reference to an element or subslice, without doing bounds checking.

    This is generally not recommended, use with caution! Calling this method with an out-of-bounds
    index is undefined behavior even if the resulting reference is not used. For a safe alternative
    see get_mut.

    /// Returns a mutable reference to an element or subslice, without doing
    /// bounds checking.
    ///
    /// This is generally not recommended, use with caution!
    /// Calling this method with an out-of-bounds index is *[undefined behavior]*
    /// even if the resulting reference is not used.
    /// For a safe alternative see [`get_mut`].
    ///
    /// [`get_mut`]: #method.get_mut
    /// [undefined behavior]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
    ///
    /// # Examples
    ///
    /// ```
    /// let x = &mut [1, 2, 4];
    ///
    /// unsafe {
    ///     let elem = x.get_unchecked_mut(1);
    ///     *elem = 13;
    /// }
    /// assert_eq!(x, &[1, 13, 4]);
    /// ```
    #[stable(feature = "rust1", since = "1.0.0")]
    #[inline]
    pub unsafe fn get_unchecked_mut<I>(&mut self, index: I) -> &mut I::Output
    where
        I: SliceIndex<Self>,
    {
        // SAFETY: the caller must uphold the safety requirements for `get_unchecked_mut`.
        unsafe { index.get_unchecked_mut(self) }
    }

    Examples
    let x = &mut [1, 2, 4];
    unsafe {
        let elem = x.get_unchecked_mut(1);
        *elem = 13;
    }
    assert_eq!(x, &[1, 13, 4]);

 */

fn main_0() {
    let x = &mut [1, 2, 4];
    unsafe {
        assert_eq!(x.get_unchecked(1), &2);
    }
    unsafe {
        let elem = x.get_unchecked_mut(1);
        *elem = 13;
    }
    assert_eq!(x, &[1, 13, 4]);
}

/*
14. pub const fn as_ptr(&self) -> *const T
    [src][−]
     // 返回指向切片缓冲区的原始指针
    Returns a raw pointer to the slice's buffer.

     // 调用方必须确保切片的生命周期比"as_ptr"返回的指针的生命周期长，否则它将指向垃圾
    The caller must ensure that the slice outlives the pointer this function returns, or else it
    will end up pointing to garbage.

     // 调用方还必须确保指针(不可传递)指向的内存从未使用此指针或从其派生的任何指针写入
    The caller must also ensure that the memory the pointer (non-transitively) points to is never
    written to (except inside an UnsafeCell) using this pointer or any pointer derived from it. If
    you need to mutate the contents of the slice, use as_mut_ptr.

     // 修改此切片引用的容器可能会导致其缓冲区重新分配，这也会使指向它的任何指针无效
    Modifying the container referenced by this slice may cause its buffer to be reallocated, which
    would also make any pointers to it invalid.

    /// Returns a raw pointer to the slice's buffer.
    ///
    /// The caller must ensure that the slice outlives the pointer this
    /// function returns, or else it will end up pointing to garbage.
    ///
    /// The caller must also ensure that the memory the pointer (non-transitively) points to
    /// is never written to (except inside an `UnsafeCell`) using this pointer or any pointer
    /// derived from it. If you need to mutate the contents of the slice, use [`as_mut_ptr`].
    ///
    /// Modifying the container referenced by this slice may cause its buffer
    /// to be reallocated, which would also make any pointers to it invalid.
    ///
    /// # Examples
    ///
    /// ```
    /// let x = &[1, 2, 4];
    /// let x_ptr = x.as_ptr();
    ///
    /// unsafe {
    ///     for i in 0..x.len() {
    ///         assert_eq!(x.get_unchecked(i), &*x_ptr.add(i));
    ///     }
    /// }
    /// ```
    ///
    /// [`as_mut_ptr`]: #method.as_mut_ptr
    #[stable(feature = "rust1", since = "1.0.0")]
    #[rustc_const_stable(feature = "const_slice_as_ptr", since = "1.32.0")]
    #[inline]
    pub const fn as_ptr(&self) -> *const T {
        self as *const [T] as *const T
    }

    Examples
    let x = &[1, 2, 4];
    let x_ptr = x.as_ptr();
    unsafe {
        for i in 0..x.len() {
            assert_eq!(x.get_unchecked(i), &*x_ptr.add(i));
        }
    }

15. pub fn as_mut_ptr(&mut self) -> *mut T
    [src][−]
    Returns an unsafe mutable pointer to the slice's buffer.

    The caller must ensure that the slice outlives the pointer this function returns, or else it
    will end up pointing to garbage.

    Modifying the container referenced by this slice may cause its buffer to be reallocated, which
    would also make any pointers to it invalid.

    /// Returns an unsafe mutable pointer to the slice's buffer.
    ///
    /// The caller must ensure that the slice outlives the pointer this
    /// function returns, or else it will end up pointing to garbage.
    ///
    /// Modifying the container referenced by this slice may cause its buffer
    /// to be reallocated, which would also make any pointers to it invalid.
    ///
    /// # Examples
    ///
    /// ```
    /// let x = &mut [1, 2, 4];
    /// let x_ptr = x.as_mut_ptr();
    ///
    /// unsafe {
    ///     for i in 0..x.len() {
    ///         *x_ptr.add(i) += 2;
    ///     }
    /// }
    /// assert_eq!(x, &[3, 4, 6]);
    /// ```
    #[stable(feature = "rust1", since = "1.0.0")]
    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut T {
        self as *mut [T] as *mut T
    }

    Examples
    let x = &mut [1, 2, 4];
    let x_ptr = x.as_mut_ptr();
    unsafe {
        for i in 0..x.len() {
            *x_ptr.add(i) += 2;
        }
    }
    assert_eq!(x, &[3, 4, 6]);

 */
fn main_1() {
    // as_ptr
    let x = &[1, 2, 4];
    let x_ptr = x.as_ptr();
    unsafe {
        for i in 0..x.len() {
            // " x_ptr.add(i) " ： 指针偏移(返回原生指针指向的地址)
            assert_eq!(x.get_unchecked(i), &(*(x_ptr.add(i))));
            assert_eq!(x.get_unchecked(i), &(*x_ptr.add(i)));
            assert_eq!(x.get_unchecked(i), &*x_ptr.add(i));
            // 转换为原生指针再比较
            assert_eq!(x.get_unchecked(i) as *const i32, x_ptr.add(i));
        }
    }
    // mut
    let x = &mut [1, 2, 4];
    let x_ptr = x.as_mut_ptr();
    unsafe {
        for i in 0..x.len() {
            *x_ptr.add(i) += 2;
        }
    }
    assert_eq!(x, &[3, 4, 6]);
}

/*
16. pub fn as_ptr_range(&self) -> Range<*const T>
    [src][−]
    🔬 This is a nightly-only experimental API. (slice_ptr_range #65807)
    // 返回跨越切片的两个原始指针
    Returns the two raw pointers spanning the slice.

    // 返回的范围是半开的(如" 0x9afb9c..0x9afba8 ")，意味着结束指针指向切片的最后一个元素
    The returned range is half-open, which means that the end pointer points one past the last
    // 如此，一个空切片由两个相等的指针表示，并且两个指针之间的差表示切片的大小
    element of the slice. This way, an empty slice is represented by two equal pointers, and the
    difference between the two pointers represents the size of the slice.

    // 请参阅 as_ptr 以获取有关使用这些指针的警告。结束指针需要格外小心，因为它没有指向切片中的有效元素。
    See as_ptr for warnings on using these pointers. The end pointer requires extra caution, as it
    does not point to a valid element in the slice.

    // 这个函数对于与使用两个指针来引用内存中的元素的外部接口进行交互很有用
    This function is useful for interacting with foreign interfaces which use two pointers to refer
    to a range of elements in memory, as is common in C++.

    // 检查指向某个元素的指针是否引用了此 slice 的元素也可能很有用
    It can also be useful to check if a pointer to an element refers to an element of this slice:

        #![feature(slice_ptr_range)]

        let a = [1, 2, 3];
        let x = &a[1] as *const _;
        let y = &5 as *const _;

        assert!(a.as_ptr_range().contains(&x));
        assert!(!a.as_ptr_range().contains(&y));

    /// Returns the two raw pointers spanning the slice.
    ///
    /// The returned range is half-open, which means that the end pointer
    /// points *one past* the last element of the slice. This way, an empty
    /// slice is represented by two equal pointers, and the difference between
    /// the two pointers represents the size of the slice.
    ///
    /// See [`as_ptr`] for warnings on using these pointers. The end pointer
    /// requires extra caution, as it does not point to a valid element in the
    /// slice.
    ///
    /// This function is useful for interacting with foreign interfaces which
    /// use two pointers to refer to a range of elements in memory, as is
    /// common in C++.
    ///
    /// It can also be useful to check if a pointer to an element refers to an
    /// element of this slice:
    ///
    /// ```
    /// #![feature(slice_ptr_range)]
    ///
    /// let a = [1, 2, 3];
    /// let x = &a[1] as *const _;
    /// let y = &5 as *const _;
    ///
    /// assert!(a.as_ptr_range().contains(&x));
    /// assert!(!a.as_ptr_range().contains(&y));
    /// ```
    ///
    /// [`as_ptr`]: #method.as_ptr
    #[unstable(feature = "slice_ptr_range", issue = "65807")]
    #[inline]
    pub fn as_ptr_range(&self) -> Range<*const T> {
        // The `add` here is safe, because:
        //
        //   - Both pointers are part of the same object, as pointing directly
        //     past the object also counts.
        //
        //   - The size of the slice is never larger than isize::MAX bytes, as
        //     noted here:
        //       - https://github.com/rust-lang/unsafe-code-guidelines/issues/102#issuecomment-473340447
        //       - https://doc.rust-lang.org/reference/behavior-considered-undefined.html
        //       - https://doc.rust-lang.org/core/slice/fn.from_raw_parts.html#safety
        //     (This doesn't seem normative yet, but the very same assumption is
        //     made in many places, including the Index implementation of slices.)
        //
        //   - There is no wrapping around involved, as slices do not wrap past
        //     the end of the address space.
        //
        // See the documentation of pointer::add.
        let start = self.as_ptr();
        let end = unsafe { start.add(self.len()) };
        start..end
    }

17. pub fn as_mut_ptr_range(&mut self) -> Range<*mut T>
    [src][−]
    🔬 This is a nightly-only experimental API. (slice_ptr_range #65807)
    // 返回跨越切片的两个不安全的可变指针
    Returns the two unsafe mutable pointers spanning the slice.

    The returned range is half-open, which means that the end pointer points one past the last
    element of the slice. This way, an empty slice is represented by two equal pointers, and the
    difference between the two pointers represents the size of the slice.

    See as_mut_ptr for warnings on using these pointers. The end pointer requires extra caution, as
    it does not point to a valid element in the slice.

    This function is useful for interacting with foreign interfaces which use two pointers to refer
    to a range of elements in memory, as is common in C++.

    /// Returns the two unsafe mutable pointers spanning the slice.
    ///
    /// The returned range is half-open, which means that the end pointer
    /// points *one past* the last element of the slice. This way, an empty
    /// slice is represented by two equal pointers, and the difference between
    /// the two pointers represents the size of the slice.
    ///
    /// See [`as_mut_ptr`] for warnings on using these pointers. The end
    /// pointer requires extra caution, as it does not point to a valid element
    /// in the slice.
    ///
    /// This function is useful for interacting with foreign interfaces which
    /// use two pointers to refer to a range of elements in memory, as is
    /// common in C++.
    ///
    /// [`as_mut_ptr`]: #method.as_mut_ptr
    #[unstable(feature = "slice_ptr_range", issue = "65807")]
    #[inline]
    pub fn as_mut_ptr_range(&mut self) -> Range<*mut T> {
        // See as_ptr_range() above for why `add` here is safe.
        let start = self.as_mut_ptr();
        let end = unsafe { start.add(self.len()) };
        start..end
    }

*/
/* 使用" as_ptr_range() "：
       置顶标注" #![feature(slice_ptr_range)] "
 */
fn main() {
    let a = [1, 2, 3];
    let x = &a[1] as *const _;
    let y = &5 as *const _;
    assert!(a.as_ptr_range().contains(&x));
    // Console:" 0x9afb9c..0x9afba8 "
    println!("{:?}",a.as_ptr_range());
    assert!(!a.as_ptr_range().contains(&y));
}