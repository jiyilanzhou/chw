
// telemetry[təˈlemətri]n.遥测(遥感)    // digest[dɪˈdʒest]v&n.消化(吸收),文摘
// equivocate[ɪˈkwɪvəkeɪt]v.模棱两可,含糊其辞   // deposit[dɪˈpɒzɪt]n.存款,存放
// extrinsic[eksˈtrɪnsɪk]adj.外在的,外来的    // hybrid[ˈhaɪbrɪd]n.混合,杂种
// stake[steɪk]n.赌注     // shaft[ˈɪərə]n.轴      // era[ˈɪərə]n.时代,纪元
/*
0. Overview 总览
    Substrate 是一个区块链开发框架，具有完全通用的状态转换功能(State Transition Funtion,STF)
    和用于共识，网络和配置的模块化组件

1. Architecture (图" 0_substrate客户端组件.png ")
    a. 存储：达成不信任共识
    b. 运行时：无叉运行时升级
    c. 点对点网络：libp2p 网络协议栈
    d. 共识
    e. RPC

2. Usage 用法(图" 1_substrate用法.png ")
    a. 使用 Substrate Node ：提供最少自定义量
    b. 使用 Substrate Frame ：配置数据类型、自定义选择 pallet
    c. 使用 Substrate Core : 可忽略整个 Frame 系统，可从头开始自定义设计和实现运行时

*/
