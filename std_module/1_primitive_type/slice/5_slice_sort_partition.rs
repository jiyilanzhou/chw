
/*
Primitive Type slice ：
    文档:" https://doc.rust-lang.org/std/primitive.slice.html "

51. pub fn sort_unstable(&mut self)
    where
        T: Ord,
    1.20.0
    [src]
    [−]
    // 对切片进行排序但可能不保留相等元素的顺序(非稳定排序)
    Sorts the slice, but may not preserve the order of equal elements.

    This sort is unstable (i.e., may reorder equal elements), in-place (i.e., does not allocate),
     O(n * log(n)) worst-case.

    Current implementation
    The current algorithm is based on pattern-defeating quicksort by Orson Peters, which combines
    the fast average case of randomized quicksort with the fast worst case of heapsort, while
    achieving linear time on slices with certain patterns. It uses some randomization to avoid
    degenerate cases, but with a fixed seed to always provide deterministic behavior.

    It is typically faster than stable sorting, except in a few special cases, e.g., when the slice
    consists of several concatenated sorted sequences.

    /// Sorts the slice, but may not preserve the order of equal elements.
    ///
    /// This sort is unstable (i.e., may reorder equal elements), in-place
    /// (i.e., does not allocate), and `O(n * log(n))` worst-case.
    ///
    /// # Current implementation
    ///
    /// The current algorithm is based on [pattern-defeating quicksort][pdqsort] by Orson Peters,
    /// which combines the fast average case of randomized quicksort with the fast worst case of
    /// heapsort, while achieving linear time on slices with certain patterns. It uses some
    /// randomization to avoid degenerate cases, but with a fixed seed to always provide
    /// deterministic behavior.
    ///
    /// It is typically faster than stable sorting, except in a few special cases, e.g., when the
    /// slice consists of several concatenated sorted sequences.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut v = [-5, 4, 1, -3, 2];
    ///
    /// v.sort_unstable();
    /// assert!(v == [-5, -3, 1, 2, 4]);
    /// ```
    ///
    /// [pdqsort]: https://github.com/orlp/pdqsort
    #[stable(feature = "sort_unstable", since = "1.20.0")]
    #[inline]
    pub fn sort_unstable(&mut self)
    where
        T: Ord,
    {
        sort::quicksort(self, |a, b| a.lt(b));
    }

    Examples
    let mut v = [-5, 4, 1, -3, 2];
    v.sort_unstable();
    assert!(v == [-5, -3, 1, 2, 4]);

52. pub fn sort_unstable_by<F>(&mut self, compare: F)
    where
        F: FnMut(&T, &T) -> Ordering,
    1.20.0
    [src]
    [−]
    // 使用比较器功能对切片进行排序，但可能不会保留相等元素的顺序(非稳定排序)
    Sorts the slice with a comparator function, but may not preserve the order of equal elements.

    This sort is unstable (i.e., may reorder equal elements), in-place (i.e., does not allocate),
    and O(n * log(n)) worst-case.

    The comparator function must define a total ordering for the elements in the slice. If the
    ordering is not total, the order of the elements is unspecified. An order is a total order if it
    is (for all a, b and c):

        total and antisymmetric: exactly one of a < b, a == b or a > b is true; and
        transitive, a < b and b < c implies a < c. The same must hold for both == and >.

    For example, while f64 doesn't implement Ord because NaN != NaN, we can use partial_cmp as our
    sort function when we know the slice doesn't contain a NaN.

        let mut floats = [5f64, 4.0, 1.0, 3.0, 2.0];
        floats.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
        assert_eq!(floats, [1.0, 2.0, 3.0, 4.0, 5.0]);
        Run

    Current implementation
    The current algorithm is based on pattern-defeating quicksort by Orson Peters, which combines
    the fast average case of randomized quicksort with the fast worst case of heapsort, while
    achieving linear time on slices with certain patterns. It uses some randomization to avoid
    degenerate cases, but with a fixed seed to always provide deterministic behavior.

    It is typically faster than stable sorting, except in a few special cases, e.g., when the slice
    consists of several concatenated sorted sequences.

    /// Sorts the slice with a comparator function, but may not preserve the order of equal
    /// elements.
    ///
    /// This sort is unstable (i.e., may reorder equal elements), in-place
    /// (i.e., does not allocate), and `O(n * log(n))` worst-case.
    ///
    /// The comparator function must define a total ordering for the elements in the slice. If
    /// the ordering is not total, the order of the elements is unspecified. An order is a
    /// total order if it is (for all a, b and c):
    ///
    /// * total and antisymmetric: exactly one of a < b, a == b or a > b is true; and
    /// * transitive, a < b and b < c implies a < c. The same must hold for both == and >.
    ///
    /// For example, while [`f64`] doesn't implement [`Ord`] because `NaN != NaN`, we can use
    /// `partial_cmp` as our sort function when we know the slice doesn't contain a `NaN`.
    ///
    /// ```
    /// let mut floats = [5f64, 4.0, 1.0, 3.0, 2.0];
    /// floats.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
    /// assert_eq!(floats, [1.0, 2.0, 3.0, 4.0, 5.0]);
    /// ```
    ///
    /// # Current implementation
    ///
    /// The current algorithm is based on [pattern-defeating quicksort][pdqsort] by Orson Peters,
    /// which combines the fast average case of randomized quicksort with the fast worst case of
    /// heapsort, while achieving linear time on slices with certain patterns. It uses some
    /// randomization to avoid degenerate cases, but with a fixed seed to always provide
    /// deterministic behavior.
    ///
    /// It is typically faster than stable sorting, except in a few special cases, e.g., when the
    /// slice consists of several concatenated sorted sequences.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut v = [5, 4, 1, 3, 2];
    /// v.sort_unstable_by(|a, b| a.cmp(b));
    /// assert!(v == [1, 2, 3, 4, 5]);
    ///
    /// // reverse sorting
    /// v.sort_unstable_by(|a, b| b.cmp(a));
    /// assert!(v == [5, 4, 3, 2, 1]);
    /// ```
    ///
    /// [pdqsort]: https://github.com/orlp/pdqsort
    #[stable(feature = "sort_unstable", since = "1.20.0")]
    #[inline]
    pub fn sort_unstable_by<F>(&mut self, mut compare: F)
    where
        F: FnMut(&T, &T) -> Ordering,
    {
        sort::quicksort(self, |a, b| compare(a, b) == Ordering::Less);
    }

    Examples
    let mut v = [5, 4, 1, 3, 2];
    v.sort_unstable_by(|a, b| a.cmp(b));
    assert!(v == [1, 2, 3, 4, 5]);
    // reverse sorting
    v.sort_unstable_by(|a, b| b.cmp(a));
    assert!(v == [5, 4, 3, 2, 1]);

53. pub fn sort_unstable_by_key<K, F>(&mut self, f: F)
    where
        F: FnMut(&T) -> K,
        K: Ord,
    1.20.0
    [src]
    [−]
    // 使用键提取功能对切片进行排序，但可能不会保留相等元素的顺序
    Sorts the slice with a key extraction function, but may not preserve the order of equal elements.

    This sort is unstable (i.e., may reorder equal elements), in-place (i.e., does not allocate),
    and O(m * n * log(n)) worst-case, where the key function is O(m).

    Current implementation
    The current algorithm is based on pattern-defeating quicksort by Orson Peters, which combines
    the fast average case of randomized quicksort with the fast worst case of heapsort, while
    achieving linear time on slices with certain patterns. It uses some randomization to avoid
    degenerate cases, but with a fixed seed to always provide deterministic behavior.

    Due to its key calling strategy, sort_unstable_by_key is likely to be slower than
    sort_by_cached_key in cases where the key function is expensive.

    /// Sorts the slice with a key extraction function, but may not preserve the order of equal
    /// elements.
    ///
    /// This sort is unstable (i.e., may reorder equal elements), in-place
    /// (i.e., does not allocate), and `O(m * n * log(n))` worst-case, where the key function is
    /// `O(m)`.
    ///
    /// # Current implementation
    ///
    /// The current algorithm is based on [pattern-defeating quicksort][pdqsort] by Orson Peters,
    /// which combines the fast average case of randomized quicksort with the fast worst case of
    /// heapsort, while achieving linear time on slices with certain patterns. It uses some
    /// randomization to avoid degenerate cases, but with a fixed seed to always provide
    /// deterministic behavior.
    ///
    /// Due to its key calling strategy, [`sort_unstable_by_key`](#method.sort_unstable_by_key)
    /// is likely to be slower than [`sort_by_cached_key`](#method.sort_by_cached_key) in
    /// cases where the key function is expensive.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut v = [-5i32, 4, 1, -3, 2];
    ///
    /// v.sort_unstable_by_key(|k| k.abs());
    /// assert!(v == [1, 2, -3, 4, -5]);
    /// ```
    ///
    /// [pdqsort]: https://github.com/orlp/pdqsort
    #[stable(feature = "sort_unstable", since = "1.20.0")]
    #[inline]
    pub fn sort_unstable_by_key<K, F>(&mut self, mut f: F)
    where
        F: FnMut(&T) -> K,
        K: Ord,
    {
        sort::quicksort(self, |a, b| f(a).lt(&f(b)));
    }

    Examples
    let mut v = [-5i32, 4, 1, -3, 2];
    v.sort_unstable_by_key(|k| k.abs());
    assert!(v == [1, 2, -3, 4, -5]);

*/
#![feature(slice_partition_at_index)]
fn main_0() {
    // sort_unstable_by
    let mut v = [5, 4, 1, 3, 2];
    v.sort_unstable_by(|a, b| a.cmp(b));
    assert!(v == [1, 2, 3, 4, 5]);
    // reverse sorting
    v.sort_unstable_by(|a, b| b.cmp(a));
    assert!(v == [5, 4, 3, 2, 1]);

    // sort_unstable_by_key
    let mut v = [-5i32, 4, 1, -3, 2];
    v.sort_unstable_by_key(|k| k.abs());
    assert!(v == [1, 2, -3, 4, -5]);
}

/*
54. pub fn partition_at_index(
        &mut self,
        index: usize
    ) -> (&mut [T], &mut T, &mut [T])
    where
        T: Ord,
    [src][−]
    🔬 This is a nightly-only experimental API. (slice_partition_at_index #55300)
    // 重新排序切片，使目标索引元素处于其最终排序位置([自]类比快排:确定目标元素位置)
    Reorder the slice such that the element at index is at its final sorted position.

    This reordering has the additional property that any value at position i < index will be less
    than or equal to any value at a position j > index. Additionally, this reordering is unstable
    (i.e. any number of equal elements may end up at position index), in-place (i.e. does not
    allocate), and O(n) worst-case. This function is also/ known as "kth element" in other libraries.
    It returns a triplet of the following values: all elements less than the one at the given index,
    the value at the given index, and all elements greater than the one at the given index.

    Current implementation
    // 当前算法基于用于的相同快速排序算法的快速选择部分 sort_unstable
    The current algorithm is based on the quickselect portion of the same quicksort algorithm used
    for sort_unstable.

    Panics
    Panics when index >= len(), meaning it always panics on empty slices.

    /// Reorder the slice such that the element at `index` is at its final sorted position.
    ///
    /// This reordering has the additional property that any value at position `i < index` will be
    /// less than or equal to any value at a position `j > index`. Additionally, this reordering is
    /// unstable (i.e. any number of equal elements may end up at position `index`), in-place
    /// (i.e. does not allocate), and `O(n)` worst-case. This function is also/ known as "kth
    /// element" in other libraries. It returns a triplet of the following values: all elements less
    /// than the one at the given index, the value at the given index, and all elements greater than
    /// the one at the given index.
    ///
    /// # Current implementation
    ///
    /// The current algorithm is based on the quickselect portion of the same quicksort algorithm
    /// used for [`sort_unstable`].
    ///
    /// [`sort_unstable`]: #method.sort_unstable
    ///
    /// # Panics
    ///
    /// Panics when `index >= len()`, meaning it always panics on empty slices.
    ///
    /// # Examples
    ///
    /// ```
    /// #![feature(slice_partition_at_index)]
    ///
    /// let mut v = [-5i32, 4, 1, -3, 2];
    ///
    /// // Find the median
    /// v.partition_at_index(2);
    ///
    /// // We are only guaranteed the slice will be one of the following, based on the way we sort
    /// // about the specified index.
    /// assert!(v == [-3, -5, 1, 2, 4] ||
    ///         v == [-5, -3, 1, 2, 4] ||
    ///         v == [-3, -5, 1, 4, 2] ||
    ///         v == [-5, -3, 1, 4, 2]);
    /// ```
    #[unstable(feature = "slice_partition_at_index", issue = "55300")]
    #[inline]
    pub fn partition_at_index(&mut self, index: usize) -> (&mut [T], &mut T, &mut [T])
    where
        T: Ord,
    {
        let mut f = |a: &T, b: &T| a.lt(b);
        sort::partition_at_index(self, index, &mut f)
    }

    Examples
    #![feature(slice_partition_at_index)]
    let mut v = [-5i32, 4, 1, -3, 2];
    // Find the median
    v.partition_at_index(2);
    // We are only guaranteed the slice will be one of the following, based on the way we sort
    // about the specified index.
    assert!(v == [-3, -5, 1, 2, 4] ||
            v == [-5, -3, 1, 2, 4] ||
            v == [-3, -5, 1, 4, 2] ||
            v == [-5, -3, 1, 4, 2]);

55. pub fn partition_at_index_by<F>(
        &mut self,
        index: usize,
        compare: F
    ) -> (&mut [T], &mut T, &mut [T])
    where
        F: FnMut(&T, &T) -> Ordering,
    [src][−]
    🔬 This is a nightly-only experimental API. (slice_partition_at_index #55300)
    // 使用比较器功能对切片进行重新排序，以使 index 索引处的元素处于其最终排序位置
    Reorder the slice with a comparator function such that the element at index is at its final
    sorted position.

    This reordering has the additional property that any value at position i < index will be less
    than or equal to any value at a position j > index using the comparator function. Additionally,
    this reordering is unstable (i.e. any number of equal elements may end up at position index),
    in-place (i.e. does not allocate), and O(n) worst-case. This function is also known as "kth
    element" in other libraries. It returns a triplet of the following values: all elements less
    than the one at the given index, the value at the given index, and all elements greater than the
    one at the given index, using the provided comparator function.

    Current implementation
    The current algorithm is based on the quickselect portion of the same quicksort algorithm used
    for sort_unstable.

    Panics
    Panics when index >= len(), meaning it always panics on empty slices.

   /// Reorder the slice with a comparator function such that the element at `index` is at its
    /// final sorted position.
    ///
    /// This reordering has the additional property that any value at position `i < index` will be
    /// less than or equal to any value at a position `j > index` using the comparator function.
    /// Additionally, this reordering is unstable (i.e. any number of equal elements may end up at
    /// position `index`), in-place (i.e. does not allocate), and `O(n)` worst-case. This function
    /// is also known as "kth element" in other libraries. It returns a triplet of the following
    /// values: all elements less than the one at the given index, the value at the given index,
    /// and all elements greater than the one at the given index, using the provided comparator
    /// function.
    ///
    /// # Current implementation
    ///
    /// The current algorithm is based on the quickselect portion of the same quicksort algorithm
    /// used for [`sort_unstable`].
    ///
    /// [`sort_unstable`]: #method.sort_unstable
    ///
    /// # Panics
    ///
    /// Panics when `index >= len()`, meaning it always panics on empty slices.
    ///
    /// # Examples
    ///
    /// ```
    /// #![feature(slice_partition_at_index)]
    ///
    /// let mut v = [-5i32, 4, 1, -3, 2];
    ///
    /// // Find the median as if the slice were sorted in descending order.
    /// v.partition_at_index_by(2, |a, b| b.cmp(a));
    ///
    /// // We are only guaranteed the slice will be one of the following, based on the way we sort
    /// // about the specified index.
    /// assert!(v == [2, 4, 1, -5, -3] ||
    ///         v == [2, 4, 1, -3, -5] ||
    ///         v == [4, 2, 1, -5, -3] ||
    ///         v == [4, 2, 1, -3, -5]);
    /// ```
    #[unstable(feature = "slice_partition_at_index", issue = "55300")]
    #[inline]
    pub fn partition_at_index_by<F>(
        &mut self,
        index: usize,
        mut compare: F,
    ) -> (&mut [T], &mut T, &mut [T])
    where
        F: FnMut(&T, &T) -> Ordering,
    {
        let mut f = |a: &T, b: &T| compare(a, b) == Less;
        sort::partition_at_index(self, index, &mut f)
    }

    Examples
    #![feature(slice_partition_at_index)]
    let mut v = [-5i32, 4, 1, -3, 2];
    // Find the median as if the slice were sorted in descending order.
    v.partition_at_index_by(2, |a, b| b.cmp(a));
    // We are only guaranteed the slice will be one of the following, based on the way we sort
    // about the specified index.
    assert!(v == [2, 4, 1, -5, -3] ||
            v == [2, 4, 1, -3, -5] ||
            v == [4, 2, 1, -5, -3] ||
            v == [4, 2, 1, -3, -5]);

56. pub fn partition_at_index_by_key<K, F>(
        &mut self,
        index: usize,
        f: F
    ) -> (&mut [T], &mut T, &mut [T])
    where
        F: FnMut(&T) -> K,
        K: Ord,
    [src][−]
    🔬 This is a nightly-only experimental API. (slice_partition_at_index #55300)
    Reorder the slice with a key extraction function such that the element at index is at its final
    sorted position.

    This reordering has the additional property that any value at position i < index will be less
    than or equal to any value at a position j > index using the key extraction function.
    Additionally, this reordering is unstable (i.e. any number of equal elements may end up at
    position index), in-place (i.e. does not allocate), and O(n) worst-case. This function is also
    known as "kth element" in other libraries. It returns a triplet of the following values: all
    elements less than the one at the given index, the value at the given index, and all elements
    greater than the one at the given index, using the provided key extraction function.

    Current implementation
    The current algorithm is based on the quickselect portion of the same quicksort algorithm used
    for sort_unstable.

    Panics
    Panics when index >= len(), meaning it always panics on empty slices.

    /// Reorder the slice with a key extraction function such that the element at `index` is at its
    /// final sorted position.
    ///
    /// This reordering has the additional property that any value at position `i < index` will be
    /// less than or equal to any value at a position `j > index` using the key extraction function.
    /// Additionally, this reordering is unstable (i.e. any number of equal elements may end up at
    /// position `index`), in-place (i.e. does not allocate), and `O(n)` worst-case. This function
    /// is also known as "kth element" in other libraries. It returns a triplet of the following
    /// values: all elements less than the one at the given index, the value at the given index, and
    /// all elements greater than the one at the given index, using the provided key extraction
    /// function.
    ///
    /// # Current implementation
    ///
    /// The current algorithm is based on the quickselect portion of the same quicksort algorithm
    /// used for [`sort_unstable`].
    ///
    /// [`sort_unstable`]: #method.sort_unstable
    ///
    /// # Panics
    ///
    /// Panics when `index >= len()`, meaning it always panics on empty slices.
    ///
    /// # Examples
    ///
    /// ```
    /// #![feature(slice_partition_at_index)]
    ///
    /// let mut v = [-5i32, 4, 1, -3, 2];
    ///
    /// // Return the median as if the array were sorted according to absolute value.
    /// v.partition_at_index_by_key(2, |a| a.abs());
    ///
    /// // We are only guaranteed the slice will be one of the following, based on the way we sort
    /// // about the specified index.
    /// assert!(v == [1, 2, -3, 4, -5] ||
    ///         v == [1, 2, -3, -5, 4] ||
    ///         v == [2, 1, -3, 4, -5] ||
    ///         v == [2, 1, -3, -5, 4]);
    /// ```
    #[unstable(feature = "slice_partition_at_index", issue = "55300")]
    #[inline]
    pub fn partition_at_index_by_key<K, F>(
        &mut self,
        index: usize,
        mut f: F,
    ) -> (&mut [T], &mut T, &mut [T])
    where
        F: FnMut(&T) -> K,
        K: Ord,
    {
        let mut g = |a: &T, b: &T| f(a).lt(&f(b));
        sort::partition_at_index(self, index, &mut g)
    }

    Examples
    #![feature(slice_partition_at_index)]
    let mut v = [-5i32, 4, 1, -3, 2];
    // Return the median as if the array were sorted according to absolute value.
    v.partition_at_index_by_key(2, |a| a.abs());
    // We are only guaranteed the slice will be one of the following, based on the way we sort
    // about the specified index.
    assert!(v == [1, 2, -3, 4, -5] ||
            v == [1, 2, -3, -5, 4] ||
            v == [2, 1, -3, 4, -5] ||
            v == [2, 1, -3, -5, 4]);

 */
// 须标注"    #![feature(slice_partition_at_index)] "
fn main() {
    // partition_at_index
    let mut v = [-5i32, 4, 1, -3, 2];
    // 找到中位数 Find the median
    v.partition_at_index(2);
    // We are only guaranteed the slice will be one of the following, based on the way we sort
    // about the specified index.
    assert!(v == [-3, -5, 1, 2, 4] ||
        v == [-5, -3, 1, 2, 4] ||
        v == [-3, -5, 1, 4, 2] ||
        v == [-5, -3, 1, 4, 2]);

    // partition_at_index_by
    let mut v = [-5i32, 4, 1, -3, 2];
    // Find the median as if the slice were sorted in descending order.
    v.partition_at_index_by(2, |a, b| b.cmp(a));
    // We are only guaranteed the slice will be one of the following, based on the way we sort
    // about the specified index.
    assert!(v == [2, 4, 1, -5, -3] ||
        v == [2, 4, 1, -3, -5] ||
        v == [4, 2, 1, -5, -3] ||
        v == [4, 2, 1, -3, -5]);

    // partition_at_index_by_key
    let mut v = [-5i32, 4, 1, -3, 2];
    // Return the median as if the array were sorted according to absolute value.
    v.partition_at_index_by_key(2, |a| a.abs());
    // We are only guaranteed the slice will be one of the following, based on the way we sort
    // about the specified index.
    assert!(v == [1, 2, -3, 4, -5] ||
        v == [1, 2, -3, -5, 4] ||
        v == [2, 1, -3, 4, -5] ||
        v == [2, 1, -3, -5, 4]);

}