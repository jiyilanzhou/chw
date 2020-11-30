
/*
										目 录
第1章 入门指南 1
	安装 1
        在Linux或macOS环境中安装Rust 2
        在Windows环境中安装Rust 3
        更新与卸载 4
        常见问题 4
        本地文档 4
	Hello， World! 5
        创建一个文件夹 5
        编写并运行一个Rust程序 6
        对这个程序的剖析 7
        编译与运行是两个不同的步骤 8
	Hello， Cargo! 10
        使用Cargo创建一个项目 10
        使用Cargo构建和运行项目 13
        以Release模式进行构建 15
        学会习惯Cargo 15
	总结 16

第2章 编写一个猜数游戏 17
	创建一个新的项目 18
	处理一次猜测 19
        使用变量来存储值 20
        使用Result类型来处理可能失败的情况 22
        通过println!中的占位符输出对应的值 24
        尝试运行代码 24
	生成一个保密数字 25
        借助包来获得更多功能 25
        生成一个随机数 28
	比较猜测数字与保密数字 31
	使用循环来实现多次猜测 35
        在猜测成功时优雅地退出 37
        处理非法输入 38
	总结 40

第3章 通用编程概念 42
	变量与可变性 43
        变量与常量之间的不同 46
        隐藏 47
	数据类型 48
        标量类型 49
        复合类型 54
	函数 58
        函数参数 60
        函数体中的语句和表达式 61
        函数的返回值 63
	注释 66
	控制流 67
        if表达式 67
        使用循环重复执行代码 72
	总结 78

第4章 认识所有权 79
	什么是所有权 79
        所有权规则 82
        变量作用域 82
        String类型 83
        内存与分配 84
        所有权与函数 91
        返回值与作用域 92
	引用与借用 94
        可变引用 96
        悬垂引用 99
        引用的规则 101
	切片 101
        字符串切片 104
        其他类型的切片 109
	总结 109

第5章 使用结构体来组织相关联的数据 111
	定义并实例化结构体 112
        在变量名与字段名相同时使用简化版的字段初始化方法 114
        使用结构体更新语法根据其他实例创建新实例 114
        使用不需要对字段命名的元组结构体来创建不同的类型 115
        没有任何字段的空结构体 116
	一个使用结构体的示例程序 118
        使用元组来重构代码 119
        使用结构体来重构代码：增加有意义的描述信息 120
        通过派生trait增加实用功能 121
	方法 124
        定义方法 124
        带有更多参数的方法 127
        关联函数 128
        多个impl块 129
	总结 129

第6章 枚举与模式匹配 130
	定义枚举 131
        枚举值 131
        Option枚举及其在空值处理方面的优势 136
	控制流运算符match 140
        绑定值的模式 142
        匹配Option＜T＞ 143
        匹配必须穷举所有的可能 145
        通配符 146
	简单控制流if let 146
	总结 148

第7章 使用包、单元包及模块来管理日渐复杂的项目 150
	包与单元包 152
	通过定义模块来控制作用域及私有性 153
	用于在模块树中指明条目的路径 156
        使用pub关键字来暴露路径 159
        使用super关键字开始构造相对路径 161
        将结构体或枚举声明为公共的 162
	用use关键字将路径导入作用域 165
        创建use路径时的惯用模式 166
        使用as关键字来提供新的名称 168
        使用pub use重导出名称 169
        使用外部包 170
        使用嵌套的路径来清理众多use语句 171
        通配符 172
	将模块拆分为不同的文件 172
	总结 174

第8章 通用集合类型 175
	使用动态数组存储多个值 176
        创建动态数组 176
        更新动态数组 177
        销毁动态数组时也会销毁其中的元素 177
        读取动态数组中的元素 178
        遍历动态数组中的值 181
        使用枚举来存储多个类型的值 181
	使用字符串存储UTF-8编码的文本 183
        字符串是什么 183
        创建一个新的字符串 184
        更新字符串 185
        字符串索引 188
        字符串切片 191
        遍历字符串的方法 192
        字符串的确没那么简单 193
	在哈希映射中存储键值对 193
        创建一个新的哈希映射 194
        哈希映射与所有权 195
        访问哈希映射中的值 196
        更新哈希映射 197
        哈希函数 199
	总结 200

第9章 错误处理 201
	不可恢复错误与panic! 202
	    使用panic!产生的回溯信息 203
	可恢复错误与Result 207
        匹配不同的错误 210
        失败时触发panic的快捷方式：unwrap和expect 212
        传播错误 213
	要不要使用panic! 219
        示例、原型和测试 220
        当你比编译器拥有更多信息时 220
        错误处理的指导原则 221
        创建自定义类型来进行有效性验证 222
	总结 225

第10章 泛型、trait与生命周期 226
	通过将代码提取为函数来减少重复工作 227
	泛型数据类型 230
        在函数定义中 230
        在结构体定义中 234
        在枚举定义中 236
        在方法定义中 237
        泛型代码的性能问题 239
	trait：定义共享行为 241
        定义trait 241
        为类型实现trait 242
        默认实现 245
        使用trait作为参数 247
        返回实现了trait的类型 249
        使用trait约束来修复largest函数 251
        使用trait约束来有条件地实现方法 254
	使用生命周期保证引用的有效性 256
        使用生命周期来避免悬垂引用 256
        借用检查器 257
        函数中的泛型生命周期 259
        生命周期标注语法 260
        函数签名中的生命周期标注 261
        深入理解生命周期 264
        结构体定义中的生命周期标注 266
        生命周期省略 267
        方法定义中的生命周期标注 270
        静态生命周期 271
	同时使用泛型参数、trait约束与生命周期 272
	总结 273

第11章 编写自动化测试 274
	如何编写测试 275
        测试函数的构成 275
        使用assert!宏检查结果 280
        使用assert_eq!宏和assert_ne!宏判断相等性 284
        添加自定义的错误提示信息 287
        使用should_panic检查panic 289
        使用Result＜T， E＞编写测试 294
	控制测试的运行方式 295
        并行或串行地进行测试 296
        显示函数输出 296
        只运行部分特定名称的测试 299
        通过显式指定来忽略某些测试 301
	测试的组织结构 303
        单元测试 303
        集成测试 305
	总结 311

第12章 I/O项目：编写一个命令行程序 312
	接收命令行参数 313
        读取参数值 314
        将参数值存入变量 316
	读取文件 317
	重构代码以增强模块化程度和错误处理能力 319
        二进制项目的关注点分离 320
        修复错误处理逻辑 325
        从main中分离逻辑 330
        将代码分离为独立的代码包 333
	使用测试驱动开发来编写库功能 335
        编写一个会失败的测试 336
        编写可以通过测试的代码 339
	处理环境变量 343
        为不区分大小写的search函数编写一个会失败的测试 343
        实现search_case_insensitive函数 345
	将错误提示信息打印到标准错误而不是标准输出 349
        确认错误被写到了哪里 350
        将错误提示信息打印到标准错误 351
	总结 352

第13章 函数式语言特性：迭代器与闭包 353
	闭包：能够捕获环境的匿名函数 354
        使用闭包来创建抽象化的程序行为 354
        闭包的类型推断和类型标注 361
        使用泛型参数和Fn trait来存储闭包 363
        Cacher实现的局限性 367
        使用闭包捕获上下文环境 368
	使用迭代器处理元素序列 371
        Iterator trait和next方法 373
        消耗迭代器的方法 374
        生成其他迭代器的方法 375
        使用闭包捕获环境 376
        使用Iterator trait来创建自定义迭代器 378
	改进I/O项目 381
        使用迭代器代替clone 381
        使用迭代器适配器让代码更加清晰 385
	比较循环和迭代器的性能 386
	总结 388

第14章 进一步认识Cargo及crates．io 390
	使用发布配置来定制构建 391
	将包发布到crates．io上 392
        编写有用的文档注释 393
        使用pub use来导出合适的公共API 397
        创建crates．io账户 401
        为包添加元数据 401
        发布到crates．io 403
        发布已有包的新版本 404
        使用cargo yank命令从cargo．io上移除版本 404
	Cargo工作空间 405
        创建工作空间 405
        在工作空间中创建第二个包 407
	使用cargo install从crates．io上安装可执行程序 413
	使用自定义命令扩展Cargo的功能 414
	总结 414

第15章 智能指针 415
	使用Box＜T＞在堆上分配数据 417
        使用Box＜T＞在堆上存储数据 417
        使用装箱定义递归类型 418
	通过Deref trait将智能指针视作常规引用 423
        使用解引用运算符跳转到指针指向的值 424
        把Box＜T＞当成引用来操作 425
        定义我们自己的智能指针 426
        通过实现Deref trait来将类型视作引用 427
        函数和方法的隐式解引用转换 428
        解引用转换与可变性 430
	借助Drop trait在清理时运行代码 431
	    使用std：：mem：：drop提前丢弃值 433
	基于引用计数的智能指针Rc＜T＞ 435
        使用Rc＜T＞共享数据 436
        克隆Rc＜T＞会增加引用计数 439
	RefCell＜T＞和内部可变性模式 440
        使用RefCell＜T＞在运行时检查借用规则 441
        内部可变性：可变地借用一个不可变的值 442
        将Rc＜T＞和RefCell＜T＞结合使用来实现一个拥有多重所有权的可变数据 450
	循环引用会造成内存泄漏 452
        创建循环引用 453
        使用Weak＜T＞代替Rc＜T＞来避免循环引用 456
	总结 463

第16章 无畏并发 464
	使用线程同时运行代码 466
        使用spawn创建新线程 467
        使用join句柄等待所有线程结束 469
        在线程中使用move闭包 471
	使用消息传递在线程间转移数据 475
        通道和所有权转移 478
        发送多个值并观察接收者的等待过程 480
        通过克隆发送者创建多个生产者 481
	共享状态的并发 483
        互斥体一次只允许一个线程访问数据 484
        RefCell＜T＞/Rc＜T＞和Mutex＜T＞/Arc＜T＞之间的相似性 493
	使用Sync trait和Send trait对并发进行扩展 494
        允许线程间转移所有权的Send trait 494
        允许多线程同时访问的Sync trait 495
        手动实现Send和Sync是不安全的 495
	总结 495

第17章 Rust的面向对象编程特性 497
	面向对象语言的特性 497
        对象包含数据和行为 498
        封装实现细节 498
        作为类型系统和代码共享机制的继承 500
	使用trait对象来存储不同类型的值 502
        为共有行为定义一个trait 503
        实现trait 505
        trait对象会执行动态派发 509
        trait对象必须保证对象安全 510
	实现一种面向对象的设计模式 511
        定义Post并新建一个处于草稿状态下的新实例 513
        存储文章内容的文本 514
        确保草稿的可读内容为空 515
        请求审批文章并改变其状态 516
        添加approve方法来改变content的行为 518
        状态模式的权衡取舍 521
	总结 527

第18章 模式匹配 529
	所有可以使用模式的场合 530
        match分支 530
        if let条件表达式 531
        while let条件循环 533
        for循环 533
        let语句 534
        函数的参数 536
	可失败性：模式是否会匹配失败 537
	模式语法 539
        匹配字面量 539
        匹配命名变量 540
        多重模式 541
        使用．．．来匹配值区间 542
        使用解构来分解值 543
        忽略模式中的值 548
        使用匹配守卫添加额外条件 554
        @绑定 556
	总结 557

第19章 高级特性 559
	不安全Rust 560
        不安全超能力 561
        解引用裸指针 562
        调用不安全函数或方法 564
        访问或修改一个可变静态变量 570
        实现不安全trait 572
        使用不安全代码的时机 573
	高级trait 573
        在trait的定义中使用关联类型指定占位类型 573
        默认泛型参数和运算符重载 575
        用于消除歧义的完全限定语法：调用相同名称的方法 578
        用于在trait中附带另外一个trait功能的超trait 582
        使用newtype模式在外部类型上实现外部trait 585
	高级类型 586
        使用newtype模式实现类型安全与抽象 587
        使用类型别名创建同义类型 587
        永不返回的Never类型 590
        动态大小类型和Sized trait 593
	高级函数与闭包 595
        函数指针 595
        返回闭包 598
	宏 599
        宏与函数之间的差别 599
        用于通用元编程的macro_rules!声明宏 600
        基于属性创建代码的过程宏 603
        如何编写一个自定义derive宏 604
        属性宏 611
        函数宏 611
	总结 612

第20章 最后的项目：构建多线程Web服务器 613
	构建单线程Web服务器 614
        监听TCP连接 615
        读取请求 617
        仔细观察HTTP请求 620
        编写响应 621
        返回真正的HTML 622
        验证请求有效性并选择性地响应 624
        少许重构 626
	把单线程服务器修改为多线程服务器 628
        在现有的服务器实现中模拟一个慢请求 628
        使用线程池改进吞吐量 629
	优雅地停机与清理 652
        为ThreadPool实现Drop trait 652
        通知线程停止监听任务 655
	总结 661

附录A 关键字 662
	当前正在使用的关键字 662
	将来可能会使用的保留关键字 664
	原始标识符 665

附录B 运算符和符号 667
	运算符 667
	非运算符符号 669

附录C 可派生 trait  673
	面向程序员格式化输出的 Debug  674
	用于相等性比较的 PartialEq 和 Eq  675
	使用 PartialOrd 和 Ord 进行次序比较 675
	使用Clone和Copy复制值  676
	用于将值映射到另外一个长度固定的值的Hash 677
	用于提供默认值的 Default  678

附录D 有用的开发工具  679
	使用 rustfmt 自动格式化代码  679
	使用 rustfix 修复代码  680
	使用 Clippy 完成更多的代码分析  681
	使用 Rust 语言服务器来集成IDE 683

附录E 版本 684

*/