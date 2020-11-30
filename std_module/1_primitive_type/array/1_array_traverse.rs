
/*
0. 数组遍历
   (文档" https://doc.rust-lang.org/std/primitive.array.html ")

 */
// An array itself is not iterable:
fn main() {
    let mut array: [i32; 3] = [0; 3];
    /*编译错误：
      error[E0277]: `[i32; 3]` is not an iterator
          for x in array {}
                   ^^ borrow the array with `&` or call `.iter()` on it to iterate over it
         = help: the trait `std::iter::Iterator` is not implemented for `[i32; 3]`
         = note: arrays are not iterators, but slices like the following are: `&[1, 2, 3]`
         = note: required by `std::iter::IntoIterator::into_iter`
       解决方案 1 (推荐使用)：通过调用 slice 方法将 array 强制为 slice
       解雇方案 2 (数组元素数量不高于 32 个时) ：可使用数组引用实现的 IntoIterator 实现
     */
    // for x in array {}

    // 范围遍历
    for i in 0..array.len(){
        // print!("{}\t",array[i]);
    }

    /* (数组遍历)解决方案 1 (推荐使用)：通过调用 slice 方法将 array 强制为 slice
        源码：(源于" ...\rust\src\libcore\slice\mod.rs "）
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

     */
    for i in array.iter() {
        //print!("{}\t",i);       // Console:" 0	0	0 "
    }
    /* 通过调用 slice 方法将 array 强制为 slice
       源码：(源于" ...\rust\src\libcore\slice\mod.rs "）
            #[stable(feature = "rust1", since = "1.0.0")]
            impl<'a, T> IntoIterator for &'a [T] {
                type Item = &'a T;
                type IntoIter = Iter<'a, T>;
                fn into_iter(self) -> Iter<'a, T> {
                   // 本质亦是调用 slice 本身实现 iter
                    self.iter()
                }
            }
       可使其可变的源码：(来源同上)
           #[stable(feature = "rust1", since = "1.0.0")]
           impl<'a, T> IntoIterator for &'a mut [T] {
               type Item = &'a mut T;
               type IntoIter = IterMut<'a, T>;
               fn into_iter(self) -> IterMut<'a, T> {
                  // 本质亦是调用 slice 本身实现 iter
                   self.iter_mut()
               }
           }
       // 存在现象及缘由:" array.into_iter() "虽可读写取但" array. "却没有
          相应的" into_iter "方法提示：原因是 into_iter() 为数组引用或切片
          引用的方法(数组虽然可强制为切片但也基于调用切片方法后，故调用方法前
          没有".into_iter"方法提示)。若修改为" (&array). "则有".into_iter"
          方法提示(因数组引用实现了 IntoIterator trait )。
     */
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

    /* (数组遍历)解决方案 2 (数组元素数量不高于 32 个时) ：可使用数组引用的 IntoIterator 实现
        源码：(源于"...\rust\src\libcore\array\mod.rs")
            #[stable(feature = "rust1", since = "1.0.0")]
            impl<'a, T, const N: usize> IntoIterator for &'a [T; N]
            where
                [T; N]: LengthAtMost32,
            {
                 // Item 未使用上是否可取消赋值或赋值为空 tuple (因其对结果无影响)[???]
                type Item = &'a T;
                type IntoIter = Iter<'a, T>;
                // 既然已经定义 Iter<'a,T> 的别名但其下为何不使用 Self::IntoIter 替代[???]
                fn into_iter(self) -> Iter<'a, T> {
                    self.iter()
                }
            }
        可使其可变的源码:(来源同上)
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
     */
    /* // for i in &array{ ... }    // " &array "即是调用" &array.into_iter() "
       // for i in (&array).into_iter(){...}   //即调用数组引用的" into_iter() "
     */
    // 使用数组引用的 IntoIterator 实现修改
    for i in (&mut array).into_iter() {
        print!("{}\t", i);       // Console:" 1	1 1 "
        *i += 1
    }
    // 使用数组引用的 IntoIterator 实现读取
    for i in (&array).into_iter() {
        print!("{}\t",i);       // Console:" 2 2 2 "
    }
    // println!("{:?}", array);

    /* (综上所述)针对数组的读写可有两种方式:
        方式 1 ：直接使用数组引用 &array 实现的 IntoIterator
                 其内" into_iter() " : 封装" iter() "及" iter_mut() "的实现
        方式 2 ：调用切片的方法(强制为切片)
                 直接使用 slice 本身实现的" iter "及" iter_mut() "的方法实现
                 当然亦可使用切片引用实现 IntoIterator (同理其内"into_iter()"
                 封装" iter() "及" iter_mut() "方法实现[范围:LengthAtMost32]
        综述：无论数组还是切片皆使用" iter() "及" iter_mut() "即可
     */
}
