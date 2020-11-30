
// include[ɪnˈkluːd]v.包含,包括         // inclusion[ɪnˈkluːʒn]n.包含
/*
0. Transaction Fees 交易费用

1. Fee Calculation 费用计算
    a. Inclusion Fee  入会费
    b. Fee Multiplier 费用乘数
    c. Additional Fees 附加费用

2. Default Weight Annotations  默认重量注释
    Substrate 中的所有可调度功能必须指定权重

3. Parameterizing over Database Accesses  通过数据库访问进行参数化

4. Dispatch Classes  派遣班
    // 急件被分成三类：Normal，Operational 和 Mandatory
    a. Normal Dispatches 正常派送
    b. Operational Dispatches 运营调度
    c. Mandatory Dispatches 强制派遣

5. Dynamic Weights 动态权重
    除了纯粹固定的权重和常量之外，权重计算还可以考虑可调度对象的输入参数

6. Post Dispatch Weight Correction 调度后权重校正
    根据执行逻辑，可调度项可能消耗的权重比预定的预调度要少

7. Custom Fees 自定义费用

8. Custom Weights  自定义权重

9. Custom Inclusion Fee  自定义收录费用

 */