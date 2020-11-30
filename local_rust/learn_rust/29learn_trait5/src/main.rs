
// 对任何实现某 trait 的类型有条件的实现特定 trait
trait GetName {
    fn get_name(&self) -> &String;
}
trait PrintName {
    fn print_name(&self);
}

/* 为泛型 T (受"GetName"限定)实现 PrintName trait ：
       即是给实现 GetName 的类型实现 PrintName trait (换言之
       只要实现 GetName trait 则为其亦实现 PrintName trait )
*/
//impl PrintName for GetName{  // 编译报错"help: use `dyn`: `dyn GetName`"(原因待查询[?])
impl<T: GetName> PrintName for T {  // 功能类似"接口继承"
    fn print_name(&self) {
        println!("name = {}", self.get_name());
    }
}

// Student 结构体及其" GetName trait "实现
struct Student {
    name: String,
}
impl GetName for Student {
    fn get_name(&self) -> &String {
        &(self.name)
    }
}

fn main() {
    let s = Student{name: String::from("stuName")};
    s.print_name();
}