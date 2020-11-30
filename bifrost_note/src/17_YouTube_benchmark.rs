
/*
0. Concept
    a. Weight is a representation of block execution time
       10^12 Weight = 1 Second 即 1000 Weight = 1 nanosecond
    b. 可以使用一些额外的零即使用 Weight 0 类似于亚纳秒级测量
       输入内容以便为它们多加 1000 这就是为什么只有 10 ~ 12 点而非 10 ~ 9 的原因
    c. 6 秒产生一个块，但其它部分会阻止同步过程，例如结点上有一个 6 秒钟的块，并分配 6 秒钟的 2 秒用于实际
       进行外部交易(图"1_Blcck_Import_Weight_Breakdown.png")，其它用于准备)，但这些都是可以调节的
    d. Weight is specific to each blockchain (图"2_Weight is specific to each blockchain.png")
       不同计算机 1s 内计算的数量不同

1. benchmark 的作用
    a. 主要目标：基准测试的上的是确保运行时会很安全(无论何种情况)
    b. 次要目标：提供一堆程序及优化工具，以便可以优化时间且降低它的，从而最大程度地提高吞吐量
    c. 基准测试实际上是在衡量实际的时间

2. benchmarks! 宏
    benchmarks!{
        extrinisic_name{
            /* setup initial state */
        }:_{ /* execute extrinsic or function */ }
        verify{
            /* verify final state */
        }
    }
    // 在最坏的情况下执行任务
    // 一旦将这些时间安排构建到框架中，就是"线性回归分析"(通过数千个数据点输入正确的时间安排)


 */

/*
3. 每次运行基准测试都会经历的 10 个步骤(图"1_benchmark 10 steps.png")：
    a. 基准测试实际上是使用内存数据库运行的，计算数据库读取和写入的数量，故使用基准测试中的一种
       数据库覆盖(实际上并没有写到数据库而是内存数据库)
    b. 其中有些与读写按键相关的权重，在一起将是链条的立总重量
    c. 基准将是未初始化和私有的，基本上会将存储项目列入白名单，因为它实际上从未获得，读或写本质
       上只是一种短暂的变化，所以会进行基准测试，只是针对该基准而非针对整个基准
    d. 权重公式: " time =  Ax + By + Cz + K "给定输入，可告知需要多长时间都能承受(即为
       功能分配权重[图"2_time.png"])
    e. 当最初尝试弄清楚一个函数的权重，其中一个规则是看不懂存储，因为读取存储非常昂贵
    f. 加权函数权重必须基本精简，只是计算而已。因此为了获得该函数的初始权重，我们无法知道有多少
       个注册服务商，有多少个子账户，该账户拥有的所有内容都必须读入存储，因此假设最糟糕的情况是
       当结束时实际上杀死了身份
    g. 隔离数据库的基准测试，故最初使用黑匣子测量，包括数据库在内的所有内容都包裹在其中
    h. 想象一下，有一个区块链，其外部时间大于数据库读写时间，仅根据选择数据库的性质以及想使用的
       CPU 信息，a 路径是最昂贵的，但翻转后可能 b 路径实际上是更昂贵的路径。因此难以对区块链路径
       做出假设，因此作为开发人员，需要对所有路径进行基准测试，然后据权重公式选择最昂贵的
    f. 基准测试是一项安全测试

4. 第一件事实际上是将所有已知的数据库密钥列入白名单
       Whitelist known DB keys

5. Examplel Benchmark (The Identity Pallet)  [视频时间:"35:38"]
    a. 添加 benchmarking.rs
       [视频时间:" 2:09:20 ~ 2:14:22 / 2:20:00 ~ 2:31:48 / 2:34:20 ~ 3:14:38  ]
    b. benchmarks! {
            _ { }

            benchmark_for_func {
                ...
            }:func(param) // 当 func 与 benchmark_for_func 同名时 func 可用" _ "替代
            verify{
                ...
            }
    c. 只有在 ..runtime/Cargo.toml 内加载" pallet_xxx/runtime-benchmarks "才会加载
       相应模块

6. benchmark 基准测试与 WeightInfo 编写
    a. 先进行 benchmarking 基准测试编写，执行命令自动生成相应的实现 WeightInfo 的文件
    b. 基于 benchmarking 生成的实现文件再进行 WeightInfo 的编写


 */