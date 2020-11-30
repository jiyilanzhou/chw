
// mutable[ˈmjuːtəbl]adj. 可变的       // mut[mʌt]n.笨蛋,傻瓜
/*
0. 泛型：泛指的类型(即不定类型)
   场景：如公司合并后采用两套不同类型的标准记录分数
   a. 解决方案 1 ：使用不同结构体 如
                   struct ScoreA {
                       id: String
                   }
                   struct ScoreB {
                       id: i32
                   }
    b. 解决方案 2 ：使用泛型(降低冗余、易维护)
                   struct Score<T> {
                       id: T
                   }

 */
#[derive(Debug)]
struct Score<T> {
    id: T
}

impl<T> Score<T> {
    // 关联函数
    fn new(id:T) -> Score<T> {
        Score {
            id
        }
    }
    // 获取 id
    fn get_id(&self)->&T{
        &self.id
    }
}
// 外部初始化
fn init<T>(id:T)->Score<T>{
    Score{
        id
    }
}

fn main() {
    // 使用外部函数 init() 初始化
    let s = init("uuid");
    println!("{:?}\t id:{}",s,s.get_id());
    // 使用关联函数 new()
    let s = Score::new(3);
    println!("{:?}\t id:{}",s,s.get_id());
}