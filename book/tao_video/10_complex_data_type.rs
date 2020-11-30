/*
0. 自定义复合类型
    a. 结构体 Struct
    b. 枚举体 Enum
    c. 联合体 Union

1. 结构体 Struct
    a. 具名结构体
        struct Point {      // 具名结构体：包含字段
            x: f32,
            y: f32,
        }
    b. 元组结构体
        (0). struct Pair(i32, f32);   // 元组结构体
        (1). 元组结构体一般用于包装一些基本数据类型去扩展其功能如:
                struct Score(u32);
                impl Score {
                    fn pass(&self) -> bool {
                        self.0 >= 60
                    }
                }
                fn main() {
                    let s = Score(59);
                    assert_eq!(s.pass(), false);
                }
        (2). NewType 模式 : 当元组结构体只包含一个成员时即为 newtype 模式(包装类)
    c. 单元结构体
        struct Unit; // 单元结构体：其实例就是单元结构体本身(无论创建多少个实例，编译器都会将其优化为一个，
                     // 单元结构体也不会占用实际的内存空间，是一个零大小类型)

2. 结构体内存布局
    Rust 编译器会对结构体进行内存对齐，以便提升 CPU 访问效率，结构体中对齐规则一般以其成员中最大对齐元素为
    对齐大小，且每个成员都得进行对齐

 */
struct A {
    a: u8,      // 1 byte
    b: u32,     // 4 byte
    c: u16,     // 2 byte
}
#[repr(C)]      // 指定结构体 B 按照 C 语言结构体布局故编译器不会重排(可手动优化内存布局)
struct B {
    a: u8,      // 1 byte
    b: u32,     // 4 byte
    c: u16,     // 2 byte
}
fn main_0() {
    /* 内存对齐：
        按字段 a,b,c 顺序来看最大对齐元素是 u32 类型(32 bit 即 4 byte)。理论上应按 u32 的 4 个字节对齐则
        u8 应补齐 3 字节，u16 补齐 2 字节，结构体 A 大小就是每个字段补齐后的大小之和，总共应占用 12 字节。
        但实际上 Rust 编译器会进行字段重排以最大程度优化内存占用。
            // 编译器重排字段：优化内存占用(优化后的结构体 A)，这样一来，只需要字段 a 补齐 1 个字节即是
            // 结构体 A 最大对齐字段 u32 的倍数，满足了 CPU 的优化要求(故编译器进行字段重排)
            struct A {
                a: u8,
                b: u32,
                c: u16,
            }
        // 取消编译器自动对字段优化重排可用内在布局属性" #[repr(C)] "来指定 C 语言结构体的内存布局。换言之
        // 当必须使用 C 语言内存布局时可手动修改结构体字段顺序来优化内在布局(如调换"a、b"或"b、c"位置)。
     */
    println!("{:?}",std::mem::size_of::<A>());      // Console: " 8 "
    println!("{:?}",std::mem::size_of::<B>());      // Console: " 12 "
}

/*
3. 枚举体与联合体内存布局
    a. (枚举体内存布局与联合体内存布局相似)以枚举类型成员最大的对齐值为准(无需为每个枚举值都对齐)
    b. 枚举体实质上就是带 tag 的联合体, tag 可理解为一种编号(如同元组索引)，枚举体中的每个值不占大小，但
       tag 占 1 个字节。

 */
// 枚举体 C 中的每个值实际上不占大小但 tag 占 1 个字节故总占用 1 byte
enum C {
    One,
    Two,
}
enum E {
    /* 函数构造项：
       a. 此枚举除简单枚举值 N (此枚举值亦可称作"判别式")外还有两个函数项构造器(类型构造器)，其会根据类型
          构造类函数项。
       b. 此枚举内最大对齐类型值为" M(Box<u32>) ": M 的 tag 占 1 byte、Box<u32> 占 8 byte，故为对齐则
          tag 需再补 7 byte 。因此 M(Box<u32>) 共占用 16 byte
     */
    N,
    H(u32),         // 8 byte
    M(Box<u32>)     // 16 byte
}
// 联合体 U 因其没有 tag 故其总占用即是其内部最大类型对齐值
union U {
    u: u32,     // 4 byte
    v: u64,     // 8 byte
}

fn main() {
    println!("E:{:?}",std::mem::size_of::<C>());        // Console:" 1 "
    println!("E:{:?}",std::mem::size_of::<E>());        // Console:" 16 "
    println!("E:{:?}",std::mem::size_of::<Box<u32>>());        // Console:" 8 "
    println!("U:{:?}",std::mem::size_of::<U>());        // Console:" 8 "
}
