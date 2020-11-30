
// 实现一个缓存，只处理第一次传入的值并保存
struct Cacher<T>                // 定义缓存
    where T: Fn(u32) -> u32     // 泛型限定为 Fn
{
    calculation: T,
    value: Option<u32>,
}

// 实现
impl<T> Cacher<T>
    where T: Fn(u32) -> u32
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,      // 形参实参同名可省写
            value: None,
        }
    }
    // [自]返回非引用故无需使用生命周期标识
    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                /* self.calcaulation 即" Fn(u32)->u32 "
                   故" (self.calculation)(arg) "即函数调用。
                 */
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            },
        }
    }
}

fn main() {
    // 匿名函数接收闭包(即相当于设置"回调函数")
    let mut c = Cacher::new(|x| x+1);

    // 仅首次初始化
    let v1 = c.value(1);
    println!("v1 = {}", v1);        // Console :" 2 "

    // 后续调用不再初始化
    let v2 = c.value(2);
    println!("v2 = {}", v2);        // Console :" 2 "

}
