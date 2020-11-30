
/* 迭代器：
    1、迭代器负责遍历序列中的每一项和决定序列何时结束的逻辑。
    2、创建迭代器：迭代器是惰性的，意思就是在调用方法使用迭代器之前，不会有任何效果
    3、每个迭代器都实现了 iterator trait (定义于标准库)：
        trait Iterator {
            type Item;
            // type Item 和 Self::Item 这种用法叫做定义 trait 的关联类型
            fn next(mut self) -> Option<Self::Item>;
            // ...
        }
    // next 是 Iterator 要求实现的唯一方法其一次返回一个元素，当迭代器结束时返回 None

*/

fn main() {
    let v1 = vec![1, 2, 3];
    let mut v1_iter = v1.iter(); //到目前为止，不会对 v1 产生任何影响
    /* // 迭代器遍历
        for val in v1_iter {
            println!("val = {}", val);
        }
    */

    if let Some(v) = v1_iter.next() {
        println!("v = {}", v);          // 1
    }
    if let Some(v) = v1_iter.next() {
        println!("v = {}", v);          // 2
    }
    if let Some(v) = v1_iter.next() {
        println!("v = {}", v);          // 3
    }
    if let Some(v) = v1_iter.next() {
        println!("v = {}", v);
    } else {
        println!("At end");
    }

    //-----迭代可变引用-----
    let mut v2 = vec![1, 2, 3];
    let mut v2_iter = v2.iter_mut();
    if let Some(v) = v2_iter.next() {
        *v = 3;
    }
    println!("v2 = {:?}", v2);

    //-----消费适配器([自]在迭代器基础上消费其内元素)------
    let v1 = vec![1, 2, 3];
    /*" v1.iter() "返回 Iter 即"pub fn iter(&self)->Iter<'_,T>"其部分源码如下:
        pub struct Iter<'a, T: 'a> {
            ptr: *const T,
            end: *const T,
            _marker: marker::PhantomData<&'a T>,
        }
    */
    let v1_iter = v1.iter();
    let total: i32 = v1_iter.sum();     // 调用消费适配器 sum 来求和
    println!("total = {}", total);

    //-----迭代适配器 map ------
    println!("++++++++++++");
    let v1 = vec![1, 2, 3];
    println!("v1 = {:?}", v1);
    // " map "对迭代器内每元素做" +1 "适配操作
    //let v2 = v1.iter().map(|x| x+1);
    //println!("v2 = {:?}", v2);  // Console:" v2 = Map { iter: Iter([1, 2, 3]) } "
    let v2: Vec<_> = v1.iter().map(|x| x+1).collect();
    println!("v2 = {:?}", v2);  // Console:" [2, 3, 4]"
    //-----迭代适配器 filter ------
    println!("++++++++++++");
    let v3 = vec![1, 23, 4, 56];
    println!("v3 = {:?}", v3);  // Console:" v3 = [1, 23, 4, 56] "
    let v4: Vec<_> = v3.into_iter().filter(|x| *x > 5).collect();
    println!("v4 = {:?}", v4);  // Console:" v4 = [23, 56] "

}
