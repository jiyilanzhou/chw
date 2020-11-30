
fn main() {
    // if
    let y = 0;
    if y == 1 {
        println!("y = 1");
    }

    // if-else
    if y == 1 {
        println!("y = 1");
    } else {
        println!("y != 1");
    }

    // if - else if - else
    println!("++++++++++++");
    let y = 2;
    if y == 1 {
        println!("y = 1");
    } else if y == 0 {
        println!("y = 0");
    } else if y == 2 {
        println!("y = 2");
    } else {
        println!("other");
    }

    // let 中使用 if
    let condition = true;
    let x = if condition {
        5
    } else {
        6
        //"six" // error (分支须返回相同类型)
    };  // 此处须以分号";"结束(因 let 绑定变量为语句)
    println!("x = {}", x);

    // loop
    let mut counter = 0;
    loop {
        println!("in loop");
        if counter == 10 {
            break;
        }
        //counter = counter + 1;
        counter += 1;
    }
    
    let result = loop {
        counter += 1;
        if counter == 20 {
            // "break"携带数据返回(类比函数中使用"return")
            break counter*2;    // 返回数据
        }
    };
    println!("result = {}", result);

    // while
    let mut i = 0;
    while i != 10 {
        i += 1;
    }
    println!("i = {}", i);

    // for
    println!("+++++++++++++++");
    let arr:[u32; 5] = [1, 2, 3, 4, 5];
    // 使用 iter(迭代器) 遍历
    for element in arr.iter(){
        println!("element = {}",element);
    }
    // 使用 &(引用) 遍历
    for element in &arr {
        println!("element = {}", element);
    }

}
