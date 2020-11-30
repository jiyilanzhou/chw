
// strip[strɪp]v. 去除,剥去     // assume[əˈsjuːm]v. 假设
#![feature(slice_strip)]
/*
Primitive Type slice ：
    文档:" https://doc.rust-lang.org/std/primitive.slice.html "

43. pub fn contains(&self, x: &T) -> bool
    where
        T: PartialEq<T>,
    [src][−]
    Returns true if the slice contains an element with the given value.

    /// Returns `true` if the slice contains an element with the given value.
    ///
    /// # Examples
    ///
    /// ```
    /// let v = [10, 40, 30];
    /// assert!(v.contains(&30));
    /// assert!(!v.contains(&50));
    /// ```
    ///
    /// If you do not have an `&T`, but just an `&U` such that `T: Borrow<U>`
    /// (e.g. `String: Borrow<str>`), you can use `iter().any`:
    ///
    /// ```
    /// let v = [String::from("hello"), String::from("world")]; // slice of `String`
    /// assert!(v.iter().any(|e| e == "hello")); // search with `&str`
    /// assert!(!v.iter().any(|e| e == "hi"));
    /// ```
    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn contains(&self, x: &T) -> bool
    where
        T: PartialEq,
    {
        x.slice_contains(self)
    }

    Examples
    let v = [10, 40, 30];
    assert!(v.contains(&30));
    assert!(!v.contains(&50));

    // 若没有 &T 但存在 &U 使得 T: Borrow<U> 如" String: Borrow<str> "则可用 iter().any
    If you do not have an &T, but just an &U such that T: Borrow<U> (e.g. String: Borrow<str>), you
    can use iter().any:
    Examples
    let v = [String::from("hello"), String::from("world")]; // slice of `String`
    assert!(v.iter().any(|e| e == "hello")); // search with `&str`
    assert!(!v.iter().any(|e| e == "hi"));

// needle[ˈniːdl]n. (指)针
44. pub fn starts_with(&self, needle: &[T]) -> bool
    where
        T: PartialEq<T>,
    [src][−]
    Returns true if needle is a prefix of the slice.

    /// Returns `true` if `needle` is a prefix of the slice.
    ///
    /// # Examples
    ///
    /// ```
    /// let v = [10, 40, 30];
    /// assert!(v.starts_with(&[10]));
    /// assert!(v.starts_with(&[10, 40]));
    /// assert!(!v.starts_with(&[50]));
    /// assert!(!v.starts_with(&[10, 50]));
    /// ```
    ///
    /// Always returns `true` if `needle` is an empty slice:
    ///
    /// ```
    /// let v = &[10, 40, 30];
    /// assert!(v.starts_with(&[]));
    /// let v: &[u8] = &[];
    /// assert!(v.starts_with(&[]));
    /// ```
    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn starts_with(&self, needle: &[T]) -> bool
    where
        T: PartialEq,
    {
        let n = needle.len();
        self.len() >= n && needle == &self[..n]
    }

    Examples
    let v = [10, 40, 30];
    assert!(v.starts_with(&[10]));
    assert!(v.starts_with(&[10, 40]));
    assert!(!v.starts_with(&[50]));
    assert!(!v.starts_with(&[10, 50]));

    // 若以空切片" &[] "为前缀判断则始终返回 true
    Always returns true if needle is an empty slice:
    Examples
    let v = &[10, 40, 30];
    assert!(v.starts_with(&[]));
    let v: &[u8] = &[];
    assert!(v.starts_with(&[]));

45. pub fn ends_with(&self, needle: &[T]) -> bool
    where
        T: PartialEq<T>,
    [src][−]
    Returns true if needle is a suffix of the slice.

    /// Returns `true` if `needle` is a suffix of the slice.
    ///
    /// # Examples
    ///
    /// ```
    /// let v = [10, 40, 30];
    /// assert!(v.ends_with(&[30]));
    /// assert!(v.ends_with(&[40, 30]));
    /// assert!(!v.ends_with(&[50]));
    /// assert!(!v.ends_with(&[50, 30]));
    /// ```
    ///
    /// Always returns `true` if `needle` is an empty slice:
    ///
    /// ```
    /// let v = &[10, 40, 30];
    /// assert!(v.ends_with(&[]));
    /// let v: &[u8] = &[];
    /// assert!(v.ends_with(&[]));
    /// ```
    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn ends_with(&self, needle: &[T]) -> bool
    where
        T: PartialEq,
    {
        let (m, n) = (self.len(), needle.len());
        m >= n && needle == &self[m - n..]
    }

    Examples
    let v = [10, 40, 30];
    assert!(v.ends_with(&[30]));
    assert!(v.ends_with(&[40, 30]));
    assert!(!v.ends_with(&[50]));
    assert!(!v.ends_with(&[50, 30]));

    // 若以空切片" &[] "为后缀判断则始终返回 true
    Always returns true if needle is an empty slice:
    Examples
    let v = &[10, 40, 30];
    assert!(v.ends_with(&[]));
    let v: &[u8] = &[];
    assert!(v.ends_with(&[]));

*/
fn main_0() {
    // contains
    let v = [10, 40, 30];
    assert!(v.contains(&30));
    assert!(!v.contains(&50));
    // any
    let v = [String::from("hello"), String::from("world")]; // slice of `String`
    assert!(v.iter().any(|e| e == "hello")); // search with `&str`
    assert!(!v.iter().any(|e| e == "hi"));

    // start_with
    let v = [10, 40, 30];
    assert!(v.starts_with(&[10]));
    assert!(v.starts_with(&[10, 40]));
    assert!(!v.starts_with(&[50]));
    assert!(!v.starts_with(&[10, 50]));
    // 若以空切片" &[] "为前缀判断则始终返回 true
    let v = &[10, 40, 30];
    assert!(v.starts_with(&[]));
    let v: &[u8] = &[];
    assert!(v.starts_with(&[]));

    // end_with
    let v = [10, 40, 30];
    assert!(v.ends_with(&[30]));
    assert!(v.ends_with(&[40, 30]));
    assert!(!v.ends_with(&[50]));
    assert!(!v.ends_with(&[50, 30]));
    // 若以空切片" &[] "为后缀判断则始终返回 true
    let v = &[10, 40, 30];
    assert!(v.ends_with(&[]));
    let v: &[u8] = &[];
    assert!(v.ends_with(&[]));
}

// strip[strɪp]v. 去除,剥去
/*
46. [−]
    #[must_use = "returns the subslice without modifying the original"]
    pub fn strip_prefix(&self, prefix: &[T]) -> Option<&[T]>
    where
        T: PartialEq<T>,
    [src][−]
    🔬 This is a nightly-only experimental API. (slice_strip #73413)
    // 返回去除前缀后的切片引用
    Returns a subslice with the prefix removed.

    This method returns None if slice does not start with prefix. Also it returns the original slice
    if prefix is an empty slice.

    /// Returns a subslice with the prefix removed.
    ///
    /// This method returns [`None`] if slice does not start with `prefix`.
    /// Also it returns the original slice if `prefix` is an empty slice.
    ///
    /// # Examples
    ///
    /// ```
    /// #![feature(slice_strip)]
    /// let v = &[10, 40, 30];
    /// assert_eq!(v.strip_prefix(&[10]), Some(&[40, 30][..]));
    /// assert_eq!(v.strip_prefix(&[10, 40]), Some(&[30][..]));
    /// assert_eq!(v.strip_prefix(&[50]), None);
    /// assert_eq!(v.strip_prefix(&[10, 50]), None);
    /// ```
    #[must_use = "returns the subslice without modifying the original"]
    #[unstable(feature = "slice_strip", issue = "73413")]
    pub fn strip_prefix(&self, prefix: &[T]) -> Option<&[T]>
    where
        T: PartialEq,
    {
        let n = prefix.len();
        if n <= self.len() {
            let (head, tail) = self.split_at(n);
            if head == prefix {
                return Some(tail);
            }
        }
        None
    }

    Examples
    #![feature(slice_strip)]
    let v = &[10, 40, 30];
    assert_eq!(v.strip_prefix(&[10]), Some(&[40, 30][..]));
    assert_eq!(v.strip_prefix(&[10, 40]), Some(&[30][..]));
    assert_eq!(v.strip_prefix(&[50]), None);
    assert_eq!(v.strip_prefix(&[10, 50]), None);

47. [−]
    #[must_use = "returns the subslice without modifying the original"]
    pub fn strip_suffix(&self, suffix: &[T]) -> Option<&[T]>
    where
        T: PartialEq<T>,
    [src][−]
    🔬 This is a nightly-only experimental API. (slice_strip #73413)
    // 返回去除后缀后的切片引用
    Returns a subslice with the suffix removed.

    This method returns None if slice does not end with suffix. Also it returns the original slice
    if suffix is an empty slice

    /// Returns a subslice with the suffix removed.
    ///
    /// This method returns [`None`] if slice does not end with `suffix`.
    /// Also it returns the original slice if `suffix` is an empty slice
    ///
    /// # Examples
    ///
    /// ```
    /// #![feature(slice_strip)]
    /// let v = &[10, 40, 30];
    /// assert_eq!(v.strip_suffix(&[30]), Some(&[10, 40][..]));
    /// assert_eq!(v.strip_suffix(&[40, 30]), Some(&[10][..]));
    /// assert_eq!(v.strip_suffix(&[50]), None);
    /// assert_eq!(v.strip_suffix(&[50, 30]), None);
    /// ```
    #[must_use = "returns the subslice without modifying the original"]
    #[unstable(feature = "slice_strip", issue = "73413")]
    pub fn strip_suffix(&self, suffix: &[T]) -> Option<&[T]>
    where
        T: PartialEq,
    {
        let (len, n) = (self.len(), suffix.len());
        if n <= len {
            let (head, tail) = self.split_at(len - n);
            if tail == suffix {
                return Some(head);
            }
        }
        None
    }

    Examples
    #![feature(slice_strip)]
    let v = &[10, 40, 30];
    assert_eq!(v.strip_suffix(&[30]), Some(&[10, 40][..]));
    assert_eq!(v.strip_suffix(&[40, 30]), Some(&[10][..]));
    assert_eq!(v.strip_suffix(&[50]), None);
    assert_eq!(v.strip_suffix(&[50, 30]), None);

 */
// " strip_prefix/suffix "使用须配置" #![feature(slice_strip)] "
fn main_1() {
    // strip_prefix : 返回去除前缀后的切片引用
    let v = &[10, 40, 30];
    assert_eq!(v.strip_prefix(&[10]), Some(&([40, 30][..])));
    assert_eq!(v.strip_prefix(&[10, 40]), Some(&[30][..]));
    assert_eq!(v.strip_prefix(&[50]), None);
    assert_eq!(v.strip_prefix(&[10, 50]), None);
    // strip_suffix :
    assert_eq!(v.strip_suffix(&[30]), Some(&[10, 40][..]));
    assert_eq!(v.strip_suffix(&[40, 30]), Some(&[10][..]));
    assert_eq!(v.strip_suffix(&[50]), None);
    assert_eq!(v.strip_suffix(&[50, 30]), None);
}

/*
48. pub fn binary_search(&self, x: &T) -> Result<usize, usize>
    where
        T: Ord,
    [src][−]
    // 二分查找已排好序的切片
    Binary searches this sorted slice for a given element.

    // 若找到该值则 Result::Ok 返回该值(其中包含匹配元素的索引)
    If the value is found then Result::Ok is returned, containing the index of the matching element.
    // 若有多个匹配项则可返回任何一个匹配项([自]但至少包含最后一个匹配选项)
    If there are multiple matches, then any one of the matches could be returned. If the value is
    // 若找不到该值则 Result::Err 返回该值，其中包含在保持排序顺序的同时可以在其中插入匹配元素的索引
    not found then Result::Err is returned, containing the index where a matching element could be
    inserted while maintaining sorted order.

    /// Binary searches this sorted slice for a given element.
    ///
    /// If the value is found then [`Result::Ok`] is returned, containing the
    /// index of the matching element. If there are multiple matches, then any
    /// one of the matches could be returned. If the value is not found then
    /// [`Result::Err`] is returned, containing the index where a matching
    /// element could be inserted while maintaining sorted order.
    ///
    /// # Examples
    ///
    /// Looks up a series of four elements. The first is found, with a
    /// uniquely determined position; the second and third are not
    /// found; the fourth could match any position in `[1, 4]`.
    ///
    /// ```
    /// let s = [0, 1, 1, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55];
    ///
    /// assert_eq!(s.binary_search(&13),  Ok(9));
    /// assert_eq!(s.binary_search(&4),   Err(7));
    /// assert_eq!(s.binary_search(&100), Err(13));
    /// let r = s.binary_search(&1);
    /// assert!(match r { Ok(1..=4) => true, _ => false, });
    /// ```
    ///
    /// If you want to insert an item to a sorted vector, while maintaining
    /// sort order:
    ///
    /// ```
    /// let mut s = vec![0, 1, 1, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55];
    /// let num = 42;
    /// let idx = s.binary_search(&num).unwrap_or_else(|x| x);
    /// s.insert(idx, num);
    /// assert_eq!(s, [0, 1, 1, 1, 1, 2, 3, 5, 8, 13, 21, 34, 42, 55]);
    /// ```
    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn binary_search(&self, x: &T) -> Result<usize, usize>
    where
        T: Ord,
    {
        self.binary_search_by(|p| p.cmp(x))
    }

    Examples
    // [自]查找一系列元素(包含单个元素)只需包含最后一个匹配元素位置即可
    // Looks up a series of four elements. The first is found, with a uniquely determined position;
    // the second and third are not found; the fourth could match any position in [1, 4].
    let s = [0, 1, 1, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55];
    assert_eq!(s.binary_search(&13),  Ok(9));
    assert_eq!(s.binary_search(&4),   Err(7));
    assert_eq!(s.binary_search(&100), Err(13));
    let r = s.binary_search(&1);
    assert!(match r { Ok(1..=4) => true, _ => false, });

    If you want to insert an item to a sorted vector, while maintaining sort order:
    Examples
    let mut s = vec![0, 1, 1, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55];
    let num = 42;
    let idx = s.binary_search(&num).unwrap_or_else(|x| x);
    s.insert(idx, num);
    assert_eq!(s, [0, 1, 1, 1, 1, 2, 3, 5, 8, 13, 21, 34, 42, 55]);

49. pub fn binary_search_by<'a, F>(&'a self, f: F) -> Result<usize, usize>
    where
        F: FnMut(&'a T) -> Ordering,
    [src][−]
    // 通过比较器二分查找已排好序的切片
    Binary searches this sorted slice with a comparator function.

    The comparator function should implement an order consistent with the sort order of the
    underlying slice, returning an order code that indicates whether its argument is Less, Equal or
    Greater the desired target.

    // 若找到该值则 Result::Ok 返回该值(其中包含匹配元素的索引)
    If the value is found then Result::Ok is returned, containing the index of the matching element.
    // 若有多个匹配项则可返回任何一个匹配项([自]但至少包含最后一个匹配选项)
    If there are multiple matches, then any one of the matches could be returned. If the value is
    // 若找不到该值则 Result::Err 返回该值，其中包含在保持排序顺序的同时可以在其中插入匹配元素的索引
    not found then Result::Err is returned, containing the index where a matching element could be
    inserted while maintaining sorted order.

    /// Binary searches this sorted slice with a comparator function.
    ///
    /// The comparator function should implement an order consistent
    /// with the sort order of the underlying slice, returning an
    /// order code that indicates whether its argument is `Less`,
    /// `Equal` or `Greater` the desired target.
    ///
    /// If the value is found then [`Result::Ok`] is returned, containing the
    /// index of the matching element. If there are multiple matches, then any
    /// one of the matches could be returned. If the value is not found then
    /// [`Result::Err`] is returned, containing the index where a matching
    /// element could be inserted while maintaining sorted order.
    ///
    /// # Examples
    ///
    /// Looks up a series of four elements. The first is found, with a
    /// uniquely determined position; the second and third are not
    /// found; the fourth could match any position in `[1, 4]`.
    ///
    /// ```
    /// let s = [0, 1, 1, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55];
    ///
    /// let seek = 13;
    /// assert_eq!(s.binary_search_by(|probe| probe.cmp(&seek)), Ok(9));
    /// let seek = 4;
    /// assert_eq!(s.binary_search_by(|probe| probe.cmp(&seek)), Err(7));
    /// let seek = 100;
    /// assert_eq!(s.binary_search_by(|probe| probe.cmp(&seek)), Err(13));
    /// let seek = 1;
    /// let r = s.binary_search_by(|probe| probe.cmp(&seek));
    /// assert!(match r { Ok(1..=4) => true, _ => false, });
    /// ```
    #[stable(feature = "rust1", since = "1.0.0")]
    #[inline]
    pub fn binary_search_by<'a, F>(&'a self, mut f: F) -> Result<usize, usize>
    where
        F: FnMut(&'a T) -> Ordering,
    {
        let s = self;
        let mut size = s.len();
        if size == 0 {
            return Err(0);
        }
        let mut base = 0usize;
        while size > 1 {
            let half = size / 2;
            let mid = base + half;
            // mid is always in [0, size), that means mid is >= 0 and < size.
            // mid >= 0: by definition
            // mid < size: mid = size / 2 + size / 4 + size / 8 ...
            let cmp = f(unsafe { s.get_unchecked(mid) });
            base = if cmp == Greater { base } else { mid };
            size -= half;
        }
        // base is always in [0, size) because base <= mid.
        let cmp = f(unsafe { s.get_unchecked(base) });
        if cmp == Equal { Ok(base) } else { Err(base + (cmp == Less) as usize) }
    }

    Examples
    // [自]同理查找一系列元素(包含单个元素)只需包含最后一个匹配元素位置即可
    // Looks up a series of four elements. The first is found, with a uniquely determined position;
    // the second and third are not found; the fourth could match any position in [1, 4].
    let s = [0, 1, 1, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55];
    let seek = 13;
    assert_eq!(s.binary_search_by(|probe| probe.cmp(&seek)), Ok(9));
    let seek = 4;
    assert_eq!(s.binary_search_by(|probe| probe.cmp(&seek)), Err(7));
    let seek = 100;
    assert_eq!(s.binary_search_by(|probe| probe.cmp(&seek)), Err(13));
    let seek = 1;
    let r = s.binary_search_by(|probe| probe.cmp(&seek));
    assert!(match r { Ok(1..=4) => true, _ => false, });

// assume[əˈsjuːm]v. 假设
50. pub fn binary_search_by_key<'a, B, F>(
        &'a self,
        b: &B,
        f: F
    ) -> Result<usize, usize>
    where
        B: Ord,
        F: FnMut(&'a T) -> B,
    1.10.0
    [src]
    [−]
    // 通过密钥二分查找已排好序的切片
    Binary searches this sorted slice with a key extraction function.

    // 假设切片是通过 key 排序则可通过相同的 key 提取
    Assumes that the slice is sorted by the key, for instance with sort_by_key using the same key
    extraction function.

    // 若找到该值则 Result::Ok 返回该值(其中包含匹配元素的索引)
    If the value is found then Result::Ok is returned, containing the index of the matching element.
    // 若有多个匹配项则可返回任何一个匹配项([自]但至少包含最后一个匹配选项)
    If there are multiple matches, then any one of the matches could be returned. If the value is
    // 若找不到该值则 Result::Err 返回该值，其中包含在保持排序顺序的同时可以在其中插入匹配元素的索引
    not found then Result::Err is returned, containing the index where a matching element could be
    inserted while maintaining sorted order.

    /// Binary searches this sorted slice with a key extraction function.
    ///
    /// Assumes that the slice is sorted by the key, for instance with
    /// [`sort_by_key`] using the same key extraction function.
    ///
    /// If the value is found then [`Result::Ok`] is returned, containing the
    /// index of the matching element. If there are multiple matches, then any
    /// one of the matches could be returned. If the value is not found then
    /// [`Result::Err`] is returned, containing the index where a matching
    /// element could be inserted while maintaining sorted order.
    ///
    /// [`sort_by_key`]: #method.sort_by_key
    ///
    /// # Examples
    ///
    /// Looks up a series of four elements in a slice of pairs sorted by
    /// their second elements. The first is found, with a uniquely
    /// determined position; the second and third are not found; the
    /// fourth could match any position in `[1, 4]`.
    ///
    /// ```
    /// let s = [(0, 0), (2, 1), (4, 1), (5, 1), (3, 1),
    ///          (1, 2), (2, 3), (4, 5), (5, 8), (3, 13),
    ///          (1, 21), (2, 34), (4, 55)];
    ///
    /// assert_eq!(s.binary_search_by_key(&13, |&(a,b)| b),  Ok(9));
    /// assert_eq!(s.binary_search_by_key(&4, |&(a,b)| b),   Err(7));
    /// assert_eq!(s.binary_search_by_key(&100, |&(a,b)| b), Err(13));
    /// let r = s.binary_search_by_key(&1, |&(a,b)| b);
    /// assert!(match r { Ok(1..=4) => true, _ => false, });
    /// ```
    #[stable(feature = "slice_binary_search_by_key", since = "1.10.0")]
    #[inline]
    pub fn binary_search_by_key<'a, B, F>(&'a self, b: &B, mut f: F) -> Result<usize, usize>
    where
        F: FnMut(&'a T) -> B,
        B: Ord,
    {
        self.binary_search_by(|k| f(k).cmp(b))
    }

    Examples
    // [自]同理查找一系列元素(包含单个元素)只需包含最后一个匹配元素位置即可
    // Looks up a series of four elements in a slice of pairs sorted by their second elements. The
    // first is found, with a uniquely determined position; the second and third are not found; the
    // fourth could match any position in [1, 4].
    let s = [(0, 0), (2, 1), (4, 1), (5, 1), (3, 1),
             (1, 2), (2, 3), (4, 5), (5, 8), (3, 13),
             (1, 21), (2, 34), (4, 55)];
    assert_eq!(s.binary_search_by_key(&13, |&(a,b)| b),  Ok(9));
    assert_eq!(s.binary_search_by_key(&4, |&(a,b)| b),   Err(7));
    assert_eq!(s.binary_search_by_key(&100, |&(a,b)| b), Err(13));
    let r = s.binary_search_by_key(&1, |&(a,b)| b);
    assert!(match r { Ok(1..=4) => true, _ => false, });

*/
fn main() {
    // binary_search
    let s = [0, 1, 1, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55];
    assert_eq!(s.binary_search(&13),  Ok(9));
    assert_eq!(s.binary_search(&4),   Err(7));
    assert_eq!(s.binary_search(&100), Err(13));
    // [自]若有多个匹配项则返回任至少包含最后一个匹配选项
    //assert_eq!(s.binary_search(&1),  Ok(3)); // 运行错误
    assert_eq!(s.binary_search(&1),  Ok(4));
    println!("{:?}",s.binary_search(&1)); // Console:" Ok(4) "
    let r = s.binary_search(&1);
    // 包含最后一个匹配元素对应索引" 4 "(其本质还是" Ok(4) "的范围匹配)
    assert!(match r { Ok(1..=4) => true, _ => false, });
    assert!(match r { Ok(0..=8) => true, _ => false, });
    assert!(match r { Ok(4..=8) => true, _ => false, });
    // 未包含最后一个匹配元素对应索引" 4 "
    //assert!(match r { Ok(1..=3) => true, _ => false, });// 运行时错误
    let mut s = vec![0, 1, 1, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55];
    let num = 42;
    // 插入数据并保持排序顺序
    let idx = s.binary_search(&num).unwrap_or_else(|x| x);
    s.insert(idx, num);
    assert_eq!(s, [0, 1, 1, 1, 1, 2, 3, 5, 8, 13, 21, 34, 42, 55]);

    // binary_search_by
    let s = [0, 1, 1, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55];
    let seek = 13;
    assert_eq!(s.binary_search_by(|probe| probe.cmp(&seek)), Ok(9));
    let seek = 4;
    assert_eq!(s.binary_search_by(|probe| probe.cmp(&seek)), Err(7));
    let seek = 100;
    assert_eq!(s.binary_search_by(|probe| probe.cmp(&seek)), Err(13));
    let seek = 1;
    let r = s.binary_search_by(|probe| probe.cmp(&seek));
    println!("{:?}",s.binary_search_by(|probe| probe.cmp(&seek))); // Console:" Ok(4) "
    assert!(match r { Ok(1..=4) => true, _ => false, });

    // binary_search_by_key
    let s = [(0, 0), (2, 1), (4, 1), (5, 1), (3, 1),
        (1, 2), (2, 3), (4, 5), (5, 8), (3, 13),
        (1, 21), (2, 34), (4, 55)];
    assert_eq!(s.binary_search_by_key(&13, |&(a,b)| b),  Ok(9));
    assert_eq!(s.binary_search_by_key(&4, |&(a,b)| b),   Err(7));
    assert_eq!(s.binary_search_by_key(&100, |&(a,b)| b), Err(13));
    let r = s.binary_search_by_key(&1, |&(a,b)| b);
    println!("{:?}",s.binary_search_by_key(&1, |&(a,b)| b)); // Console:" Ok(4) "
    assert!(match r { Ok(1..=4) => true, _ => false, });

}