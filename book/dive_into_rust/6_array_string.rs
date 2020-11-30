
/*
0. 数组和字符串

6.1.3 数组切片
    a. 对数组取借用 borrow 的操作可以生成一个" 数组切片 "。 数组切片对数组没有
       所有权故可将数组切片看作是专门用于指向数组的指针，是对数组的另一个视图。
    b. 示例(p68[*])

6.1.4 DST(Dynamic Sized Type：动态大小类型)和胖指针
    a. DST 指的是编译阶段无法确定占用空间大小的类型，为了安全性指向 DST 的指针
       一般是胖指针(数据指针及长度信息[即占用两个指针大小的内存空间])
    b. 胖指针：如对于不定长数组 [T] 有对应的胖指针 &[T] 类型；对于不定长字符串
       str 类型有对应的胖指针 &str 类型。由于不定长类型(如数组类型 [T] )在编译
       阶段无法确定占用空间大小，故目前暂不能在栈上声明一个不定长大小的类型实例，
       亦不能使用其作为函数的参数、返回值。但指向不定长数组的胖指针大小确定故其
       可用作变量实例、函数参数及返回值(如" &[T] ")
    c. DST 类型的限制(p69[*])
    d. 胖指针的设计原因：避免类数组类型作为参数传递时自动退化为裸指针类型，丢失
       长度信息的问题，保证了类型安全

6.1.5 Range ： 区间、范围(p70~72[*])
    a. Rust 中 Range 代表一个区间、范围，为内置语法
    b. " std::ops::Range<_> "本身实现了 Iterator trait 故其可直接应用到循环中
    c. " std::ops::Range "对应" start .. end "左开右闭即" [start,end) "
    d. " std::ops::RangeFrom "对应" start .. "即" [ start , + ∞ ) "
    e. " std::ops::RangeTo "对应"  .. end "即" [- ∞ , end ) "，对于无符号的
        含义则为" [ 0, end ) "
    f. " std::ops::RangeFull "对应"  ..  "即" [- ∞ , + ∞ )"，对于无符号的
        含义则为" [ 0, + ∞ ) "

6.1.6 边界检查
    a. Rust 目前还无法执行编译阶段的边界检查，故而是在运行时执行了边界检查
    b. 在 Rust 中" 索引 "操作亦是一个通用的操作符，可自行扩展。若希望某个类型可
       执行"读操作"就需该类型实现" std::ops::Index trait ",若希望可执行"写操作"
       则需该类型实现" std::ops::IndexMut trait "
    c. 在 Rust 中对于明显的数组越界行为可通过 lint 检查来发现(可参考"clippy"项目)。
       总体而言在 Rust 里面靠编译阶段静态检查暂时无法消除数组越界行为
    d. 一般情况下 Rust 不鼓励大量使用"索引"操作，因正常的"索引操作"都会执行一次"边界
       检查"，在 Rust 中更为地道的做法是尽量使用"迭代器"

6.2 字符串
    主要涉及两种类型( &str 及 String )

6.2.1 &str (p74~75[*])
    a. str 是 Rust 的内置类型， &str 是对 str 的借用。Rust 的字符串内部默认是使用
       utf-8 编码格式，而内置的 char 类型是 4 个字节长度(存储的是 Unicode Scalar
       Value )故 Rust 中的字符串不能视为 char 类型的数组而更接近 u8 类型的数组
    b. str 类型有一种方法" fn as_ptr(&self) -> *const u8 "其内部无需任何计算只需
       强制类型转换即可
    c. 寻找字符串 s 内部的第 n 个字符不能直接通过 s[n] 得到，在 Rust 中而是应该通过
       " s.char().nth(n) "获取
    d. (类似数组) str 是 DST 类型，其对应的 &str 是"字符串切片类型"，&str 类型亦是
       一个胖指针(内部包含一个指向片段头部的指针和一个长度)

6.2.2. String
    a. 与" &str "类型的主要区别是 String 有管理内存空间的权力. " &str "类型是对一块
       字符串区间的借用(其对所指向的内存空间没有所有权即使" &mut str "亦如此)
    b. String 类型在堆上动态申请了一块内存空间，其有权对这块内存空间进行扩容，其内部
       实现类似 std::Vec<u8> 类型(故可将此类型作为容纳字符串的容器使用)

*/