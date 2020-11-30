
// swap[swɒp]v&n. 交换,调换
/*
Primitive Type slice ：
    文档:" https://doc.rust-lang.org/std/primitive.slice.html "

18. pub fn swap(&mut self, a: usize, b: usize)
    [src][−]
    Swaps two elements in the slice.

    Arguments
    a - The index of the first element
    b - The index of the second element

    Panics
    Panics if a or b are out of bounds.

    /// Swaps two elements in the slice.
    ///
    /// # Arguments
    ///
    /// * a - The index of the first element
    /// * b - The index of the second element
    ///
    /// # Panics
    ///
    /// Panics if `a` or `b` are out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut v = ["a", "b", "c", "d"];
    /// v.swap(1, 3);
    /// assert!(v == ["a", "d", "c", "b"]);
    /// ```
    #[stable(feature = "rust1", since = "1.0.0")]
    #[inline]
    pub fn swap(&mut self, a: usize, b: usize) {
        unsafe {
            // Can't take two mutable loans from one vector, so instead just cast
            // them to their raw pointers to do the swap
            let pa: *mut T = &mut self[a];
            let pb: *mut T = &mut self[b];
            ptr::swap(pa, pb);
        }
    }

    Examples
    let mut v = ["a", "b", "c", "d"];
    v.swap(1, 3);
    assert!(v == ["a", "d", "c", "b"]);

19. pub fn reverse(&mut self)
    [src][−]
    Reverses the order of elements in the slice, in place.

    /// Reverses the order of elements in the slice, in place.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut v = [1, 2, 3];
    /// v.reverse();
    /// assert!(v == [3, 2, 1]);
    /// ```
    #[stable(feature = "rust1", since = "1.0.0")]
    #[inline]
    pub fn reverse(&mut self) {
        let mut i: usize = 0;
        let ln = self.len();

        // For very small types, all the individual reads in the normal
        // path perform poorly.  We can do better, given efficient unaligned
        // load/store, by loading a larger chunk and reversing a register.

        // Ideally LLVM would do this for us, as it knows better than we do
        // whether unaligned reads are efficient (since that changes between
        // different ARM versions, for example) and what the best chunk size
        // would be.  Unfortunately, as of LLVM 4.0 (2017-05) it only unrolls
        // the loop, so we need to do this ourselves.  (Hypothesis: reverse
        // is troublesome because the sides can be aligned differently --
        // will be, when the length is odd -- so there's no way of emitting
        // pre- and postludes to use fully-aligned SIMD in the middle.)

        let fast_unaligned = cfg!(any(target_arch = "x86", target_arch = "x86_64"));

        if fast_unaligned && mem::size_of::<T>() == 1 {
            // Use the llvm.bswap intrinsic to reverse u8s in a usize
            let chunk = mem::size_of::<usize>();
            while i + chunk - 1 < ln / 2 {
                unsafe {
                    let pa: *mut T = self.get_unchecked_mut(i);
                    let pb: *mut T = self.get_unchecked_mut(ln - i - chunk);
                    let va = ptr::read_unaligned(pa as *mut usize);
                    let vb = ptr::read_unaligned(pb as *mut usize);
                    ptr::write_unaligned(pa as *mut usize, vb.swap_bytes());
                    ptr::write_unaligned(pb as *mut usize, va.swap_bytes());
                }
                i += chunk;
            }
        }

        if fast_unaligned && mem::size_of::<T>() == 2 {
            // Use rotate-by-16 to reverse u16s in a u32
            let chunk = mem::size_of::<u32>() / 2;
            while i + chunk - 1 < ln / 2 {
                unsafe {
                    let pa: *mut T = self.get_unchecked_mut(i);
                    let pb: *mut T = self.get_unchecked_mut(ln - i - chunk);
                    let va = ptr::read_unaligned(pa as *mut u32);
                    let vb = ptr::read_unaligned(pb as *mut u32);
                    ptr::write_unaligned(pa as *mut u32, vb.rotate_left(16));
                    ptr::write_unaligned(pb as *mut u32, va.rotate_left(16));
                }
                i += chunk;
            }
        }

        while i < ln / 2 {
            // Unsafe swap to avoid the bounds check in safe swap.
            unsafe {
                let pa: *mut T = self.get_unchecked_mut(i);
                let pb: *mut T = self.get_unchecked_mut(ln - i - 1);
                ptr::swap(pa, pb);
            }
            i += 1;
        }
    }

    Examples
    let mut v = [1, 2, 3];
    v.reverse();
    assert!(v == [3, 2, 1]);

20. pub fn iter(&self) -> Iter<T>
    [src][−]
    Returns an iterator over the slice.

    /// Returns an iterator over the slice.
    ///
    /// # Examples
    ///
    /// ```
    /// let x = &[1, 2, 4];
    /// let mut iterator = x.iter();
    ///
    /// assert_eq!(iterator.next(), Some(&1));
    /// assert_eq!(iterator.next(), Some(&2));
    /// assert_eq!(iterator.next(), Some(&4));
    /// assert_eq!(iterator.next(), None);
    /// ```
    #[stable(feature = "rust1", since = "1.0.0")]
    #[inline]
    pub fn iter(&self) -> Iter<'_, T> {
        unsafe {
            let ptr = self.as_ptr();
            assume(!ptr.is_null());

            let end = if mem::size_of::<T>() == 0 {
                (ptr as *const u8).wrapping_add(self.len()) as *const T
            } else {
                ptr.add(self.len())
            };

            Iter { ptr: NonNull::new_unchecked(ptr as *mut T), end, _marker: marker::PhantomData }
        }
    }

    Examples
    let x = &[1, 2, 4];
    let mut iterator = x.iter();
    assert_eq!(iterator.next(), Some(&1));
    assert_eq!(iterator.next(), Some(&2));
    assert_eq!(iterator.next(), Some(&4));
    assert_eq!(iterator.next(), None);

21. pub fn iter_mut(&mut self) -> IterMut<T>
    [src][−]
    Returns an iterator that allows modifying each value.

    Panics
    Panics if size is 0.

    /// Returns an iterator that allows modifying each value.
    ///
    /// # Examples
    ///
    /// ```
    /// let x = &mut [1, 2, 4];
    /// for elem in x.iter_mut() {
    ///     *elem += 2;
    /// }
    /// assert_eq!(x, &[3, 4, 6]);
    /// ```
    #[stable(feature = "rust1", since = "1.0.0")]
    #[inline]
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        unsafe {
            let ptr = self.as_mut_ptr();
            assume(!ptr.is_null());

            let end = if mem::size_of::<T>() == 0 {
                (ptr as *mut u8).wrapping_add(self.len()) as *mut T
            } else {
                ptr.add(self.len())
            };

            IterMut { ptr: NonNull::new_unchecked(ptr), end, _marker: marker::PhantomData }
        }
    }

    Examples
    let x = &mut [1, 2, 4];
    for elem in x.iter_mut() {
        *elem += 2;
    }
    assert_eq!(x, &[3, 4, 6]);

22. pub fn windows(&self, size: usize) -> Windows<T>
    [src][−]
    // 返回所有指定大小的连续窗口的迭代器
    Returns an iterator over all contiguous windows of length size. The windows overlap. If the
    slice is shorter than size, the iterator returns no values.

    Panics
    Panics if size is 0.

    /// Returns an iterator over all contiguous windows of length
    /// `size`. The windows overlap. If the slice is shorter than
    /// `size`, the iterator returns no values.
    ///
    /// # Panics
    ///
    /// Panics if `size` is 0.
    ///
    /// # Examples
    ///
    /// ```
    /// let slice = ['r', 'u', 's', 't'];
    /// let mut iter = slice.windows(2);
    /// assert_eq!(iter.next().unwrap(), &['r', 'u']);
    /// assert_eq!(iter.next().unwrap(), &['u', 's']);
    /// assert_eq!(iter.next().unwrap(), &['s', 't']);
    /// assert!(iter.next().is_none());
    /// ```
    ///
    /// If the slice is shorter than `size`:
    ///
    /// ```
    /// let slice = ['f', 'o', 'o'];
    /// let mut iter = slice.windows(4);
    /// assert!(iter.next().is_none());
    /// ```
    #[stable(feature = "rust1", since = "1.0.0")]
    #[inline]
    pub fn windows(&self, size: usize) -> Windows<'_, T> {
        assert!(size != 0);
        Windows { v: self, size }
    }

    Examples
    let slice = ['r', 'u', 's', 't'];
    let mut iter = slice.windows(2);
    assert_eq!(iter.next().unwrap(), &['r', 'u']);
    assert_eq!(iter.next().unwrap(), &['u', 's']);
    assert_eq!(iter.next().unwrap(), &['s', 't']);
    assert!(iter.next().is_none());

*/

fn main_0() {
    let slice = ['r', 'u', 's', 't'];
    let mut iter = slice.windows(2);
    assert_eq!(iter.next().unwrap(), &['r', 'u']);
    assert_eq!(iter.next().unwrap(), &['u', 's']);
    assert_eq!(iter.next().unwrap(), &['s', 't']);
    assert!(iter.next().is_none());
    // If the slice is shorter than size:the iterator returns no values.
    let slice = ['f', 'o', 'o'];
    let mut iter = slice.windows(4);
    assert!(iter.next().is_none());
}

/*
// chunk[tʃʌŋk]n. 数据块(区块、语块、大块)
23. pub fn chunks(&self, chunk_size: usize) -> Chunks<T>
    [src][−]
    Returns an iterator over chunk_size elements of the slice at a time, starting at the beginning
    of the slice.

    The chunks are slices and do not overlap. If chunk_size does not divide the length of the slice,
    then the last chunk will not have length chunk_size.

    See chunks_exact for a variant of this iterator that returns chunks of always exactly chunk_size
    elements, and rchunks for the same iterator but starting at the end of the slice.

    Panics
    Panics if chunk_size is 0.

    /// Returns an iterator over `chunk_size` elements of the slice at a time, starting at the
    /// beginning of the slice.
    ///
    /// The chunks are slices and do not overlap. If `chunk_size` does not divide the length of the
    /// slice, then the last chunk will not have length `chunk_size`.
    ///
    /// See [`chunks_exact`] for a variant of this iterator that returns chunks of always exactly
    /// `chunk_size` elements, and [`rchunks`] for the same iterator but starting at the end of the
    /// slice.
    ///
    /// # Panics
    ///
    /// Panics if `chunk_size` is 0.
    ///
    /// # Examples
    ///
    /// ```
    /// let slice = ['l', 'o', 'r', 'e', 'm'];
    /// let mut iter = slice.chunks(2);
    /// assert_eq!(iter.next().unwrap(), &['l', 'o']);
    /// assert_eq!(iter.next().unwrap(), &['r', 'e']);
    /// assert_eq!(iter.next().unwrap(), &['m']);
    /// assert!(iter.next().is_none());
    /// ```
    ///
    /// [`chunks_exact`]: #method.chunks_exact
    /// [`rchunks`]: #method.rchunks
    #[stable(feature = "rust1", since = "1.0.0")]
    #[inline]
    pub fn chunks(&self, chunk_size: usize) -> Chunks<'_, T> {
        assert!(chunk_size != 0);
        Chunks { v: self, chunk_size }
    }

    Examples
    let slice = ['l', 'o', 'r', 'e', 'm'];
    let mut iter = slice.chunks(2);
    assert_eq!(iter.next().unwrap(), &['l', 'o']);
    assert_eq!(iter.next().unwrap(), &['r', 'e']);
    assert_eq!(iter.next().unwrap(), &['m']);
    assert!(iter.next().is_none());

24. pub fn chunks_mut(&mut self, chunk_size: usize) -> ChunksMut<T>
    [src][−]
    Returns an iterator over chunk_size elements of the slice at a time, starting at the beginning
    of the slice.

    // 块切片不重叠
    The chunks are mutable slices, and do not overlap. If chunk_size does not divide the length of
    the slice, then the last chunk will not have length chunk_size.

    See chunks_exact_mut for a variant of this iterator that returns chunks of always exactly
    chunk_size elements, and rchunks_mut for the same iterator but starting at the end of the slice.

    Panics
    Panics if chunk_size is 0.

    /// Returns an iterator over `chunk_size` elements of the slice at a time, starting at the
    /// beginning of the slice.
    ///
    /// The chunks are mutable slices, and do not overlap. If `chunk_size` does not divide the
    /// length of the slice, then the last chunk will not have length `chunk_size`.
    ///
    /// See [`chunks_exact_mut`] for a variant of this iterator that returns chunks of always
    /// exactly `chunk_size` elements, and [`rchunks_mut`] for the same iterator but starting at
    /// the end of the slice.
    ///
    /// # Panics
    ///
    /// Panics if `chunk_size` is 0.
    ///
    /// # Examples
    ///
    /// ```
    /// let v = &mut [0, 0, 0, 0, 0];
    /// let mut count = 1;
    ///
    /// for chunk in v.chunks_mut(2) {
    ///     for elem in chunk.iter_mut() {
    ///         *elem += count;
    ///     }
    ///     count += 1;
    /// }
    /// assert_eq!(v, &[1, 1, 2, 2, 3]);
    /// ```
    ///
    /// [`chunks_exact_mut`]: #method.chunks_exact_mut
    /// [`rchunks_mut`]: #method.rchunks_mut
    #[stable(feature = "rust1", since = "1.0.0")]
    #[inline]
    pub fn chunks_mut(&mut self, chunk_size: usize) -> ChunksMut<'_, T> {
        assert!(chunk_size != 0);
        ChunksMut { v: self, chunk_size }
    }

    Examples
    let v = &mut [0, 0, 0, 0, 0];
    let mut count = 1;
    for chunk in v.chunks_mut(2) {
        for elem in chunk.iter_mut() {
            *elem += count;
        }
        count += 1;
    }
    assert_eq!(v, &[1, 1, 2, 2, 3]);

 */
fn main_1() {
    // chunks
    let slice = ['l', 'o', 'r', 'e', 'm'];
    let mut iter = slice.chunks(2);
    assert_eq!(iter.next().unwrap(), &['l', 'o']);
    assert_eq!(iter.next().unwrap(), &['r', 'e']);
    assert_eq!(iter.next().unwrap(), &['m']);
    assert!(iter.next().is_none());
    // chunks_mut
    let v = &mut [0, 0, 0, 0, 0];
    let mut count = 1;
    for chunk in v.chunks_mut(2) {
        for elem in chunk.iter_mut() {
            *elem += count;
        }
        count += 1;
    }
    assert_eq!(v, &[1, 1, 2, 2, 3]);
}

/*
25. pub fn chunks_exact(&self, chunk_size: usize) -> ChunksExact<T>
    1.31.0
    [src]
    [−]
    Returns an iterator over chunk_size elements of the slice at a time, starting at the beginning
    of the slice.

    The chunks are slices and do not overlap. If chunk_size does not divide the length of the slice,
    then the last up to chunk_size-1 elements will be omitted and can be retrieved from the
    remainder function of the iterator.

    Due to each chunk having exactly chunk_size elements, the compiler can often optimize the
    resulting code better than in the case of chunks.

    See chunks for a variant of this iterator that also returns the remainder as a smaller chunk,
    and rchunks_exact for the same iterator but starting at the end of the slice.

    Panics
    Panics if chunk_size is 0.

    /// Returns an iterator over `chunk_size` elements of the slice at a time, starting at the
    /// beginning of the slice.
    ///
    /// The chunks are slices and do not overlap. If `chunk_size` does not divide the length of the
    /// slice, then the last up to `chunk_size-1` elements will be omitted and can be retrieved
    /// from the `remainder` function of the iterator.
    ///
    /// Due to each chunk having exactly `chunk_size` elements, the compiler can often optimize the
    /// resulting code better than in the case of [`chunks`].
    ///
    /// See [`chunks`] for a variant of this iterator that also returns the remainder as a smaller
    /// chunk, and [`rchunks_exact`] for the same iterator but starting at the end of the slice.
    ///
    /// # Panics
    ///
    /// Panics if `chunk_size` is 0.
    ///
    /// # Examples
    ///
    /// ```
    /// let slice = ['l', 'o', 'r', 'e', 'm'];
    /// let mut iter = slice.chunks_exact(2);
    /// assert_eq!(iter.next().unwrap(), &['l', 'o']);
    /// assert_eq!(iter.next().unwrap(), &['r', 'e']);
    /// assert!(iter.next().is_none());
    /// assert_eq!(iter.remainder(), &['m']);
    /// ```
    ///
    /// [`chunks`]: #method.chunks
    /// [`rchunks_exact`]: #method.rchunks_exact
    #[stable(feature = "chunks_exact", since = "1.31.0")]
    #[inline]
    pub fn chunks_exact(&self, chunk_size: usize) -> ChunksExact<'_, T> {
        assert!(chunk_size != 0);
        let rem = self.len() % chunk_size;
        let len = self.len() - rem;
        let (fst, snd) = self.split_at(len);
        ChunksExact { v: fst, rem: snd, chunk_size }
    }

    Examples
    let slice = ['l', 'o', 'r', 'e', 'm'];
    let mut iter = slice.chunks_exact(2);
    assert_eq!(iter.next().unwrap(), &['l', 'o']);
    assert_eq!(iter.next().unwrap(), &['r', 'e']);
    assert!(iter.next().is_none());
    assert_eq!(iter.remainder(), &['m']);

26. pub fn chunks_exact_mut(&mut self, chunk_size: usize) -> ChunksExactMut<T>
    1.31.0
    [src]
    [−]
    Returns an iterator over chunk_size elements of the slice at a time, starting at the beginning
    of the slice.

    The chunks are mutable slices, and do not overlap. If chunk_size does not divide the length of
    the slice, then the last up to chunk_size-1 elements will be omitted and can be retrieved from
    the into_remainder function of the iterator.

    Due to each chunk having exactly chunk_size elements, the compiler can often optimize the
    resulting code better than in the case of chunks_mut.

    See chunks_mut for a variant of this iterator that also returns the remainder as a smaller
    chunk, and rchunks_exact_mut for the same iterator but starting at the end of the slice.

    /// Returns an iterator over `chunk_size` elements of the slice at a time, starting at the
    /// beginning of the slice.
    ///
    /// The chunks are mutable slices, and do not overlap. If `chunk_size` does not divide the
    /// length of the slice, then the last up to `chunk_size-1` elements will be omitted and can be
    /// retrieved from the `into_remainder` function of the iterator.
    ///
    /// Due to each chunk having exactly `chunk_size` elements, the compiler can often optimize the
    /// resulting code better than in the case of [`chunks_mut`].
    ///
    /// See [`chunks_mut`] for a variant of this iterator that also returns the remainder as a
    /// smaller chunk, and [`rchunks_exact_mut`] for the same iterator but starting at the end of
    /// the slice.
    ///
    /// # Panics
    ///
    /// Panics if `chunk_size` is 0.
    ///
    /// # Examples
    ///
    /// ```
    /// let v = &mut [0, 0, 0, 0, 0];
    /// let mut count = 1;
    ///
    /// for chunk in v.chunks_exact_mut(2) {
    ///     for elem in chunk.iter_mut() {
    ///         *elem += count;
    ///     }
    ///     count += 1;
    /// }
    /// assert_eq!(v, &[1, 1, 2, 2, 0]);
    /// ```
    ///
    /// [`chunks_mut`]: #method.chunks_mut
    /// [`rchunks_exact_mut`]: #method.rchunks_exact_mut
    #[stable(feature = "chunks_exact", since = "1.31.0")]
    #[inline]
    pub fn chunks_exact_mut(&mut self, chunk_size: usize) -> ChunksExactMut<'_, T> {
        assert!(chunk_size != 0);
        let rem = self.len() % chunk_size;
        let len = self.len() - rem;
        let (fst, snd) = self.split_at_mut(len);
        ChunksExactMut { v: fst, rem: snd, chunk_size }
    }

    Examples
    let v = &mut [0, 0, 0, 0, 0];
    let mut count = 1;
    for chunk in v.chunks_exact_mut(2) {
        for elem in chunk.iter_mut() {
            *elem += count;
        }
        count += 1;
    }
    assert_eq!(v, &[1, 1, 2, 2, 0]);

 */

fn main_2() {
    // chunks_exact
    let slice = ['l', 'o', 'r', 'e', 'm'];
    let mut iter = slice.chunks_exact(2);
    assert_eq!(iter.next().unwrap(), &['l', 'o']);
    assert_eq!(iter.next().unwrap(), &['r', 'e']);
    assert!(iter.next().is_none());
    assert_eq!(iter.remainder(), &['m']);
    // chunks_exact_mut
    let v = &mut [0, 0, 0, 0, 0];
    let mut count = 1;
    for chunk in v.chunks_exact_mut(2) {
        for elem in chunk.iter_mut() {
            *elem += count;
        }
        count += 1;
    }
    assert_eq!(v, &[1, 1, 2, 2, 0]);
}

/*
27. pub fn rchunks(&self, chunk_size: usize) -> RChunks<T>
    1.31.0
    [src]
    [−]
    Returns an iterator over chunk_size elements of the slice at a time, starting at the end of the
    slice.

    The chunks are slices and do not overlap. If chunk_size does not divide the length of the slice,
    then the last chunk will not have length chunk_size.

    See rchunks_exact for a variant of this iterator that returns chunks of always exactly
    chunk_size elements, and chunks for the same iterator but starting at the beginning of the slice.

    Panics
    Panics if chunk_size is 0.

    /// Returns an iterator over `chunk_size` elements of the slice at a time, starting at the end
    /// of the slice.
    ///
    /// The chunks are slices and do not overlap. If `chunk_size` does not divide the length of the
    /// slice, then the last chunk will not have length `chunk_size`.
    ///
    /// See [`rchunks_exact`] for a variant of this iterator that returns chunks of always exactly
    /// `chunk_size` elements, and [`chunks`] for the same iterator but starting at the beginning
    /// of the slice.
    ///
    /// # Panics
    ///
    /// Panics if `chunk_size` is 0.
    ///
    /// # Examples
    ///
    /// ```
    /// let slice = ['l', 'o', 'r', 'e', 'm'];
    /// let mut iter = slice.rchunks(2);
    /// assert_eq!(iter.next().unwrap(), &['e', 'm']);
    /// assert_eq!(iter.next().unwrap(), &['o', 'r']);
    /// assert_eq!(iter.next().unwrap(), &['l']);
    /// assert!(iter.next().is_none());
    /// ```
    ///
    /// [`rchunks_exact`]: #method.rchunks_exact
    /// [`chunks`]: #method.chunks
    #[stable(feature = "rchunks", since = "1.31.0")]
    #[inline]
    pub fn rchunks(&self, chunk_size: usize) -> RChunks<'_, T> {
        assert!(chunk_size != 0);
        RChunks { v: self, chunk_size }
    }

    Examples
    let slice = ['l', 'o', 'r', 'e', 'm'];
    let mut iter = slice.rchunks(2);
    assert_eq!(iter.next().unwrap(), &['e', 'm']);
    assert_eq!(iter.next().unwrap(), &['o', 'r']);
    assert_eq!(iter.next().unwrap(), &['l']);
    assert!(iter.next().is_none());

28. pub fn rchunks_mut(&mut self, chunk_size: usize) -> RChunksMut<T>
    1.31.0
    [src]
    [−]
    Returns an iterator over chunk_size elements of the slice at a time, starting at the end of the
    slice.

    The chunks are mutable slices, and do not overlap. If chunk_size does not divide the length of
    the slice, then the last chunk will not have length chunk_size.

    See rchunks_exact_mut for a variant of this iterator that returns chunks of always exactly
    chunk_size elements, and chunks_mut for the same iterator but starting at the beginning of the
    slice.

    Panics
    Panics if chunk_size is 0.

    /// Returns an iterator over `chunk_size` elements of the slice at a time, starting at the end
    /// of the slice.
    ///
    /// The chunks are mutable slices, and do not overlap. If `chunk_size` does not divide the
    /// length of the slice, then the last chunk will not have length `chunk_size`.
    ///
    /// See [`rchunks_exact_mut`] for a variant of this iterator that returns chunks of always
    /// exactly `chunk_size` elements, and [`chunks_mut`] for the same iterator but starting at the
    /// beginning of the slice.
    ///
    /// # Panics
    ///
    /// Panics if `chunk_size` is 0.
    ///
    /// # Examples
    ///
    /// ```
    /// let v = &mut [0, 0, 0, 0, 0];
    /// let mut count = 1;
    ///
    /// for chunk in v.rchunks_mut(2) {
    ///     for elem in chunk.iter_mut() {
    ///         *elem += count;
    ///     }
    ///     count += 1;
    /// }
    /// assert_eq!(v, &[3, 2, 2, 1, 1]);
    /// ```
    ///
    /// [`rchunks_exact_mut`]: #method.rchunks_exact_mut
    /// [`chunks_mut`]: #method.chunks_mut
    #[stable(feature = "rchunks", since = "1.31.0")]
    #[inline]
    pub fn rchunks_mut(&mut self, chunk_size: usize) -> RChunksMut<'_, T> {
        assert!(chunk_size != 0);
        RChunksMut { v: self, chunk_size }
    }

    Examples
    let v = &mut [0, 0, 0, 0, 0];
    let mut count = 1;
    for chunk in v.rchunks_mut(2) {
        for elem in chunk.iter_mut() {
            *elem += count;
        }
        count += 1;
    }
    assert_eq!(v, &[3, 2, 2, 1, 1]);

 */
fn main_3() {
    // rchunks
    let slice = ['l', 'o', 'r', 'e', 'm'];
    let mut iter = slice.rchunks(2);
    assert_eq!(iter.next().unwrap(), &['e', 'm']);
    assert_eq!(iter.next().unwrap(), &['o', 'r']);
    assert_eq!(iter.next().unwrap(), &['l']);
    assert!(iter.next().is_none());
    // rchunks_mut
    let v = &mut [0, 0, 0, 0, 0];
    let mut count = 1;
    // 因已反转故先遍历的是原切片尾部向前 chunk 的切片
    for chunk in v.rchunks_mut(2) {
        for elem in chunk.iter_mut() {
            *elem += count;
        }
        count += 1;
    }
    assert_eq!(v, &[3, 2, 2, 1, 1]);
}

/*
29. pub fn rchunks_exact(&self, chunk_size: usize) -> RChunksExact<T>
    1.31.0
    [src]
    [−]
    Returns an iterator over chunk_size elements of the slice at a time, starting at the end of the
    slice.

    The chunks are slices and do not overlap. If chunk_size does not divide the length of the slice,
    then the last up to chunk_size-1 elements will be omitted and can be retrieved from the
    remainder function of the iterator.

    Due to each chunk having exactly chunk_size elements, the compiler can often optimize the
    resulting code better than in the case of chunks.

    See rchunks for a variant of this iterator that also returns the remainder as a smaller chunk,
    and chunks_exact for the same iterator but starting at the beginning of the slice.

    Panics
    Panics if chunk_size is 0.

    /// Returns an iterator over `chunk_size` elements of the slice at a time, starting at the
    /// end of the slice.
    ///
    /// The chunks are slices and do not overlap. If `chunk_size` does not divide the length of the
    /// slice, then the last up to `chunk_size-1` elements will be omitted and can be retrieved
    /// from the `remainder` function of the iterator.
    ///
    /// Due to each chunk having exactly `chunk_size` elements, the compiler can often optimize the
    /// resulting code better than in the case of [`chunks`].
    ///
    /// See [`rchunks`] for a variant of this iterator that also returns the remainder as a smaller
    /// chunk, and [`chunks_exact`] for the same iterator but starting at the beginning of the
    /// slice.
    ///
    /// # Panics
    ///
    /// Panics if `chunk_size` is 0.
    ///
    /// # Examples
    ///
    /// ```
    /// let slice = ['l', 'o', 'r', 'e', 'm'];
    /// let mut iter = slice.rchunks_exact(2);
    /// assert_eq!(iter.next().unwrap(), &['e', 'm']);
    /// assert_eq!(iter.next().unwrap(), &['o', 'r']);
    /// assert!(iter.next().is_none());
    /// assert_eq!(iter.remainder(), &['l']);
    /// ```
    ///
    /// [`chunks`]: #method.chunks
    /// [`rchunks`]: #method.rchunks
    /// [`chunks_exact`]: #method.chunks_exact
    #[stable(feature = "rchunks", since = "1.31.0")]
    #[inline]
    pub fn rchunks_exact(&self, chunk_size: usize) -> RChunksExact<'_, T> {
        assert!(chunk_size != 0);
        let rem = self.len() % chunk_size;
        let (fst, snd) = self.split_at(rem);
        RChunksExact { v: snd, rem: fst, chunk_size }
    }

    Examples
    let slice = ['l', 'o', 'r', 'e', 'm'];
    let mut iter = slice.rchunks_exact(2);
    assert_eq!(iter.next().unwrap(), &['e', 'm']);
    assert_eq!(iter.next().unwrap(), &['o', 'r']);
    assert!(iter.next().is_none());
    assert_eq!(iter.remainder(), &['l']);

30. pub fn rchunks_exact_mut(&mut self, chunk_size: usize) -> RChunksExactMut<T>
    1.31.0
    [src]
    [−]
    Returns an iterator over chunk_size elements of the slice at a time, starting at the end of the
    slice.

    The chunks are mutable slices, and do not overlap. If chunk_size does not divide the length of
    the slice, then the last up to chunk_size-1 elements will be omitted and can be retrieved from
    the into_remainder function of the iterator.

    Due to each chunk having exactly chunk_size elements, the compiler can often optimize the
    resulting code better than in the case of chunks_mut.

    See rchunks_mut for a variant of this iterator that also returns the remainder as a smaller
    chunk, and chunks_exact_mut for the same iterator but starting at the beginning of the slice.

    Panics
    Panics if chunk_size is 0.

    /// Returns an iterator over `chunk_size` elements of the slice at a time, starting at the end
    /// of the slice.
    ///
    /// The chunks are mutable slices, and do not overlap. If `chunk_size` does not divide the
    /// length of the slice, then the last up to `chunk_size-1` elements will be omitted and can be
    /// retrieved from the `into_remainder` function of the iterator.
    ///
    /// Due to each chunk having exactly `chunk_size` elements, the compiler can often optimize the
    /// resulting code better than in the case of [`chunks_mut`].
    ///
    /// See [`rchunks_mut`] for a variant of this iterator that also returns the remainder as a
    /// smaller chunk, and [`chunks_exact_mut`] for the same iterator but starting at the beginning
    /// of the slice.
    ///
    /// # Panics
    ///
    /// Panics if `chunk_size` is 0.
    ///
    /// # Examples
    ///
    /// ```
    /// let v = &mut [0, 0, 0, 0, 0];
    /// let mut count = 1;
    ///
    /// for chunk in v.rchunks_exact_mut(2) {
    ///     for elem in chunk.iter_mut() {
    ///         *elem += count;
    ///     }
    ///     count += 1;
    /// }
    /// assert_eq!(v, &[0, 2, 2, 1, 1]);
    /// ```
    ///
    /// [`chunks_mut`]: #method.chunks_mut
    /// [`rchunks_mut`]: #method.rchunks_mut
    /// [`chunks_exact_mut`]: #method.chunks_exact_mut
    #[stable(feature = "rchunks", since = "1.31.0")]
    #[inline]
    pub fn rchunks_exact_mut(&mut self, chunk_size: usize) -> RChunksExactMut<'_, T> {
        assert!(chunk_size != 0);
        let rem = self.len() % chunk_size;
        let (fst, snd) = self.split_at_mut(rem);
        RChunksExactMut { v: snd, rem: fst, chunk_size }
    }

    Examples
    let v = &mut [0, 0, 0, 0, 0];
    let mut count = 1;
    for chunk in v.rchunks_exact_mut(2) {
        for elem in chunk.iter_mut() {
            *elem += count;
        }
        count += 1;
    }
    assert_eq!(v, &[0, 2, 2, 1, 1]);

 */
fn main() {
    // rchunks_exact
    let slice = ['l', 'o', 'r', 'e', 'm'];
    let mut iter = slice.rchunks_exact(2);
    assert_eq!(iter.next().unwrap(), &['e', 'm']);
    assert_eq!(iter.next().unwrap(), &['o', 'r']);
    assert!(iter.next().is_none());
    assert_eq!(iter.remainder(), &['l']);
    // rchunks_exact_mut
    let v = &mut [0, 0, 0, 0, 0];
    let mut count = 1;
    for chunk in v.rchunks_exact_mut(2) {
        for elem in chunk.iter_mut() {
            *elem += count;
        }
        count += 1;
    }
    assert_eq!(v, &[0, 2, 2, 1, 1]);

}