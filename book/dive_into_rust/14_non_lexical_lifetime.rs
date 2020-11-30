
// lexic ['leksik]adj.词汇的[等价于" lexical "]
// lexical [ˈleksɪkl]adj.词汇/词法(的)
/*
0. NLL(Non-Lexical-Lifetime)
   NLL : [自]非词法作用域

14.1 NLL 希望解决的问题

14.2 NLL 的原理
    a. NLL 设计目的是让" 借用 "的生命周期不要过长(适可而止)，避免不必要的编译
       错误(把实际上正确的代码也一起拒绝掉)
    b. " #[feature(nll)} "

*/