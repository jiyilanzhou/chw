
// rustaceans[rʌsteɪʃns]n. Rust 开发者们
/*
0. Option 及 Result API


1. Send trait 和 Sync trait
    数据竞争是线程安全最大的" 隐患 ": Golang 提供协程和 CSP 并发模型解决方案；Rust 则从
    正面解决问题即依赖"类型系统"及"所有权"机制( Rust 提供了 Send 及 Sync 两个标签 trait
    [其是 Rust 无数据竞争并发的基石])。" Send、Sync "标签 trait 内部无具体方法实现(类比
    "Copy、Sized")。在多线程间传递未实现 Send 及 Sync 的类型则编译报错
     源码:   pub unsafe auto trait Send {
                 // empty.
             }
     源码:   pub unsafe auto trait Sync {
                 // Empty
             }

2. rayon 第三方库使用( Rust 核心成员编写)
    // 未使用第三方库(顺序迭代)
    use std::path::PathBuf;
    fn load_images(paths: &[PathBuf]) -> Vec<Image> {
        paths.iter() // For each path ...(顺序迭代器)
            .map(|path|{
            Image::load(path)       // ...load an image...
                                    // (图像间无关联故应该并行执行[可使用第三方库 rayon ])
        })
        .collect()         // ... create and return a vector
    }

    // 使用第三方库完成并行执行
    extern crate rayon; // Third-party library
    fn load_images(paths: &[PathBuf]) -> Vec<Image> {
        let mut jpegs = 0;
        paths.par_iter()                 // ... make it parallel (并行迭代器[并行执行])
            .map(|path|{
                // 并行加载图像并统计" .jpeg "格式数量(并发执行会造成数据竞争)
                // 可使用" AtomicU32，Mutex 等修正( To fix : use AtomicU32,Mutex,etc)
                if path.ends_with("jpeg"){ jpegs += 1;} // Data-race (数据竞争：编译错误)
                Image::load(path)       // ...load an image...
        })
        .collect()         // ... create and return a vector
    }
    // Can also do : processes with channels,mutexs,non-blocking data structures ...
    // 文档编写：视频" Into Rust (3) - 所有权 [时间 ：16:10 ~ 17:38 ] "

 */


fn main() {

}

/*
   c. 智能指针
        智能指针可让 Rust 利用栈来隐式自动释放堆内存，从而避免显式调用 free 之类的函数
        去释放(更加便于开发)
 */