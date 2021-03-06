
/*
0. 使用" 包、单元包 及 模块 "来管理日渐复杂的项目
    a. 一个包(package)可以拥有多个二进制单元包及一个可选的库单元包。而随着 包内代码
       规模的增长，可将部分代码拆分到独立的单元包(crate)中，并将其作为外部的依赖进行
       引用。对于那些特别巨大的、拥有多个相关联的包的项目，Cargo 提供了另外一种解决
       方案" 工作空间(workspace) "
    b. 模块系统(module system)
       (0). 包 package ： 一个用于构建、测试并分享单元包的 Cargo 功能
       (1). 单元包 crate：一个用于生成库或可执行文件的树型模块结构
       (2). 模块 module 及 use 关键字：用于控制文件结构、作用域及路径的私有性
       (3). 路径 path ：一种用于命名条目的方法，这些条目包括结构体、函数和模块等

1. 包 与 单元包(p152)
    a. Cargo 会默认将 src/main.rs 视作一个二进制单元包的根结点而无需指定，这个
       二进制包与包拥有相同的名称。同样若包的目录中包含文件 src/lib.rs，Cargo
       也会自动将其视作与包同名的库单元包根节点
    b. 假设包中同时存在 src/main.rs 及 src/lib.rs 则会分别存储在一个二进制单元包
       与一个库单元包，它们拥有与包相同的名称
    c. 可在路径 src/bin 下添加添加源文件来创建出更多的二进制单元包，这个路径下的
       每个源文件都会被视作单独的二进制单元包。单元包可以将相关功能分组并放至同一
       作用域下，这样便可以使这些功能轻松地在多个项目中共享。
    d. 将单元包的功能保留在它们自己的作用域中有助于指明某个特定功能来源于哪个单元包
       并避免可能的命名冲突。如 rand 包提供一个名为 Rng 的 trait，开发者亦在自己
       的单元包中定义同名为 Rng 的 struct，正是因为这些功能被放置在各自的作用域中，
       当引用 rand 依赖时编译器才不会为某个 Rng 的具体含义而困惑，即在开发单元包中
       直接使用 Rng 则其指向内部定义的 Rng struct，若欲访问 rand 包中的 Rng trait
       则可通过" rand::Rng "指定

2. 通过定义模块来控制作用域及私有性(p153[*])
    a. 关键字：将路径引入作用域的 use 关键字，将条目标记为公开的 pub 关键字等
    b. 模块允许将单元包内的代码按照可读性与易用性来进行分组，同时还允许控制条目的
       私有性。换言之即是模块决定了一个条目是否可以被外部代码使用(公共)或仅仅只是
       一个内部的实现细节而不对外暴露(私有)
    c. 以 mod 关键字开头定义一个模块。通过使用模块可将相关的定义分为一组并根据其
       关系指定有意义的名称，开发者可轻松地在此类代码中找到某个定义(因为可据分组
       进行搜索[而无需遍历所有定义])。开发者亦可把新功能的代码按这些模块进行划分
       并放入其中，从而保持程序的组织结构不变
    d. src/main.rs 及 src/lib.rs 被称作单元包的根节点，因为这两个文件的内容各自
       组成了一个名为 crate 的模块，并位于单元包模块结构的底部，这个模块结构也被
       称为模块树(module tree)
    e. 当模块 A 包含在模块 B 内时，将模块 A 称作模块 B 的子节点(child)，并将模块
        B 称作模块 A 的父节点(parent)，注意整个模块树都被放置在一个名为 crate 的
        隐式根模块下
    f. "模块树"类比"文件系统目录树",正如文件系统中的目录一样可使用模块来组织代码
        同时也需要对应的方法来定位模块

3. 用于在模块树中指明条目的路径(p156[*])
    a. 类似于文件系统中使用路径进行导航的方式，为在 Rust 的模块树中找到某个条目
       同样需要使用路径(如调用某个函数时需要知晓其路径)
    b. 路径有两种形式: 绝对路径 及 相对路径
       (0). 绝对路径：使用单元包或字面量 crate 从根节点开始
       (1). 相对路径：使用 self、super 或内部标识符从当前模块开始
       // 绝对路径或相对路径都由至少一个标识符组成，标识符间使用双冒号(::)分隔
    c. 使用绝对路径还是相对路径基于项目的实际情况，通常取决于是否会移动条目的定义
       代码及使用该条目的代码。如将代码及其相关联的模块同时移至新模块则需更新绝对
       路径而相对路径依然有效；若单独将使用模块的代码移至新模块则需要更新相对路径
       而绝对路径会保持不变。
    d. 模块不仅仅用于组织代码，同时还定义了 Rust 中的私有边界(privacy boundary):
       外部代码无法知晓、调用或依赖那些由私有边界封装了的实现细节，因此想要将一个
       条目(如函数或结构体)声明为私有时可将其放置到某个模块中
    e. Rust 中的所有条目(函数、方法、结构体、枚举、模块及常量)默认都是私有的。处于
       父级模块中的条目无法使用子模块中的私有条目，但子模块中的条目可以使用其所有
       祖先模块中的条目。虽然子模块包装并隐藏了自身的实现细节，但它却依然能够感知
       当前定义环境的上下文。
    f. Rust 模块系统默认隐藏内部的实现细节，如此能够明确知道修改哪些内部实现不会
       破坏外部代码。同时可使用 pub 关键字来将某些条目标记为公共的，从而使子模块
       中的这些部分被暴露到祖先模块中

4. 使用 pub 关键字来暴露路径
    a. 将模块变为公开状态并不会影响到其内部条目的状态，模块前的 pub 关键字仅意味着
       祖先模块拥有了指向该模块的权限。私有性规则不仅作用于模块，也同样作用于函数、
       方法、结构体及枚举。
    b. 相同模块下的同级节点可相互访问(虽然同级节点未声明为公共的 pub )，同理相对
       路径亦是从相同模块下开始定位

5. 使用 super 关键字开始构建相对路径(p161[*])
    同样亦可从父模块开始构造相对路径，这一方式需要在路径起始处使用 super 关键字。
    其有些类似在文件系统中使用" ../ "语法定位父级目录。同理，相对路径主要是避免
    代码及其关联的模块共同移动到新模块时的更新。

6. 将结构体或枚举声明为公共的
    a. 结构体与枚举皆可使用 pub 来声明为公共的但存在细微差别
    b. 在结构体前使用 pub 时结构体本身就成了公共的，但其字段依旧保持了私有状态，
       可逐一决定是否将某个字段公开
    c. 若公开结构体存在未公开的字段，则结构体需要提供公共的关联函数来构造实例(
       若缺少这样的关联函数则无法在其它模块创建或获取结构体实例[结构体实例化须
       实例化所有字段：因 Rust 不存在默认零值而其又存在私有字段故无法创建])
    d. [自](结构体字段为属性：遵循了默认的私有性规则)而枚举变体是值，只有在公共
       可用时才能实现最大功效，而必须为所有枚举变体添加 pub 则显得烦琐，故枚举
       所有变体都默认为公共的。

7. 用 use 关键字将路径导入作用域(p165[*])    // (类比 Golang 中的" import ")
    a. 基于路径(绝对路径、相对路径)来调用的写法显得有些重复与冗长，可借助 use
       关键字来将路径引入作用域后像使用本地条目一样来调用路径中的条目
    b. 在作用域中使用 use 引入路径有些类似于在文件系统中创建符号链接
    c. 使用 use 来指定相对路径稍微有一些不同，可在传递给 use 路径的开始处使用
       关键字 self 而不是从当前作用域中可用的名称开始

*/
mod front_of_house{
    pub mod hosting{
        pub fn add_to_waitlist(){}
    }
}
// self 关键字可省
//use self::front_of_house::hosting;
use front_of_house::hosting;
pub fn eat_at_restanrant(){
    hosting::add_to_waitlist();
}

/*
8. 创建 use 路径时的惯用模式(p168[*])
    a. 一般使用 use 将条目(函数、方法、结构体、枚举、模块及常量等)的父模块引入
       作用域(简洁且可区分其下的同名条目)
    b. 使用父模块将拥有相同名称的类型引入作用域(即使用父模块区分其下的同名条目)
       如将" use std::fmt;"及" use std::io;"不同的父模块引入作用域，然后使用
       " fmt::Result "和" io::Result "区分不同的 Result 类型

9. 使用 as 关键字来提供新的名称
    除引入父模块来解决同名类型引入作用域冲突外，亦可使用在路径后使用 as 关键字
    为类型指定新的本地名称即" 别名 "如：
    use std::fmt::Result;
    use std::io::Result as IoResult; // 使用"IoResult"别名避免"Result"冲突

10. 使用 pub use 重导出名称(p169)
    a. 当使用 use 关键字将名称引入作用域时，这个名称会以私有的方式在新作用域中
       生效。为使外部能够访问到这些名称，可通过组合使用 pub 与 use 实现。这项
       技术也被称作重导出(re-exporting)，因为不仅将条目引入作用域而且使该条目
       可以被外部代码从新的作用域引入自己的作用域(如" pub use crate::*... ")
    b. 当代码的内部结构与外部所期望的访问结构不同时，重导出技术会显得非常有用。

11. 使用外部包(p170[*])
    a. 先在 Cargo.toml 之 [dependencies] 下添加项目所需外部包依赖，Cargo 会从
       " crate.io "上下载相应的依赖包并使其对当前项目可用。如"Cargo.toml"配置
            [dependencies]
            rand = "0.5.5"
    b. 接着为了将依赖包(rand)引入当前作用域，使用 use 以包名(rand)开始并在其后
       列出想要引入作用域的条目(如" rand::thread_rng ")
    c. Rust 社区成员已经在 crate.io 上上传了许多可用的包，引入项目首先将其列入
       " Cargo.toml "文件，然后使用 use 来将特定条目引入作用域
    d. 标准库(std)实际上也同样被视作当前项目的外部包，由于标准库已经被内置到了
       Rust 语言中故无需特意修改 Cargo.toml 来包含 std，但同样需要用 use 来将
       标准库中特定条目引入当前项目作用域。如"use std::collections::HashMap;"
       (标准库绝对路径以" std "开头：std 是标准库单元包的名称)

12. 使用嵌套的路径来清理众多 use 语句
    如" use std::{cmp::Ordering,io}; / use std::io::{self,Write}; "(子路径)

13. 通配符 *
    a. 通配符可将某个路径下的公共条目都导入作用域如"use std::collections::*;"，
       但请小心谨慎使用通配符这一特性。因为通配符会使得难以确定作用域中存在哪些
       名称以及某名称的具体定义位置。
    b. 测试代码常常会使用通配符将所有需要测试的引入 tests 模块。此外通配符还经常
       用于"预导入模块"

14. 将模块拆分为不同的文件(p173)

*/