/* 需求：创建一个实体类,包含:
         用户 ID i32
         用户名 String
         用户年龄 u8
         用户标签:数组(最多5个)
         提供 new 函数: 用来初始化实体
 */
// 结构体命名：系统推荐大驼峰式(如" UserModel ")
#[derive(Debug)]
pub struct User<'a> {
    id: i32,
    pub username: String,
    age: u8,
    tags: [&'a str; 5],
}

impl<'a> User<'a> {
    pub fn new() -> User<'a> {
        User{
            id: 0,
            username: "".to_string(),
            age: 0,
            tags: ["";5]
        }
    }
}