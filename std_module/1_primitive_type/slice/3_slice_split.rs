
// split[splɪt]v. 分割
/*
Primitive Type slice ：
    文档:" https://doc.rust-lang.org/std/primitive.slice.html "

31. pub fn split_at(&self, mid: usize) -> (&[T], &[T])
    [src][−]
    // 按索引将切片一分为二
    Divides one slice into two at an index.

    The first will contain all indices from [0, mid) (excluding the index mid itself) and the second
    will contain all indices from [mid, len) (excluding the index len itself).

    Panics
    Panics if mid > len.

    /// Divides one slice into two at an index.
    ///
    /// The first will contain all indices from `[0, mid)` (excluding
    /// the index `mid` itself) and the second will contain all
    /// indices from `[mid, len)` (excluding the index `len` itself).
    ///
    /// # Panics
    ///
    /// Panics if `mid > len`.
    ///
    /// # Examples
    ///
    /// ```
    /// let v = [1, 2, 3, 4, 5, 6];
    ///
    /// {
    ///    let (left, right) = v.split_at(0);
    ///    assert!(left == []);
    ///    assert!(right == [1, 2, 3, 4, 5, 6]);
    /// }
    ///
    /// {
    ///     let (left, right) = v.split_at(2);
    ///     assert!(left == [1, 2]);
    ///     assert!(right == [3, 4, 5, 6]);
    /// }
    ///
    /// {
    ///     let (left, right) = v.split_at(6);
    ///     assert!(left == [1, 2, 3, 4, 5, 6]);
    ///     assert!(right == []);
    /// }
    /// ```
    #[stable(feature = "rust1", since = "1.0.0")]
    #[inline]
    pub fn split_at(&self, mid: usize) -> (&[T], &[T]) {
        (&self[..mid], &self[mid..])
    }

    Examples
    let v = [1, 2, 3, 4, 5, 6];
    {
       let (left, right) = v.split_at(0);
       assert!(left == []);
       assert!(right == [1, 2, 3, 4, 5, 6]);
    }
    {
        let (left, right) = v.split_at(2);
        assert!(left == [1, 2]);
        assert!(right == [3, 4, 5, 6]);
    }
    {
        let (left, right) = v.split_at(6);
        assert!(left == [1, 2, 3, 4, 5, 6]);
        assert!(right == []);
    }

32. pub fn split_at_mut(&mut self, mid: usize) -> (&mut [T], &mut [T])
    [src][−]
    Divides one mutable slice into two at an index.

    The first will contain all indices from [0, mid) (excluding the index mid itself) and the second
    will contain all indices from [mid, len) (excluding the index len itself).

    Panics
    Panics if mid > len.

    /// Divides one mutable slice into two at an index.
    ///
    /// The first will contain all indices from `[0, mid)` (excluding
    /// the index `mid` itself) and the second will contain all
    /// indices from `[mid, len)` (excluding the index `len` itself).
    ///
    /// # Panics
    ///
    /// Panics if `mid > len`.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut v = [1, 0, 3, 0, 5, 6];
    /// // scoped to restrict the lifetime of the borrows
    /// {
    ///     let (left, right) = v.split_at_mut(2);
    ///     assert!(left == [1, 0]);
    ///     assert!(right == [3, 0, 5, 6]);
    ///     left[1] = 2;
    ///     right[1] = 4;
    /// }
    /// assert!(v == [1, 2, 3, 4, 5, 6]);
    /// ```
    #[stable(feature = "rust1", since = "1.0.0")]
    #[inline]
    pub fn split_at_mut(&mut self, mid: usize) -> (&mut [T], &mut [T]) {
        let len = self.len();
        let ptr = self.as_mut_ptr();

        unsafe {
            assert!(mid <= len);

            (from_raw_parts_mut(ptr, mid), from_raw_parts_mut(ptr.add(mid), len - mid))
        }
    }

    Examples
    let mut v = [1, 0, 3, 0, 5, 6];
    // scoped to restrict the lifetime of the borrows
    {
        let (left, right) = v.split_at_mut(2);
        assert!(left == [1, 0]);
        assert!(right == [3, 0, 5, 6]);
        left[1] = 2;
        right[1] = 4;
    }
    assert!(v == [1, 2, 3, 4, 5, 6]);

*/
#![feature(split_inclusive)]
fn main_0() {
    // split_at
    let v = [1, 2, 3, 4, 5, 6];
    {
        let (left, right) = v.split_at(0);
        assert!(left == []);
        assert!(right == [1, 2, 3, 4, 5, 6]);
    }
    {
        let (left, right) = v.split_at(2);
        assert!(left == [1, 2]);
        assert!(right == [3, 4, 5, 6]);
    }
    {
        let (left, right) = v.split_at(6);
        assert!(left == [1, 2, 3, 4, 5, 6]);
        assert!(right == []);
    }
    // split_at_mut
    let mut v = [1, 0, 3, 0, 5, 6];
    {
        let (left, right) = v.split_at_mut(2);
        assert!(left == [1, 0]);
        assert!(right == [3, 0, 5, 6]);
        left[1] = 2;
        right[1] = 4;
    }
    assert!(v == [1, 2, 3, 4, 5, 6]);
}

// predicate[ˈprɛdɪˌkeɪt]n.述语,断言
/*
33. pub fn split<F>(&self, pred: F) -> Split<T, F>
    where
        F: FnMut(&T) -> bool,
    [src][−]
    // 返回由匹配元素分割的子切片迭代器(子切片不包括匹配元素)
    Returns an iterator over subslices separated by elements that match pred. The matched element is
    // 匹配的元素不包含在子切片中
    not contained in the subslices.

    /// Returns an iterator over subslices separated by elements that match
    /// `pred`. The matched element is not contained in the subslices.
    ///
    /// # Examples
    ///
    /// ```
    /// let slice = [10, 40, 33, 20];
    /// let mut iter = slice.split(|num| num % 3 == 0);
    ///
    /// assert_eq!(iter.next().unwrap(), &[10, 40]);
    /// assert_eq!(iter.next().unwrap(), &[20]);
    /// assert!(iter.next().is_none());
    /// ```
    ///
    /// If the first element is matched, an empty slice will be the first item
    /// returned by the iterator. Similarly, if the last element in the slice
    /// is matched, an empty slice will be the last item returned by the
    /// iterator:
    ///
    /// ```
    /// let slice = [10, 40, 33];
    /// let mut iter = slice.split(|num| num % 3 == 0);
    ///
    /// assert_eq!(iter.next().unwrap(), &[10, 40]);
    /// assert_eq!(iter.next().unwrap(), &[]);
    /// assert!(iter.next().is_none());
    /// ```
    ///
    /// If two matched elements are directly adjacent, an empty slice will be
    /// present between them:
    ///
    /// ```
    /// let slice = [10, 6, 33, 20];
    /// let mut iter = slice.split(|num| num % 3 == 0);
    ///
    /// assert_eq!(iter.next().unwrap(), &[10]);
    /// assert_eq!(iter.next().unwrap(), &[]);
    /// assert_eq!(iter.next().unwrap(), &[20]);
    /// assert!(iter.next().is_none());
    /// ```
    #[stable(feature = "rust1", since = "1.0.0")]
    #[inline]
    pub fn split<F>(&self, pred: F) -> Split<'_, T, F>
    where
        F: FnMut(&T) -> bool,
    {
        Split { v: self, pred, finished: false }
    }

    Examples
    let slice = [10, 40, 33, 20];
    let mut iter = slice.split(|num| num % 3 == 0);
    assert_eq!(iter.next().unwrap(), &[10, 40]);
    assert_eq!(iter.next().unwrap(), &[20]);
    assert!(iter.next().is_none());

    // 首尾元素匹配则其位置返回一个空切片
    If the first element is matched, an empty slice will be the first item returned
    by the iterator. Similarly, if the last element in the slice is matched, an empty
    slice will be the last item returned by the iterator:
    Examples
    let slice = [6, 10, 40, 33];
    let mut iter = slice.split(|num| num % 3 == 0);
    assert_eq!(iter.next().unwrap(), &[]);
    assert_eq!(iter.next().unwrap(), &[10, 40]);
    assert_eq!(iter.next().unwrap(), &[]);
    assert!(iter.next().is_none());

    // 相邻两个元素匹配亦在其间返回一个空切片
    If two matched elements are directly adjacent, an empty slice will be present between them:
    Examples
    let slice = [10, 6, 33, 20];
    let mut iter = slice.split(|num| num % 3 == 0);
    assert_eq!(iter.next().unwrap(), &[10]);
    assert_eq!(iter.next().unwrap(), &[]);
    assert_eq!(iter.next().unwrap(), &[20]);
    assert!(iter.next().is_none());

34. pub fn split_mut<F>(&mut self, pred: F) -> SplitMut<T, F>
    where
        F: FnMut(&T) -> bool,
    [src][−]
    Returns an iterator over mutable subslices separated by elements that match pred. The matched
    element is not contained in the subslices.

    /// Returns an iterator over mutable subslices separated by elements that
    /// match `pred`. The matched element is not contained in the subslices.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut v = [10, 40, 30, 20, 60, 50];
    ///
    /// for group in v.split_mut(|num| *num % 3 == 0) {
    ///     group[0] = 1;
    /// }
    /// assert_eq!(v, [1, 40, 30, 1, 60, 1]);
    /// ```
    #[stable(feature = "rust1", since = "1.0.0")]
    #[inline]
    pub fn split_mut<F>(&mut self, pred: F) -> SplitMut<'_, T, F>
    where
        F: FnMut(&T) -> bool,
    {
        SplitMut { v: self, pred, finished: false }
    }

    Examples
    let mut v = [10, 40, 30, 20, 60, 50];
    for group in v.split_mut(|num| *num % 3 == 0) {
        group[0] = 1;
    }
    assert_eq!(v, [1, 40, 30, 1, 60, 1]);

 */
fn main_1() {
    // split ：返回由匹配元素分割的子切片
    let slice = [10, 40, 33, 20];
    let mut iter = slice.split(|num| num % 3 == 0);
    assert_eq!(iter.next().unwrap(), &[10, 40]);
    assert_eq!(iter.next().unwrap(), &[20]);
    assert!(iter.next().is_none());
    // split : 首尾元素匹配则其位置返回一个空切片
    let slice = [6,10, 40, 33];
    let mut iter = slice.split(|num| num % 3 == 0);
    assert_eq!(iter.next().unwrap(), &[]);
    assert_eq!(iter.next().unwrap(), &[10, 40]);
    assert_eq!(iter.next().unwrap(), &[]);
    assert!(iter.next().is_none());
    // split : 相邻两元素匹配亦在其间返回一个空切片
    let slice = [10, 6,18,33, 20];
    let mut iter = slice.split(|num| num % 3 == 0);
    assert_eq!(iter.next().unwrap(), &[10]);
    assert_eq!(iter.next().unwrap(), &[]);
    assert_eq!(iter.next().unwrap(), &[]);
    assert_eq!(iter.next().unwrap(), &[20]);
    assert!(iter.next().is_none());
    // split : 首尾元素匹配或相邻两元素匹配则分别返回一个空切片
    let slice = [6,18];
    let mut iter = slice.split(|num| num % 3 == 0);
    assert_eq!(iter.next().unwrap(), &[]);
    assert_eq!(iter.next().unwrap(), &[]);
    assert_eq!(iter.next().unwrap(), &[]);
    assert!(iter.next().is_none());
    // split_mut
    let mut v = [10, 40, 30, 20, 60, 50];
    for group in v.split_mut(|num| *num % 3 == 0) {
        group[0] = 1;
    }
    assert_eq!(v, [1, 40, 30, 1, 60, 1]);
}

/*
35. pub fn split_inclusive<F>(&self, pred: F) -> SplitInclusive<T, F>
    where
        F: FnMut(&T) -> bool,
    [src][−]
    🔬 This is a nightly-only experimental API. (split_inclusive #72360)
    // 返回由匹配元素分割的子切片迭代器(匹配的元素包含在上一个子切片的末尾作为终止符)
    Returns an iterator over subslices separated by elements that match pred. The matched element is
    contained in the end of the previous subslice as a terminator.

    /// Returns an iterator over subslices separated by elements that match
    /// `pred`. The matched element is contained in the end of the previous
    /// subslice as a terminator.
    ///
    /// # Examples
    ///
    /// ```
    /// #![feature(split_inclusive)]
    /// let slice = [10, 40, 33, 20];
    /// let mut iter = slice.split_inclusive(|num| num % 3 == 0);
    ///
    /// assert_eq!(iter.next().unwrap(), &[10, 40, 33]);
    /// assert_eq!(iter.next().unwrap(), &[20]);
    /// assert!(iter.next().is_none());
    /// ```
    ///
    /// If the last element of the slice is matched,
    /// that element will be considered the terminator of the preceding slice.
    /// That slice will be the last item returned by the iterator.
    ///
    /// ```
    /// #![feature(split_inclusive)]
    /// let slice = [3, 10, 40, 33];
    /// let mut iter = slice.split_inclusive(|num| num % 3 == 0);
    ///
    /// assert_eq!(iter.next().unwrap(), &[3]);
    /// assert_eq!(iter.next().unwrap(), &[10, 40, 33]);
    /// assert!(iter.next().is_none());
    /// ```
    #[unstable(feature = "split_inclusive", issue = "72360")]
    #[inline]
    pub fn split_inclusive<F>(&self, pred: F) -> SplitInclusive<'_, T, F>
    where
        F: FnMut(&T) -> bool,
    {
        SplitInclusive { v: self, pred, finished: false }
    }

    Examples
    #![feature(split_inclusive)]
    let slice = [10, 40, 33, 20];
    let mut iter = slice.split_inclusive(|num| num % 3 == 0);
    assert_eq!(iter.next().unwrap(), &[10, 40, 33]);
    assert_eq!(iter.next().unwrap(), &[20]);
    assert!(iter.next().is_none());

    // 如果切片末元素匹配，则该元素将被视为前一个切片(迭代器返回的最后一个项)的终止符
    If the last element of the slice is matched, that element will be considered the terminator of
    the preceding slice. That slice will be the last item returned by the iterator.
    Examples
    #![feature(split_inclusive)]
    let slice = [3, 10, 40, 33];
    let mut iter = slice.split_inclusive(|num| num % 3 == 0);
    assert_eq!(iter.next().unwrap(), &[3]);
    assert_eq!(iter.next().unwrap(), &[10, 40, 33]);
    assert!(iter.next().is_none());

36. pub fn split_inclusive_mut<F>(&mut self, pred: F) -> SplitInclusiveMut<T, F>
    where
        F: FnMut(&T) -> bool,
    [src][−]
    🔬 This is a nightly-only experimental API. (split_inclusive #72360)
    Returns an iterator over mutable subslices separated by elements that match pred. The matched
    element is contained in the previous subslice as a terminator.

    /// Returns an iterator over mutable subslices separated by elements that
    /// match `pred`. The matched element is contained in the previous
    /// subslice as a terminator.
    ///
    /// # Examples
    ///
    /// ```
    /// #![feature(split_inclusive)]
    /// let mut v = [10, 40, 30, 20, 60, 50];
    ///
    /// for group in v.split_inclusive_mut(|num| *num % 3 == 0) {
    ///     let terminator_idx = group.len()-1;
    ///     group[terminator_idx] = 1;
    /// }
    /// assert_eq!(v, [10, 40, 1, 20, 1, 1]);
    /// ```
    #[unstable(feature = "split_inclusive", issue = "72360")]
    #[inline]
    pub fn split_inclusive_mut<F>(&mut self, pred: F) -> SplitInclusiveMut<'_, T, F>
    where
        F: FnMut(&T) -> bool,
    {
        SplitInclusiveMut { v: self, pred, finished: false }
    }

    Examples
    #![feature(split_inclusive)]
    let mut v = [10, 40, 30, 20, 60, 50];
    for group in v.split_inclusive_mut(|num| *num % 3 == 0) {
        let terminator_idx = group.len()-1;
        group[terminator_idx] = 1;
    }
    assert_eq!(v, [10, 40, 1, 20, 1, 1]);

 */
// 须标注" #![feature(split_inclusive)] "
fn main_2() {
    // split_inclusive ：返回由匹配元素分割的子切片迭代器(匹配的元素包含在上一个子切片的末尾作为终止符)
    let slice = [10, 40, 33, 20];
    let mut iter = slice.split_inclusive(|num| num % 3 == 0);
    assert_eq!(iter.next().unwrap(), &[10, 40, 33]);
    assert_eq!(iter.next().unwrap(), &[20]);
    assert!(iter.next().is_none());
    // split_inclusive : 若切片末元素匹配，则该元素将被视为前一个切片(迭代器返回的最后一个项)的终止符
    let slice = [3, 10, 40, 33];
    let mut iter = slice.split_inclusive(|num| num % 3 == 0);
    assert_eq!(iter.next().unwrap(), &[3]);
    assert_eq!(iter.next().unwrap(), &[10, 40, 33]);
    assert!(iter.next().is_none());
    // split_inclusive ：若元素匹配则该元素将被视为前一个切片的终止符
    let slice = [3, 18];
    let mut iter = slice.split_inclusive(|num| num % 3 == 0);
    assert_eq!(iter.next().unwrap(), &[3]);
    assert_eq!(iter.next().unwrap(), &[18]);
    assert!(iter.next().is_none());
    // split_inclusive_mut
    let mut v = [10, 40, 30, 20, 60, 50];
    for group in v.split_inclusive_mut(|num| *num % 3 == 0) {
        let terminator_idx = group.len()-1;
        group[terminator_idx] = 1;
    }
    assert_eq!(v, [10, 40, 1, 20, 1, 1]);
}

/*
37. pub fn rsplit<F>(&self, pred: F) -> RSplit<T, F>
    where
        F: FnMut(&T) -> bool,
    1.27.0
    [src]
    [−]
    // 返回由匹配元素从后向前分割切片的迭代器(匹配的元素不包含在子切片中)
    Returns an iterator over subslices separated by elements that match pred, starting at the end of
    the slice and working backwards. The matched element is not contained in the subslices.

    /// Returns an iterator over subslices separated by elements that match
    /// `pred`, starting at the end of the slice and working backwards.
    /// The matched element is not contained in the subslices.
    ///
    /// # Examples
    ///
    /// ```
    /// let slice = [11, 22, 33, 0, 44, 55];
    /// let mut iter = slice.rsplit(|num| *num == 0);
    ///
    /// assert_eq!(iter.next().unwrap(), &[44, 55]);
    /// assert_eq!(iter.next().unwrap(), &[11, 22, 33]);
    /// assert_eq!(iter.next(), None);
    /// ```
    ///
    /// As with `split()`, if the first or last element is matched, an empty
    /// slice will be the first (or last) item returned by the iterator.
    ///
    /// ```
    /// let v = &[0, 1, 1, 2, 3, 5, 8];
    /// let mut it = v.rsplit(|n| *n % 2 == 0);
    /// assert_eq!(it.next().unwrap(), &[]);
    /// assert_eq!(it.next().unwrap(), &[3, 5]);
    /// assert_eq!(it.next().unwrap(), &[1, 1]);
    /// assert_eq!(it.next().unwrap(), &[]);
    /// assert_eq!(it.next(), None);
    /// ```
    #[stable(feature = "slice_rsplit", since = "1.27.0")]
    #[inline]
    pub fn rsplit<F>(&self, pred: F) -> RSplit<'_, T, F>
    where
        F: FnMut(&T) -> bool,
    {
        RSplit { inner: self.split(pred) }
    }

    Examples
    let slice = [11, 22, 33, 0, 44, 55];
    let mut iter = slice.rsplit(|num| *num == 0);
    assert_eq!(iter.next().unwrap(), &[44, 55]);
    assert_eq!(iter.next().unwrap(), &[11, 22, 33]);
    assert_eq!(iter.next(), None);

    // 若首或尾元素匹配则其位置分别返回一个空切片
    As with split(), if the first or last element is matched, an empty slice will be the first (or
    last) item returned by the iterator.
    Examples
    let v = &[0, 1, 1, 2, 3, 5, 8];
    let mut it = v.rsplit(|n| *n % 2 == 0);
    assert_eq!(it.next().unwrap(), &[]);
    assert_eq!(it.next().unwrap(), &[3, 5]);
    assert_eq!(it.next().unwrap(), &[1, 1]);
    assert_eq!(it.next().unwrap(), &[]);
    assert_eq!(it.next(), None);
    // 相邻两元素匹配其间亦返回一个空切片
    // 首、尾或两元素间元素匹配则分别返回一个空切片

38. ub fn rsplit_mut<F>(&mut self, pred: F) -> RSplitMut<T, F>
    where
        F: FnMut(&T) -> bool,
    1.27.0
    [src]
    [−]
    // 返回由匹配元素分割的可变子切片迭代器
    Returns an iterator over mutable subslices separated by elements that match pred, starting at
    the end of the slice and working backwards. The matched element is not contained in the
    subslices.

    /// Returns an iterator over mutable subslices separated by elements that
    /// match `pred`, starting at the end of the slice and working
    /// backwards. The matched element is not contained in the subslices.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut v = [100, 400, 300, 200, 600, 500];
    ///
    /// let mut count = 0;
    /// for group in v.rsplit_mut(|num| *num % 3 == 0) {
    ///     count += 1;
    ///     group[0] = count;
    /// }
    /// assert_eq!(v, [3, 400, 300, 2, 600, 1]);
    /// ```
    ///
    #[stable(feature = "slice_rsplit", since = "1.27.0")]
    #[inline]
    pub fn rsplit_mut<F>(&mut self, pred: F) -> RSplitMut<'_, T, F>
    where
        F: FnMut(&T) -> bool,
    {
        RSplitMut { inner: self.split_mut(pred) }
    }

    Examples
    let mut v = [100, 400, 300, 200, 600, 500];
    let mut count = 0;
    for group in v.rsplit_mut(|num| *num % 3 == 0) {
        count += 1;
        group[0] = count;
    }
    assert_eq!(v, [3, 400, 300, 2, 600, 1]);

 */
fn main_3() {
    // rsplit
    let slice = [11, 22, 33, 0, 44, 55];
    let mut iter = slice.rsplit(|num| *num == 0);
    assert_eq!(iter.next().unwrap(), &[44, 55]);
    assert_eq!(iter.next().unwrap(), &[11, 22, 33]);
    assert_eq!(iter.next(), None);
    // rsplit : 若首或尾元素匹配则其位置分别返回一个空切片
    let v = &[0, 1, 1, 2, 3, 5, 8];
    let mut it = v.rsplit(|n| *n % 2 == 0);
    assert_eq!(it.next().unwrap(), &[]);
    assert_eq!(it.next().unwrap(), &[3, 5]);
    assert_eq!(it.next().unwrap(), &[1, 1]);
    assert_eq!(it.next().unwrap(), &[]);
    assert_eq!(it.next(), None);
    // split : 相邻两元素匹配其间亦返回一个空切片
    let v = &[0, 1, 1, 4, 2, 6, 3, 8];
    let mut it = v.rsplit(|n| *n % 2 == 0);
    assert_eq!(it.next().unwrap(), &[]);
    assert_eq!(it.next().unwrap(), &[3]);
    assert_eq!(it.next().unwrap(), &[]);
    assert_eq!(it.next().unwrap(), &[]);
    assert_eq!(it.next().unwrap(), &[1, 1]);
    assert_eq!(it.next().unwrap(), &[]);
    assert_eq!(it.next(), None);
    // rsplit : 首、尾或两元素间元素匹配则分别返回一个空切片
    let v = &[0, 6, 8];
    let mut it = v.rsplit(|n| *n % 2 == 0);
    assert_eq!(it.next().unwrap(), &[]);
    assert_eq!(it.next().unwrap(), &[]);
    assert_eq!(it.next().unwrap(), &[]);
    assert_eq!(it.next().unwrap(), &[]);
    assert_eq!(it.next(), None);

    // rsplit_mut :
    let mut v = [100, 400, 300, 200, 600, 500];
    let mut count = 0;
    for group in v.rsplit_mut(|num| *num % 3 == 0) {
        count += 1;
        group[0] = count;
    }
    assert_eq!(v, [3, 400, 300, 2, 600, 1]);

}

/*
39. pub fn splitn<F>(&self, n: usize, pred: F) -> SplitN<T, F>
    where
        F: FnMut(&T) -> bool,
    [src][−]
    // 返回由匹配元素分割的指定子切片最大数量的子项迭代器(匹配的元素不包含在子切片中)
    // 返回的最后一个子项(如果有)将包含切片的其余部分
    Returns an iterator over subslices separated by elements that match pred, limited to returning
    at most n items. The matched element is not contained in the subslices.

    The last element returned, if any, will contain the remainder of the slice.

    /// Returns an iterator over subslices separated by elements that match
    /// `pred`, limited to returning at most `n` items. The matched element is
    /// not contained in the subslices.
    ///
    /// The last element returned, if any, will contain the remainder of the
    /// slice.
    ///
    /// # Examples
    ///
    /// Print the slice split once by numbers divisible by 3 (i.e., `[10, 40]`,
    /// `[20, 60, 50]`):
    ///
    /// ```
    /// let v = [10, 40, 30, 20, 60, 50];
    ///
    /// for group in v.splitn(2, |num| *num % 3 == 0) {
    ///     println!("{:?}", group);
    /// }
    /// ```
    #[stable(feature = "rust1", since = "1.0.0")]
    #[inline]
    pub fn splitn<F>(&self, n: usize, pred: F) -> SplitN<'_, T, F>
    where
        F: FnMut(&T) -> bool,
    {
        SplitN { inner: GenericSplitN { iter: self.split(pred), count: n } }
    }

    Examples
    // Print the slice split once by numbers divisible by 3 (i.e., [10, 40], [20, 60, 50]):
    let v = [10, 40, 30, 20, 60, 50];
    for group in v.splitn(2, |num| *num % 3 == 0) {
        println!("{:?}", group);
    }

40. pub fn splitn_mut<F>(&mut self, n: usize, pred: F) -> SplitNMut<T, F>
    where
        F: FnMut(&T) -> bool,
    [src][−]
    // 返回由匹配元素分割的可变且指定最大子切片数量的迭代器(匹配的元素不包含在子切片中)
    Returns an iterator over subslices separated by elements that match pred, limited to returning
    at most n items. The matched element is not contained in the subslices.

    The last element returned, if any, will contain the remainder of the slice.

    /// Returns an iterator over subslices separated by elements that match
    /// `pred`, limited to returning at most `n` items. The matched element is
    /// not contained in the subslices.
    ///
    /// The last element returned, if any, will contain the remainder of the
    /// slice.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut v = [10, 40, 30, 20, 60, 50];
    ///
    /// for group in v.splitn_mut(2, |num| *num % 3 == 0) {
    ///     group[0] = 1;
    /// }
    /// assert_eq!(v, [1, 40, 30, 1, 60, 50]);
    /// ```
    #[stable(feature = "rust1", since = "1.0.0")]
    #[inline]
    pub fn splitn_mut<F>(&mut self, n: usize, pred: F) -> SplitNMut<'_, T, F>
    where
        F: FnMut(&T) -> bool,
    {
        SplitNMut { inner: GenericSplitN { iter: self.split_mut(pred), count: n } }
    }

    Examples
    let mut v = [10, 40, 30, 20, 60, 50];
    for group in v.splitn_mut(2, |num| *num % 3 == 0) {
        group[0] = 1;
    }
    assert_eq!(v, [1, 40, 30, 1, 60, 50]);

 */
fn main_4() {
    // splitn : 至多返回 n 个子项
    let v = [10, 40, 30, 20, 60, 50];
    for group in v.splitn(2, |num| *num % 3 == 0) {
        print!("{:?}\t", group);    // Console:" [10, 40]   [20, 60, 50] "
    }

    // splitn_mut
    let mut v = [10, 40, 30, 20, 60, 50];
    for group in v.splitn_mut(2, |num| *num % 3 == 0) {
        group[0] = 1;
    }
    assert_eq!(v, [1, 40, 30, 1, 60, 50]);
}

/*
41. pub fn rsplitn<F>(&self, n: usize, pred: F) -> RSplitN<T, F>
    where
        F: FnMut(&T) -> bool,
    [src][−]
    // 返回由匹配元素分割(从后向前)且指定最大数量切片的子项迭代器(匹配的元素不包含在子切片中)
    Returns an iterator over subslices separated by elements that match pred limited to returning
    at most n items. This starts at the end of the slice and works backwards. The matched element is
    not contained in the subslices.

    The last element returned, if any, will contain the remainder of the slice.

    /// Returns an iterator over subslices separated by elements that match
    /// `pred` limited to returning at most `n` items. This starts at the end of
    /// the slice and works backwards. The matched element is not contained in
    /// the subslices.
    ///
    /// The last element returned, if any, will contain the remainder of the
    /// slice.
    ///
    /// # Examples
    ///
    /// Print the slice split once, starting from the end, by numbers divisible
    /// by 3 (i.e., `[50]`, `[10, 40, 30, 20]`):
    ///
    /// ```
    /// let v = [10, 40, 30, 20, 60, 50];
    ///
    /// for group in v.rsplitn(2, |num| *num % 3 == 0) {
    ///     println!("{:?}", group);
    /// }
    /// ```
    #[stable(feature = "rust1", since = "1.0.0")]
    #[inline]
    pub fn rsplitn<F>(&self, n: usize, pred: F) -> RSplitN<'_, T, F>
    where
        F: FnMut(&T) -> bool,
    {
        RSplitN { inner: GenericSplitN { iter: self.rsplit(pred), count: n } }
    }

    Examples
    // Print the slice split once, starting from the end, by numbers divisible by 3
    // (i.e., [50], [10, 40, 30, 20]):
    let v = [10, 40, 30, 20, 60, 50];
    for group in v.rsplitn(2, |num| *num % 3 == 0) {
        println!("{:?}", group);
    }

42. pub fn rsplitn_mut<F>(&mut self, n: usize, pred: F) -> RSplitNMut<T, F>
    where
        F: FnMut(&T) -> bool,
    [src][−]
    // 返回由匹配元素分割(从后向前)的可变且指定最大子切片数量的迭代器(子切片未含匹配元素)
    Returns an iterator over subslices separated by elements that match pred limited to returning
    at most n items. This starts at the end of the slice and works backwards. The matched element is
    not contained in the subslices.

    The last element returned, if any, will contain the remainder of the slice.

    /// Returns an iterator over subslices separated by elements that match
    /// `pred` limited to returning at most `n` items. This starts at the end of
    /// the slice and works backwards. The matched element is not contained in
    /// the subslices.
    ///
    /// The last element returned, if any, will contain the remainder of the
    /// slice.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut s = [10, 40, 30, 20, 60, 50];
    ///
    /// for group in s.rsplitn_mut(2, |num| *num % 3 == 0) {
    ///     group[0] = 1;
    /// }
    /// assert_eq!(s, [1, 40, 30, 20, 60, 1]);
    /// ```
    #[stable(feature = "rust1", since = "1.0.0")]
    #[inline]
    pub fn rsplitn_mut<F>(&mut self, n: usize, pred: F) -> RSplitNMut<'_, T, F>
    where
        F: FnMut(&T) -> bool,
    {
        RSplitNMut { inner: GenericSplitN { iter: self.rsplit_mut(pred), count: n } }
    }

    Examples
    let mut s = [10, 40, 30, 20, 60, 50];
    for group in s.rsplitn_mut(2, |num| *num % 3 == 0) {
        group[0] = 1;
    }
    assert_eq!(s, [1, 40, 30, 20, 60, 1]);

*/

fn main() {
    // rsplitn
    let v = [10, 40, 30, 20, 60, 50];
    for group in v.rsplitn(2, |num| *num % 3 == 0) {
        print!("{:?}\t", group); // Console:" [50]	[10, 40, 30, 20] "
    }

    // rsplitn_mut
    let mut s = [10, 40, 30, 20, 60, 50];
    for group in s.rsplitn_mut(2, |num| *num % 3 == 0) {
        group[0] = 1;
    }
    assert_eq!(s, [1, 40, 30, 20, 60, 1]);
}