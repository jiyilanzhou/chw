
// 获取命令行参数
fn main() {
    /* 须标注类型：因 args 可能为不同类型
        " env::args() "返回的" Args "结构体已实现 Iterator
        故可用" collect() "方法收集元素
     */
    let args: Vec<String> = std::env::args().collect();
    println!("size = {}",args.len());
    /*for arg in args {
        println!("{}", arg);
    }*/
    println!("{:?}",args);
    /* 索引元素
       // [自]不能将元素直接从容器中移出从而对原容器造成影响
       let a = args[0];  // 飘红报错:" Cannot move "
     */
    let a = &args[0];
    let b = args[0].clone();// 效率低下(不推荐)
    println!("{} --- {}",a,b);

    // 详情参阅" std::env "库
}