
// 使用 trait bound 有条件的实现方法
trait GetName {
    fn get_name(&self) -> &String;
}
trait GetAge {
    fn get_age(&self) -> u32;
}
struct PeopleMatchInformation<T, U> {
    master: T,
    student: U,
}
impl<T: GetName+GetAge, U: GetName+GetAge> PeopleMatchInformation<T, U> {
    fn print_all_information(&self) {
        println!("in print_all_information");
        println!("master name = {}", self.master.get_name());
        println!("master age = {}", self.master.get_age());
        println!("student name = {}", self.student.get_name());
        println!("student age = {}", self.student.get_age());
    }
}

// Teacher 结构体及其"GetName、GetAge"实现
struct Teacher {
    name: String,
    age: u32,
}
impl GetName for Teacher {
    fn get_name(&self) -> &String {
        &(self.name)
    }
}
impl GetAge for Teacher {
    fn get_age(&self) -> u32 {
        self.age
    }
}

// Student 结构体及其"GetName、GetAge"实现
struct Student{
    name: String,
    age: u32,
}
impl GetName for Student{
    fn get_name(&self) -> &String {
        &(self.name)
    }
}
impl GetAge for Student{
    fn get_age(&self) -> u32 {
        self.age
    }
}

fn main() {
    // 声明
    let t = Teacher{name: "teacher".to_string(), age:30};
    let s = Student{name: "student".to_string(), age:15};
    let m = PeopleMatchInformation{master: t, student: s};
    // 调用方法
    m.print_all_information();

}
