
/* 文件(如" animal.rs ")作为模块：
   可于" src/lib.rs "内将其暴露(如" pub mod animal; ")
   后方可按需使用
*/
pub mod dog {
    pub fn hello() {
        println!("wangwang");
    }

    pub fn is_dog() -> bool {
        true
    }
}

pub mod cat {
    pub fn hello() {
        println!("miaomiao");
    }

    pub fn is_cat() -> bool {
        true
    }
}