
/*
1、泛型是具体类型或者其它属性的抽象替代，用于减少代码重复.
2、在函数定义中使用泛型。
3、在结构体中使用泛型。
4、枚举中的泛型。
5、方法中的泛型。
6、总结：使用泛型并不会造成程序性能上的损失。rust 通过编译时泛型代码单态化来保证效率。
        单态化时通过填充编译时使用的具体类型，将通用代码转换为特定代码的过程。
*/

/* --------未使用泛型-----------
    // 处理 i32 类型
    fn largest_i32(list: &[i32]) -> i32 {
        let mut largest = list[0];
        for &item in list.iter() {
            if item > largest {
                largest = item;
            }
        }
        largest
    }
    // 处理 char 类型
    fn largest_char(list: &[char]) -> char {
        let mut largest = list[0];
        for &item in list.iter() {
            if item > largest {
                largest = item;
            }
        }
        largest
    }
*/

// ----------使用泛型-------------
// (编译报错)问题：难道" impl xxx "用于返回值有特殊限制[?]
//fn largest(list: &[impl PartialOrd + Copy]) -> impl PartialOrd + Copy {
fn largest<T: PartialOrd + Copy> (list: &[T]) -> T {
    let mut larger = list[0];
    for &item in list.iter() {
        if item > larger {      // 若未实现" PartialOrd "则不可比较
            larger = item;      // 若未实现" Copy "则赋值会发生所有权转移
        }
    }
    larger
}

fn main_00() {
    // 处理 i32
    let number_list = vec![1, 2, 23, 34, 8, 100];
    // 使用具体类型(未使用泛型)
    //let max_number = largest_i32(&number_list);
    // 使用泛型
    let max_number = largest(&number_list);
    println!("max_number = {}", max_number);

    // 处理 char
    let char_list= vec!['a', 'y', 'b'];
    // 使用具体类型(未使用泛型)
    //let max_char = largest_char(&char_list);
    // 使用泛型
    let max_char = largest(&char_list);
    println!("max_char = {}", max_char);
}

//------在结构体中使用泛型--------
#[derive(Debug)]
struct Point0<T> {
    x: T,
    y: T,
}

#[derive(Debug)]
struct Point1<T, U> {
    x: T,
    y: U,
}

fn main_01() {
    // 数值类型
    let integer = Point0{x: 1, y: 2};
    println!("{:#?}", integer);
    // 浮点类型
    let float = Point0{x: 1.1, y: 2.2};
    println!("{:?}", float);
    // "浮点、字符"混合类型
    let a = Point1{x: 1.1, y: 'a'};
    println!("a = {:?}", a);
}

//----------在枚举中使用泛型------------
enum Option<T> {
    Some(T),
    None,
}
enum Result<T, E> {
    Ok(T),
    Err(E)
}

//-----------在方法中使用泛型--------------
struct Point<T> {
    x: T,
    y: T,
}
impl<T> Point<T> {
    fn get_x(&self) -> &T {
        &self.x
    }
    fn get_y(&self) -> &T {
        &self.y
    }
}
struct Point2<T, U> {
    x: T,
    y: U,
}
impl<T, U> Point2<T, U> {
    fn creat_point<V, W>(self, other: Point2<V, W>) -> Point2<T, W> {
        Point2 {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    // 成员属性类型相同：数值类型
    let p = Point{x:1, y: 2};
    println!("x = {}", p.get_x());
    println!("y = {}", p.get_y());
    // 成员属性类型相同：浮点类型
    let p = Point{x:1.1, y: 2.2};
    println!("x = {}", p.get_x());
    println!("y = {}", p.get_y());

    // 成员属性类型不同
    let p1 = Point2{x: 5, y: 1.1};
    let p2 = Point2{x: "hello", y: 'c'};
    let p3 = p1.creat_point(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

}