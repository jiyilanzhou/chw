
// claim[kleɪm]n.声明,索赔      // revoke[rɪˈvəʊk]v.撤销,撤回,取消
/*
0. Proof of Existence 存证功能模块开发

1. 存证简介
    a. 存证是一种在线服务(在某一时间点验证计算机文件的存在性)，最早是通过
       比特币网络带有时间戳的交易实现的(引入新功能异常困难)。
    b. 存证的应用场景：
        数字版权：如文章、图片的版权
        司法存证：如证据链、判决过程、判决结果(通过上链真正做到公正、无徇私)
        供应链溯源：如日常生活中的粮食、蔬菜质量难以保证(通过区块链源源改善)
        电子发票：将发票上链更容易对传统商业的违规行为监督和审查

2. 代码实现 PoE (参阅" https://substrate.dev/docs/en/tutorials/build-a-dapp/ ")
    a. 下载项目
        git clone -b v2.0.0-rc5 --depth 1 https://github.com/substrate-developer-hub/substrate-node-template
    b. 拷贝" ...\pallets\template "模块并改名为" \pallets\poe "并修改相应的 Cargo.toml
         # 指定生成文档的适用平台(即不再为其它平台编译相应文档[减少文档磁盘占用容量])
         [package.metadata.docs.rs]
         targets = ['x86_64-unknown-linux-gnu']
    c.  " ..\pallets\poe\src "目录下
        (0). lib.rs : 功能文件
        (1). mock.rs ：测试依赖文件
        (2). tests.rs ：测试用例文件(源于拷贝故需注释其内容[避免编译错误])

3. 将编辑好的 poe 模块加入 Runtime
    a. 在" ..\runtime\Cargo.toml "内添加 poe 模块并追加至 [features]
    b. 在 runtime 的功能文件即" ..\runtime\src\lib.rs"内加载 poe 模块
        (0). 追加 poe 的 Runtime 实现
              pub use poe;
              impl poe::Trait for Runtime {
                  type Event = Event;
              }
        (1). 将自定义 pallet 追加到" construct_runtime! "宏
             construct_runtime!(
                 pub enum Runtime where
                     Block = Block,
                     NodeBlock = opaque::Block,
                     UncheckedExtrinsic = UncheckedExtrinsic
                 {
                     //...
                     // 追加自定义 pallet
                     PoeModule: poe::{Module, Call, Storage, Event<T>},
                 }
             );

4. 编译 // 注：［dev-dependencies.sp-core］中 sp 的全称是" substrare/primitive "
    cargo build --release

5. 前端 substrate-front-end-template 项目
    a. 下载
        git clone -b v2.0.0-rc5 --depth 1 https://github.com/substrate-developer-hub/substrate-front-end-template
    b. 进入项目安装前端依赖( yarn install)
        root@ubuntu:~/project/substrate-front-end-template# yarn install
    c. 进入项目启动
        root@ubuntu:~/project/substrate-front-end-template# yarn start
    d. 开发自定义模块
       在" ..\substrate-front-end-template\src "目录编辑自定义JS文件如" PoeModule.js "
    e. ( PoeModule.js 编辑完毕)注册到 App.js 文件如在 App.js 文件内:
       (0). 引入 PoeModule 组件
            mport PoeModule from './PoeModule'
       (1). 在组件列表添加 PoeModule 组件
            <Grid.Row>
              <PoeModule accountPair={accountPair} />
            </Grid.Row>
    f. Js更新后自动实时热备(无需重启即生效)

*/