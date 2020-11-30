
/*
0. Default type parameter 默认泛型参数(Base[p575~578)
    a. 可在使用泛型参数时为泛型指定一个默认的具体类型，当使用默认类型就能工作时，该 Trait
       实现者可以不用再指定具体类型。可以在定义泛型 Trait 时通过语法来为泛型指定默认类型
       (语法格式:" trait Trait<PlaceholderType=ConcreteType> {}")
    b. 默认泛型参数常用于运算符重载如加法运算符" + ":
       " std::ops::Add "源码：" Rhs = Self "即是默认类型参数
        #[doc(alias = "+")]
        pub trait Add<Rhs = Self> {
            /// The resulting type after applying the `+` operator.
            #[stable(feature = "rust1", since = "1.0.0")]
            type Output;

            /// Performs the `+` operation.
            #[must_use]
            #[stable(feature = "rust1", since = "1.0.0")]
            fn add(self, rhs: Rhs) -> Self::Output;
        }

1. (使用默认实现)示例 1 ：
      use std::ops::Add;
      #[derive(Debug,PartialEq)]
      struct Point {
          x: i32,
          y: i32,
      }
      // 使用默认类型参数：可不用再指定具体类型(即未指定则默认为默认类型)
      impl Add for Point {
          type Output = Self;
          fn add(self, rhs: Self) -> Self {
              Point{ x: self.x+rhs.x, y: self.y+rhs.y }
          }
      }
      assert_eq!(Point{x:1, y: 1 }+Point{ x: 2, y: 2 },Point{ x: 3, y: 3 })

2. (使用指定实现：如针对以不同单位存放值的结构体的转换计算)示例 2 ：
   指定 impl Trait<Type> 设置 Rhs 类型参数

*/
use std::ops::Add;
#[derive(Debug)]
struct Millimeters(u32);
struct Meters(u32);
// 指定 Rhs 类型参数实现
impl Add<Meters> for Millimeters {
    type Output = Self;
    fn add(self, rhs: Meters) -> Self::Output {
        Millimeters(self.0 + rhs.0 * 1000)
    }
}

fn main() {
    let millimeters = Millimeters(1);
    let meters = Meters(1);
    // Console : " Millimeters(1001) "
    println!("{:?}", millimeters + meters)
}

/*
3. 默认类型参数主要被用于以下两种场景：
    a. 扩展一个类型而不破坏现有代码(如"示例 2 ")
    b. 允许大部分用户都不需要的特定场合进行自定义 (如"示例 1 ")
    // 标准库提供的 Add trait 即是第二种场景实例：通常只需将两个同类型值相加;
       同时 Add trait 亦提供自定义额外行为的能力。在 Add Trait 的定义中使用
       默认类型参数意味着大多数情况下都无需指定类型参数(简化代码)
    // [自]分析：关联类型针对同一类型只能单次实现、泛型可多次实现而默认参数类型
                则是关联类型与泛型的综合(即针对同一类型可扩展实现且不破坏原代码)

*/