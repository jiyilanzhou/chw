
// frame[freɪm]n.框架(结构),帧
/*
0. 框架 FRAME
    托盘是 FRAME 中的单个模块，承载特定于域的逻辑

1. 总监 Overview
    (图" 1_FRAME及其支持库的体系结构.png ")

2. 货盘 Pallets
    使用 FRAME 进行构建时，Substrate 运行时由几个较小的组件(称为托盘)组成
    (图"0_frame通过在primitives实现特征来与client交互.png")

3. 系统库 System Library
    a. 该系统库为定制的 blockchain 提供低级别的类型，存储和功能。其余 pallet
       皆依赖于系统库。
    b. 系统库定义了 Substrate 运行时的所有核心类型
    c. 系统库还具有许多对系统至关重要的存储项目

4. 执行模块 Executive Module

5. 运行
    运行时库将所有这些组件和托盘组合在一起



 */