
#[derive(Debug)]
struct Dog {
    name: String,
    weight: f32,
    height: f32,
}

// 绑定关联函数或方法
impl Dog {
    // 方法
    fn get_name(&self) -> &str {
        &(self.name[..])
    }
    fn get_weight(&self) -> f32 {
        self.weight
    }
    // 关联函数
    fn show() {
        println!("oh oh oh");
    }
}
// 同一结构体实现允许多个 impl 模块
impl Dog{
    fn get_height(&self) -> f32 {
        self.height
    }
}

fn main() {
    let dog = Dog {
        name: String::from("wangcai"),
        weight: 100.0,
        height: 70.5,
    };

    println!("dog = {:#?}", dog);
    println!("name = {}", dog.get_name());
    println!("weight = {}", dog.get_weight());
    println!("height = {}", dog.get_height());
    Dog::show();

}
