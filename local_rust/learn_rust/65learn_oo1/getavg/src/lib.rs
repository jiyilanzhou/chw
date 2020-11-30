
/* 面向对象特性：
 1. 封装 ： 隐藏内部实现

 2. 继承 ： Rust 中无继承概念(其通过 trait 实现共享共享)
            trait A {
                fn method(){
                    // TODO
                }
            }
            struct SN{}
            impl A for SN{}

 3. 多态

*/
// 封装(隐藏内部实现)
pub struct AvgCollect {
    list: Vec<i32>,
    aver: f64,
}
impl AvgCollect {
    // 关联函数
    pub fn new() -> AvgCollect {
        AvgCollect {
            list: vec![],
            aver: 0.0,
        }
    }
    // 方法:追加数据
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }
    // 移除
    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }
    // 平均值
    pub fn average(&self) -> f64 {
        self.aver
    }
    // 更新
    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.aver = total as f64 / self.list.len() as f64;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
