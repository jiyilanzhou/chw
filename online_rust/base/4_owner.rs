
/*
0. 所有者
    函数获取所有权

 */

// 函数获取所有权
fn get_ownership(s:String) {
    println!("{}",s);
}
// 飘红报错"Function `change` must have a body" 待理解[?]
// fn change(s: mut String) {}
fn change(s: &mut String) {
    s.push_str("append");
    println!("{}",s);
}

fn main() {
    // 声明 String (堆空间)
    let variable = String::from("chw");
    let vary = variable.clone();
    // Console:" 0xa98030,0xa9bad0 "(地址不一致[说明堆栈都克隆了一份数据])
    println!("{:p},{:p}", variable.as_ptr(), vary.as_ptr());
    /* 追加引用(如 && )与引用计数 Rc 有什么区别：
         多重引用 && 是对栈空间变量的引用即引用指向变量(二级指针引用指向一级指向)
         而引用计数为拷贝栈空间指向数据的引用(引用指向堆空间数据[而非栈空间变量])
     */
    let vari = &variable;//"let vari = &&&variable;"效果一致(多重解引用)
    // Console:" 0xa98030,0xa98030 "(地址一致[说明 vari 指向(非拷贝) variable])
    println!("{:p},{:p}\n=============", variable.as_ptr(), vari.as_ptr());

    // 但针对 'static str 则不受此限制(因其固定不可变且存活于整个程序运行周期)
    let variable_str = "静心道";
    // 'static str 为静态字面量(未发生所有权转移[类比引用计数 Rc 功能])
    let vary_str = variable_str;    // 若为 String 则为所有权转移
    // Console:" 0x486190,0x486190 "(地址一致[说明仅拷贝一份栈空间的引用])
    println!("{:p},{:p}", variable_str.as_ptr(), vary_str.as_ptr());

    // 调用
    let mut var_str = String::from("683");
    // 修改原数据值待后续使用，则应传入 mut 修饰的引用
    change(&mut var_str);
    println!("{}",var_str);

    // 所有权转移
    get_ownership(var_str);
    // println!("{}",var_str);// 所有权转换后不可再用
}