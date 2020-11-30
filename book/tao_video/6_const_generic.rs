/*
0. 常量泛型(const generic)

1. 为什么需要常量泛型
   a. Rust 中的静态数组一直以来都是二等公民，不方便使用。如
       let arr: [i32; 3] = [1, 2, 3];
       let arr: [i32; 5] = [1, 2, 3, 4, 5];
       // 在其它编程里数组无论有多少个元素都算是同一种类型，但在 Rust 内则不然 (第一个数组和第二个数组是
          不同类型)，所以 Rust 内的数组是二等公民，无法使用一个泛型来统一定义不同长度的数组。为了解决这个
          问题，官方引入了常量泛型的概念。
    b. 官方引入 MayBeUninit 用于给泛型 T 生成一个未初始化的示例且定义了一个泛型结构体 ArrayVec (内含两个
       泛型，T 表示该数组中从属元素的类型，另一个是常量泛型 N 且指定其 usize 类型来为结构体中的字段 items
       指定一个长度为 N 的数组)。因为数组元素类型为泛型 T 并不确定元素具体类型，所以该数组的初始化值将由
       MaybeUninit 来进行占位，这就是常量泛型。因目前尚未稳定，故需在模块顶部使用追加一个内部 feature 属性
       的特性门，表明该 feature 只能在 nightly Rust 下使用，而常量泛型功能目前就包含在这个特性门中，需要
       说明的是此 features 代表最小化常量泛型即" min_const_generics "(官方团队为稳定该功能而划分的一个
       最小可用的特性范围)
     c. 目前常量泛型存在的缺陷
        (0). 常量泛型的初始化类型目前仅限于整数原生类型，包括有符号和无符号整数类型，布尔值和字符还不允许
             使用复合类型和自定义类型，也不允许使用引用，这就意味着不允许使用字符串。(此缺陷属于容易被修复
             的，近期会被官方修复)
        (1). 常量泛型参数目前只支持两种表达式：
             • 一个是简单的常量泛型参数，比如代码示例中的 `const N: usize`
             • 第二个是可以在不依赖于任何类型或常量参数的常量上下文中使用的表达式(如字面量、数学公式，使用
               常量或常量函数等，但是不能使用类似于" T;N+1 "这样的表达式)，此缺陷修复时间可能较为漫长但最终
               会被实现。

2. 类型理论
    a. 常量泛型是一种依赖类型（Depended Type）
       其实常量泛型可以看作是一种依赖类型，因为数组" [T;N] "这样的类型最终是要依赖 N 的具体值来确定
    b. 常量泛型（const generic） / 标准库内部应用如
        let data = [1, 2, 3, 4, 5, 6];
        // array_chunks() 需添加" #![feature(array_chunks)] "
        let sum1 = data.array_chunks()
            .map(|&[x, y]| { x * y })
            .sum::<i32>();
        assert_eq!(sum1, (1 * 2) + (3 * 4) + (5 * 6));
        let sum2 = data.array_chunks()
            .map(|&[x,y,z]| x * y * z )
            .sum::<i32>();
        assert_eq!(sum2,(1*2*3)+(4*5*6));

3. 常量泛型
    为了将数组提升为一等公民，特意引入了常量泛型(其本身属于一种依赖类型[依赖于数组具体的长度来确定数组类型])
    (但目前常量泛型功能尚未稳定同时也存在一些缺陷[有待 Rust 在后期发展中陆续支持与改进])

 */
#![feature(min_const_generics)]
// 此特性门用于支持在" const fn new() "函数中使用 MaybeUninit
#![feature(const_in_array_repeat_expressions)]
// #![feature(const_generics)]
#![feature(array_chunks)]

use core::mem::MaybeUninit;

#[derive(Debug)]
pub struct ArrayVec<T, const N: usize> {
    items: [MaybeUninit<T>; N],
    length: usize,
}

impl<T, const N: usize> ArrayVec<T, { N }> {
    /* const generic 目前要达到的目标：
          使用 MaybeUninit 、 添加数组常规化操作及内联优化(此功能不久的将来会稳定)
     */
    pub const fn new() -> ArrayVec<T, { N }> {
        ArrayVec {
            items: [MaybeUninit::uninit(); N],
            length: 0,
        }
    }

    // 添加数组的一些常规化操作且使用 #[inline] 属性对方法进行内联优化
    #[inline]
    pub const fn len(&self) -> usize {
        self.length
    }
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }
    #[inline]
    pub const fn capacity(&self) -> usize {
        N
    }
    #[inline]
    pub const fn is_full(&self) -> bool {
        self.len() >= self.capacity()
    }
}

impl<T, const N: usize> Drop for ArrayVec<T, { N }> {
    #[inline]
    fn drop(&mut self) {
        // Make sure the destructors for all items are run.
        // self.clear();
    }
}

fn main() {}

/*
小作业：
    1. 使用最新的 Rust 稳定版，尝试编写几个使用 if 、macth、loop 等表达式的常量函数，也可以尝试一下在常量
       函数里使用 println 宏进行打印，看看会你会发现什么？
    2. 在 crates.io 中搜索到 const-sha1 库，去看它的源码实现，看它的内部是如何应用 const fn 函数的。
    3. 思考标准库 中 std::vec::Vec 的 new 方法，为什么是常量函数？这样做有什么好处？
    4. 继续完善 ArrayVec 示例，参考： https://github.com/Michael-F-Bryan/constarrayvec

 */