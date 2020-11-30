
/*
0. 完全限定语法

 */
trait A{
    fn func(){println!("Trait A");}
}
trait B{
    fn func(){println!("Trait B");}
}
struct C;
impl A for C{}
impl B for C{}
impl C{
   // fn func(){println!("Struct C");}
}

fn main() {
    <C as A>::func();
    C::func();
    // 完全限定语法格式 : 等价于" C::func() "
    <C>::func();
}