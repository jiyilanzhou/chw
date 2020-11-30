# Bifrost  多结点升级(此例以两个结点为例) 



## 1. 准备工作  

- ### 1.1 拉取代码

```http
https://github.com/bifrost-finance/bifrost
```

- 	   ### 1.2 进入" bifrost "目录		

```
 执行" cargo build --release "编译命令
```

- 	   ### 1.3 清除缓存数据 

```shell
# clear cache
rm -rf /tmp/alice/ \
/tmp/bob/ 
```

- 			### 1.4  在" bifrost "目录下启动 alice 及 bob 结点

```shell
# 在 Terminal 启动 alice 结点
./target/release/bifrost-node --alice \
--chain local \
--base-path /tmp/alice \
--port 30333 \
--validator

# 另打开一个 Terminal 窗口启动 bob 结点
./target/release/bifrost-node --bob \
--chain local \
--base-path /tmp/bob \
--port 30334 \
--validator
```

- ### 1.5 访问前端

     ##### 1.5.1  访问 <https://polkadot.js.org/apps/#/explorer> 并设置为本地结点如下图

![](.\img\0.png)

#####      1.5.2  此时若发现前端页面无法加载节点的数据，则需把 developer_setting.json 文件内容拷贝至下图位置

![](.\img\1.png)



## 2. (针对开发)添加 pallet 模块(以 practice pallet 为例)

- ### 2.1 编辑并添加 pallet 模块(此例添加 practice )

  ```javascript
  // 若新添加的 pallet 有新增类型则需在" bifrost\docs\developer_setting.json "中增加相应类型如
  {
    "Token": {
      "symbol": "Vec<u8>",
      "precision": "u16",
      "totalSupply": "u128"
    },
    "VersionId": "u32",
    "Action": {
      "account": "AccountName",
      "name": "ActionName",
      "authorization": "Vec<PermissionLevel>",
      "data": "Vec<u8>"
    },
    // 追加新增 pallet 中自定义类型
  }
  ```
  

  
- ### 2.2 将" bifrost\bin\node\runtime\src\lib.rs "内" spec_version "的版本号加一 如：

```rust
// 部分代码
/// Runtime version.
pub const VERSION: RuntimeVersion = RuntimeVersion {
	spec_name: create_runtime_str!("bifrost"),
	impl_name: create_runtime_str!("bifrost-node"),
	authoring_version: 1,
	// Per convention: if the runtime behavior changes, increment spec_version
	// and set impl_version to 0. If only runtime
	// implementation changes and behavior does not, then leave spec_version as
	// is and increment impl_version.
    // 添加 pallet 后更新版本号(一般为"上一版本号加一")
	spec_version: 15,		
	impl_version: 1,
	apis: RUNTIME_API_VERSIONS,
	transaction_version: 1,
};
```

- ####　2.3 进入 " bifrost "目录　

```shell
 再次执行" cargo build --release "命令重新编译
```



## 3. 升级(" Scheduler "配合" set_code "方式升级)

#### 3.1 访问前端并调出控制台(观察升级时的日志信息)

![](.\img\2.png)

#### 3.2 如果升级成功则在到达预设目标块高度时更新(如此例会由 version " 14 "变为" 15 ")

- 升级成功后刷新页面即可展示新加入的模块，并测试新加入的模块功能

#### 3.3 测试结点间的互通性

- 停止原 bifrost-node 之一结点(如停止 bob 结点)，并以新编译生成的 bifrost-node 启动对应结点(如 bob 结点)，观察并测试模块功能
- 停止原 bifrost-node 另一结点(如停止 alice结点)，并以新编译生成的 bifrost-node 启动对应结点(如 alice结点)，观察并测试模块功能
- 如若上述一切正常则升级成功！

