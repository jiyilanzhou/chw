
/* @ 运算符:
     允许创建变量(存放值)的同时测试此变量值是否匹配模式
*/

enum Message {
    Hello{id: i32},
}

fn main() {
    let msg = Message::Hello{id: 25};
    match msg {
        /* 使用" @ "创建变量并测试变量值是否匹配模式
           ([自]主用于" => "表达式内使用"变量[属性值]"的场景)
        */
        Message::Hello{id: id_va @ 3..=7} => {
            println!("id_va: {}", id_va);
        },
        /* 直接测试属性值是否匹配模式：
           ([自]主用于" => "表达式内无需使用"属性值"的场景)
        */
        Message::Hello{id: 10..=20} => {
            println!("large");
        },
        // 分支内可直接获取绑定的属性值(但无法事先对其范围进行判断)
        Message::Hello{id} => {
            // 分支内可直接获取直接绑定于属性的值
            println!("id: {}", id);
        },
    }
    println!("Hello, world!");

}