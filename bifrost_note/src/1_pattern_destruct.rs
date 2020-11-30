
#[derive(Debug)]
pub struct SN {
    field:String,
    age:i32,
    score:f64
}

pub fn func( deps: SN){
    // 模式解构
    /*let SN {
        field
    } = deps;*/

    let  SN {
        field,
        score,
        // " .. "必须置于最后忽略其余属性
        ..

    } = deps;
    // 直接使用解析后的元素
    println!("{}",field);
    println!("{}",score);
}

fn main() {
    let sn = SN{
        field:String::from("chw683"),
        score:68.0,
        age:3
    };
    func(sn);
}
