
/*
0. Trait
   trait 中可以包含"函数、常量、类型"等

5.1 成员方法
    a. 所有的 trait 都有一个隐藏的类型 Self(大写"S")，代表实现了当前 trait 的
       具体实现类型。trait 中定义的函数亦可称作是关联函数(associated function)。
    b. 函数第一个参数若是 Self 相关的类型且命名为 self(小写"s")则此参数可被称为
       接收者(receiver)，具有 receiver 参数的函数即为方法(method[可通过点号"."
       进行调用])。没有 receiver 参数的函数称为静态函数(static function)，其可
       通过"类型加双冒号"方式来调用(Rust中的函数和方法没有本质区别)
    c. Rust 中 Self(大写) 和 self(小写) 皆为关键字分别表示 类型名 及 变量名。
       self 参数亦可指定类型(必须是包装在 Self(大写) 类型之上的类型)
    d. 对于第一个 self 参数，常见的类型有" self : Self "、" self : &Self "以及
       " self:&mut Self "等类型，可分别简写为" self "、" &self "及" &mut self "
       且 self 参数仅用于首参数第一个位置。如
            trait T{
                fn method1(self:Self);       // 简写 fn method1(self);
                fn mehtod2(self:&Self);      // 简写 fn method2(&self);
                fn method3(self:&mut Self);  // 简写 fn method3(&mut self);
            }
       // 具体实现类型
            impl T for Struct {
                // Self 类型是具体实现类型 Struct
                // 故 self 的类型是 &Self 即 &Struct
                fn method(&self){}
            }
    e. 针对类型仅使用 impl (而无须实现任何 trait )增加的其成员方法为此类型的" 内在
       方法(inherent methods) "
    f. trait 中可包含函数或方法的默认实现如
           trait T {
                fn func(){ println!("hello");}
                fn method(&self){ println!("world"); }
            }
            struct S;
            impl T for S{}
            fn main() {
                <S as T>::func(); // Console:" hello "
                let s = S;
                s.method();     // Console:" world "
            }
    g. self 参数甚至可以是 Box 指针类型 self : Box<Self> (目前 Rust 设计组亦在考虑
       让 self 变量的类型放得更宽允许更多的自定义类型作为 receiver )示例参见"p52[*]"
    h. impl 对象甚至可以是 trait 即" impl Trait for Trait "(p52~53[*])
       (0). impl Sharp for Round{} 和 impl<T:Round> Sharp for T{} 区别: 前者 self
            是 &Round 类型即" trait object "(胖指针)后者 self 是 &T 类型(具体类型)；
            前一种写法是为 trait object 增加成员方法而后一种写法是为所有满足 T:Round
            的具体类型增加成员方法
       (1). impl Sharp for Round{} 亦可写为"impl Sharp for dyn Round{}"(语义更明确)

*/
trait Sharp{
    fn area(&self)->f64;
}
trait Round{
    fn get_radius(&self)->f64;
}

impl Sharp for Round{
    fn area(&self) -> f64 {
        unimplemented!()
    }
}
impl<T:Round> Sharp for T{
    fn area(&self) -> f64 {
        unimplemented!()
    }
}

/*
5.2 静态方法
    a. 没有 receiver 参数的方法(即首参数非 self 变量名[即使为 Self 相关类型]的方法)
       称作"静态方法"(即首参数即使为 Self 相关类型但只要其变量名不是 self 则不能通过
       点号"."调用相应方法)。静态方法可通过 Type::functionName() 方式调用
    b. 标准库提供的示例如" Box::into_raw(b:Self)、Box::leak(b:Self) 以及 Rc 一系列
       方法 Rc::try_unwrap(this:Self)、Rc::downgrade(thjis:Self) "等都是设计接收者
        receiver 非 self 关键字的情况：其目的是强制用户使用" Type::functionName() "
       而禁止 obj.method(args) 形式的调用，这样源码表达出来的意思更清晰(可避免成员方法
       重名而造成误解),如"不会因为 Rc<T> 和 T 里面的成员方法重名而造成误解"(p54[*])。
    c. trait 中亦可定义静态函数如
       pub trait Default{
           fn default() -> Self;
       }
       // Rust 中定义静态函数未使用 static 关键字，因其把 self 参数显式在参数列表

5.3 扩展方法    // coherence[kəʊˈhɪərəns]n.一致(性)，相干性
    a. 遵循一致性规则(Coherence Rule) 或 孤儿规则(Orphan Rule)
    b. 因 trait 在编译期没有固定大小(具体实现的类型大小亦不固定)故不能直接使用 trait
       作为实例"变量、参数、返回值"的类型(区别于其它语言 interface 多态使用)错误示例:
        // Shape trait 不能用作局部变量的类型
        //let x: Sharp = Circle::new();
        // Shape trait 不能直接用作参数的类型
        //fn use_shape(arg: Sharp) {};
        // Shape trait 不能直接用作返回值的类型
        //fn ret_shape() -> Sharp {}

5.4 完整函数调用语法
    a. 格式" <T as TraitName>::item " // 通用无歧义函数调用语法
    b. 使用：函数名使用完整 path 指定同时 self 参数需要显式传递
       "<TraitName>::method(instance)"/"<ImplStruct as TraitName>::method(instance)"
    c. "成员方法"是首参数变量名为 self 且与 Self 类型相关的"普通函数"，可通过点号"."调用
        可视为一种语法糖(即方法是函数的特例：两者没有本质区别)

5.5 trait 约束和继承
    a. trait 约束写法
       (0). 冒号后添加 trait 名作为约束如
            fn my_print<T:Debug>(x:T){
                println!("The value is {:?} ", x);
            }
       (1). 使用 where 子句如：
            fn my_print<T>(x:T)
                where T: Debug
            {
                println!("The value is {:?} ", x);
            }
        // 注：一般情况上述两种写法皆可，但某些复杂情况泛型约束只有 where 子句可以表达
               如"涉及关联的类型"
    b. trait 继承(p58~59[*])
       trait Base {}
       // Derived trait 继承 Base trait
       trait Derived : Base {}    // 等同于" trait Derived where Self : Base {} "
       // " trait Derived : Base {} "与" trait Derived where Self : Base {} "没有
          本质区别，都是给 Derived trait 追加约束条件(即实现 Derived trait 的类型亦
          必须满足 Base trait 约束)

5.6 Derive
   Rust 中为某些类型实现特定 trait 时单调重复，为此 Rust 提供特殊 attribute 自动实现
   指定的 trait 。
   如" #[derive(Copy,Clone,Default,Debug,Hash,PartialEq,Eq,PartialOrd,Ord] "等则
   编译器会自动实现相应 trait 即" impl trait for StructName "

5.7 trait 别名(alias)
    类似 type alias，可对较为复杂(如携带一系列关联类型)的 trait 起别名

5.8 标准库常见的 trait

5.8.1 Display 和 Debug
    a. 实现 Display trait 的类型才能使用" {} "格式控制打印。对于所有实现了 Display
       trait 的类型都自动实现 ToString trait (其包含" to_string(&self)->String "
       即对于任何实现了 Display trait 的类型皆可对其调用" to_string() "方法格式化
       出一个字符串
    b. 实现 Debug trait 的类型才能使用" {:?} / {:#?} ”格式控制打印

5.8.2 PartialOrd/Ord/PartialEq/Eq
    a. 集合 X 中的元素特性
        (0). 反对称性
        (1). 传递性
        (2). 完全性：若集合中的元素都存在 a<b、a>b 或 a==b 三者必居其一则具完全性
    a. 偏序与全序：主要区别于是否具有完全性如浮点数为偏序(p62[*])
    b. Rust 设计" std :: cmp :: PartialOrd / Ord trait "分别表示" 偏序 / 全序 "

5.8.3 Sized trait
    Sized trait 定义于 std::marker 模块且没有任何成员方法但有 #[lang = "sized"]
    属性，说明其不同于普通 trait : 编译器对它有特殊处理，用户亦不能自定义实现 impl
    此 trait。 一个类型是否满足 Sized (确定大小)约束完全由编译器推导(用户无权指定)

5.8.4 Default trait
    a. Rust 中并没有"构造函数"概念，因相较于普通函数，构造函数本身并没有提供什么额外的
       抽象能力故 Rust 推荐使用普通的静态函数作为类型的构造器。如" fn new()-> String "
    b. 对于无参数、无错误处理的简单情况，标准库提供了 Default trait 来做这个统一抽象。
       其签名为" trait Default { fn default->Self; } "，其内仅包含一个" 静态函数 "
       default() 返回 Self 类型。标准库中很多类型都实现了这个 trait ，它相当于提供了
       一个类型的默认值。

*/