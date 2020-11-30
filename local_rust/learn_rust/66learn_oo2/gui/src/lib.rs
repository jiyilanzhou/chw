
// Draw trait
pub trait Draw {
    fn draw(&self);
}

// (屏幕)结构体及其实现
pub struct Screen {
    // 屏幕组件
    pub components: Vec<Box<dyn Draw>>,  // trait 对象(使用 dyn 关键字)
}
impl Screen {
    pub fn run(&self) {
        for comp in self.components.iter() {
            comp.draw();
        }
    }
}

// (使用泛型)屏幕结构体及其实现
pub struct Screen2<T: Draw> {
    pub components: Vec<T>,
}
impl<T> Screen2 <T>
    where T: Draw {
    pub fn run(&self) {
        for comp in self.components.iter() {
            comp.draw();
        }
    }
}

// (屏幕组件)结构体及其实现
pub struct Button {
    pub width: u32,     // 宽
    pub height: u32,    // 高
    pub label: String,  // 标签
}
pub struct SelectBox {
    pub width: u32,
    pub height: u32,
    pub option: Vec<String>,
}
impl Draw for Button {
    fn draw(&self) {
        println!("draw button, width = {}, height = {}, label = {}",
                 self.width, self.height, self.label);
    }
}
impl Draw for SelectBox {
    fn draw(&self) {
        println!("draw selectBox, width = {}, height = {}, option = {:?}",
                 self.width, self.height, self.option);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
