
//1、trait_bound 语法
//2、指定多个 trait bound
//3、返回 trait 的类型

// trait 定义
pub trait GetInformation {
    fn get_name(&self) -> &String;
    fn get_age(&self) -> u32;
}

//1、trait_bound 语法
// 直接作为参数的写法：impl
fn print_information_00(item: impl GetInformation) {
    println!("name = {}", item.get_name());
    println!("age = {}", item.get_age());
}
// 使用 trait bound 的写法:(泛型[欲使用先声明])
fn print_information_01<T: GetInformation>(item: T){
    println!("name = {}", item.get_name());
    println!("age = {}", item.get_age());
}

//2、指定多个 trait bound
trait GetName {
    fn get_name(&self) -> &String;
}
trait GetAge {
    fn get_age(&self) -> u32;
}

// trait bound 写法 1: (泛型[欲使用先声明])
fn print_information_0<T: GetName+GetAge>(item: T){
    println!("name = {}", item.get_name());
    println!("age = {}", item.get_age());
}
// trait bound 写法 2: (泛型[where])
fn print_information<T>(item: T)
    where T: GetName+GetAge
{
    println!("name = {}", item.get_name());
    println!("age = {}", item.get_age());
}
// 直接作为参数的写法：impl
fn print_information_1(item: impl GetName + GetAge) {
    println!("name = {}", item.get_name());
    println!("age = {}", item.get_age());
}

#[derive(Debug)]
pub struct Student {
    pub name: String,
    pub age: u32,
}
impl GetName for Student {
    fn get_name(&self) -> &String {
        &self.name
    }
}
impl GetAge for Student {
    fn get_age(&self) -> u32 {
        self.age
    }
}
fn produce_item_with_age() -> impl GetAge {
    Student {
        name: String::from("xiaoming"),
        age: 15,
    }
}

#[derive(Debug)]
pub struct Teacher{
    pub name: String,
    pub age: u32,
}
impl GetAge for Teacher {
    fn get_age(&self) -> u32 {
        self.age
    }
}

/*// 多态错误用法：
    fn produce_item_with_age2() -> impl GetAge {
        let bl = true;
        if bl {
            Student {name:"".to_string(),age: 0,}
        } else {
            Teacher {name: "".to_string(),age: 0 }
        }
    }
// 编译报错：
    error[E0308]: if and else have incompatible types
      if bl {
     |         Student {name:"".to_string(),age: 0,}
     |         -----------------------------expected because of this
     |     } else {
     |         Teacher {name: "".to_string(),age: 0 }
     |         ^^^^ expected struct `Student`, found struct `Teacher`
     |     }
     |_____- if and else have incompatible types
     |
        = note: expected type `Student`
                   found type `Teacher`

*/

fn main() {
    let s = Student{name: "xiaoming".to_string(), age: 10};
    //print_information_1(s);
    print_information(s);

    // trait 接收
    let s = produce_item_with_age();
}
