
// equivalent [ɪˈkwɪvələnt]n.等效   // equivocate[ɪˈkwɪvəkeɪt]v.含糊,模棱两可,双关
// benchmark[ˈbentʃmɑːk]n. 基准(检查程序)
// Dos : denial of service (拒绝服务)

/*
0. benchmark 开发概览
    a. 一组宏（benchmarks!，add_benchmark!等），使编写、测试和添加运行时基准测试变得容易。
       （0). benchmarks! (位于"substrate/frame/benchmarking/src/lib.rs"以及
                             "substrate/frame/pallet/src/benchmarking.rs")
             // 通过在 benchmarks! 宏中创建基准，它会自动生成测试功能:
                " fn test_benchmark_[benchmark_name]<T>::() -> Result<(), &'static str> "
        (1). add_benchmark! (位于" substrate/bin/node/runtime/src/lib.rs ")

    b. 一组线性回归分析函数，用于处理基准数据
        (位于"substrate/frame/benchmarking/src/analysis.rs")

    c. CLI 扩展，可轻松在节点上执行基准测试。

1.(具体开发流程)启用 runtime-benchmarks 功能标记运行测试(以" Balances "为例)
     # 测试 Balances 托盘基准的方法
     cd bin/node/cli
     cargo test -p pallet-balances --features runtime-benchmarks

2. 添加基准测试
   a. 每个托盘随附的基准不会自动添加到节点。要实际执行这些基准测试需要实现 frame_benchmarking::Benchmark 特征如：
      (位置" ../frame/babe/src/benchmarking.rs "[自]当然亦可在"..runtime/src/lib.rs"下实现[但极不推荐])
        use frame_benchmarking::benchmarks;
        benchmarks! { //... }

   b. 若结点上已经设置了一些基准（即相应 pallet 下已编辑完 benchmarking.rs 文件）则只需添加该 add_benchmark! 宏
      的另一个实例(位置" substrate/bin/node/runtime/src/lib.rs ")
        #[cfg(feature = "runtime-benchmarks")]
        impl frame_benchmarking::Benchmark<Block> for Runtime {
            ///  configuration for running benchmarks  用于运行基准测试的配置
            ///               |    name of your pallet's crate (as imported) 托盘的板条箱名称（已导入）
            ///               v                   v
            add_benchmark!(params, batches, pallet_balances, Balances);
            ///                       ^                          ^
            ///    where all benchmark results are saved         |      保存所有基准测试结果|
            ///            the `struct` created for your pallet by `construct_runtime!`
            ///                                         `construct_runtime！为您的货盘创建的`struct`！
        }

3. 使用 runtime-benchmarks feature 标志编译节点二进制文件
    cd bin/node/cli
    cargo build --release --features runtime-benchmarks

4. 运行 benchmark
   a. 获取可用基准的列表
      ./target/release/substrate benchmark --chain dev --pallet "*" --extrinsic "*" --repeat 0
   b. 运行基准测试
        ./target/release/substrate benchmark \
        --chain dev \
        --execution=wasm \
        --wasm-execution=compiled \
        --pallet pallet_balances \
         --extrinsic transfer \         // 注：若基准测试全部则使用 *
        --steps 50 \
        --repeat 20 \
        --output ./generate_file.rs
      # 运行出错：Error: Input("Output path is invalid!")
      # 分析方案：搜索错误定位源码位置(全局搜索"Output path is invalid!")
      # 查询到相应源码：
        if let Some(output_path) = &self.output {
			if !output_path.is_dir() { return Err("Output path is invalid!".into()) };
		}
     # 解决方案：传入目录而非文件即
        chw@chwdeMacBook-Pro substrate % mkdir weights \
        ./target/release/substrate benchmark \
        --chain dev \
        --execution=wasm \
        --wasm-execution=compiled \
        --pallet pallet_balances \
         --extrinsic '*' \
        --steps 50 \
        --repeat 20 \
        --output weights/

*/

/*  benchmark 开发流程(以添加 poe 模块为例[具体未实现])
    #（ project ）substrate 项目配置
    " substrate/Cargo.toml "文件配置：
        [workspace]
        members = [
            // --snip--
            # TODO
            "frame/poe",
            // --snip--
        ]

0. Poe 模块
    a. substrate/frame/poe/Cargo.toml
        [package]
        name = "pallet-poe"		# TODO chw 0.0
        [dependencies]
        # TODO chw 0.1
        frame-benchmarking = { version = "2.0.0", default-features = false,optional = true }
        [features]
        default = ["std"]
        std = [
            # TODO chw 0.2
            "frame-benchmarking/std",
            # --snip--
        ]
        # TODO chw 0.3
        runtime-benchmarks = ["frame-benchmarking"]

    b. substrate/frame/poe/src/benchmarking.rs
       # TODO chw 1.x

    c. substrate/frame/poe/src/lib.rs
       // TODO chw 2.x
       mod benchmarking;

1.Runtime 模块
    a. substrate/bin/node/runtime/Cargo.toml
        # frame dependencies
        # TODO chw 3.0
        pallet-poe = { version = "2.0.0", default-features = false, path = "../../../frame/poe" }

        [features]
        default = ["std"]
        with-tracing = [ "frame-executive/with-tracing" ]
        std = [
            # --snip--
            # TODO chw 3.1
	        "pallet-poe/std",
            # --snip--
        ]

        runtime-benchmarks = [
            # --snip--
            # TODO 3.2
            "pallet-poe/runtime-benchmarks",
            # --snip--
	    ]


    b. substrate/bin/node/runtime/src/lib.rs
        construct_runtime!(
           pub enum Runtime where
              Block = Block,
              NodeBlock = node_primitives::Block,
              UncheckedExtrinsic = UncheckedExtrinsic
          {
              # --snip--
              // TODO chw 4.1
		      Poe: pallet_poe::{Module, Call, Storage, Event<T>},
              # --snip--
          }
       impl_runtime_apis! {
           # --snip--
           #[cfg(feature = "runtime-benchmarks")]
           impl frame_benchmarking::Benchmark<Block> for Runtime {
               fn dispatch_benchmark(
                   config: frame_benchmarking::BenchmarkConfig
               ) -> Result<Vec<frame_benchmarking::BenchmarkBatch>, sp_runtime::RuntimeString> {
                 # --snip--
                 // TODO chw 4.2
			     add_benchmark!(params, batches, pallet_poe, Poe);
                 # --snip--
            }
       }

2. 进入" substrate/bin/node/cli "目录执行带 benchmark 的 build 命令
   a." substrate/bin/node/cli "目录
      // TODO 5.0 # 执行带 runtime-benchmarks 特性的编译命令即
      substrate/bin/node/cli$ cargo build --release --features runtime-benchmarks
          error: Package `node-cli v0.5.0 ...` does not have these features: `runtime-benchmarks`

   b. 解决方案 : 准备及配置依赖工作(" /bin/node/cli/Cargo.toml ")
      (0). # CLI-specific dependencies
            # TODO chw 0
            frame-benchmarking-cli = { version = "2.0.0", optional = true }
      (1). [build-dependencies]
            # TODO chw 1
            frame-benchmarking-cli = { version = "2.0.0", optional = true }
      (2). [features]
            # -- snip --
            cli = [
                "node-inspect",
                "sc-cli",
                "substrate-frame-cli",
                "sc-service/db",
                "structopt",
                "substrate-build-script-utils",
                # TODO chw 2
                "frame-benchmarking-cli",
            ]
       (3). # TODO chw3
            runtime-benchmarks = [
                "node-runtime/runtime-benchmarks",
                "frame-benchmarking-cli",
            ]

   c. 再次执行
      substrate/bin/node/cli$ cargo build --release --features runtime-benchmarks
        error: failed to select a version for `node-runtime`.
            ... required by package `node-cli v0.5.0 (../bin/node/cli)`
            versions that meet the requirements `=0.5.0` are: 0.5.0
            the package `node-cli` depends on `node-runtime`, with features: `runtime-benchmarks`
            but `node-runtime` does not have these features.
            failed to select a version for `node-runtime` which could resolve this conflict
            // 客户端结点 node-cli (即 cli 模块)依赖运行时结点 node-runtime (即 runtime 模块) 的
                 " runtime-benchmarks "属性，但是 runtime 结点没有这个属性

   d. 解决方案：在" ../bin/node/runtime/Cargo.toml "下添加" runtime-benchmarks "属性如：
       // TODO chw 6.x
       runtime-benchmarks = [
            "frame-benchmarking",
            "pallet-babe/runtime-benchmarks",
            "pallet-balances/runtime-benchmarks",
            "pallet-proxy/runtime-benchmarks",
            "pallet-treasury/runtime-benchmarks",
            // -- 追加欲测试的模块 --
       ]

   e." substrate/bin/node/cli "目录
      // TODO  # 执行带 runtime-benchmarks 特性的编译命令即
      substrate/bin/node/cli$ cargo build --release --features runtime-benchmarks
        error[E0432]: unresolved import `pallet_session_benchmarking`
          1252 |             use pallet_session_benchmarking::Module as SessionBench;
               |                 ^^ use of undeclared crate or module `pallet_session_benchmarking`
          error[E0432]: unresolved import `pallet_offences_benchmarking`
          1253 |             use pallet_offences_benchmarking::Module as OffencesBench;
               |                 ^^^use of undeclared crate or module `pallet_offences_benchmarking`
          error[E0432]: unresolved import `frame_system_benchmarking`
          1254 |             use frame_system_benchmarking::Module as SystemBench;
               |                 ^^^use of undeclared crate or module `frame_system_benchmarking`
          error[E0433]: failed to resolve: use of undeclared crate or module `hex_literal`
          1272 |                 hex_literal::hex!("26aa394e8751baa9d281e0bfa3a6d6f646c70792f7472...
               |                 ^^^^^^^^^^^ use of undeclared crate or module `hex_literal`

          对应源码位置：
            #[cfg(feature = "runtime-benchmarks")]
            impl frame_benchmarking::Benchmark<Block> for Runtime {
                fn dispatch_benchmark(
                    config: frame_benchmarking::BenchmarkConfig
                ) -> Result<Vec<frame_benchmarking::BenchmarkBatch>, sp_runtime::RuntimeString> {
                    use frame_benchmarking::{Benchmarking, BenchmarkBatch, add_benchmark, TrackedStorageKey};
                    use pallet_session_benchmarking::Module as SessionBench;
                    use pallet_offences_benchmarking::Module as OffencesBench;
                    use frame_system_benchmarking::Module as SystemBench;

                    impl pallet_session_benchmarking::Trait for Runtime {}
                    impl pallet_offences_benchmarking::Trait for Runtime {}
                    impl frame_system_benchmarking::Trait for Runtime {}

                    let whitelist: Vec<TrackedStorageKey> = vec![
                        // Block Number
                        hex_literal::hex!("26aa394eea5630e...9aca4983ac").to_vec().into(),
                        // --snip--
                    ]
                    /* " Convert "源于 construct_runtime! (
                                            pub enum Runtime where
                                                Block = Block,
                                                NodeBlock = node_primitives::Block,
                                                UncheckedExtrinsic = UncheckedExtrinsic
                                            {
                                                // --snip--
                                                Convert: brml_convert::{Module, Call, Storage, Event, Config<T>},
                                            }
                                       );
                    */
                    add_benchmark!(params, batches, brml_convert, Convert);
                    // --snip--
             }
             // 分析:据" #[cfg(feature = "runtime-benchmarks")] "知其源于" bin/node/runtime/Cargo.toml "
                文件内的" runtime-benchmarks = [ ... ] "而" runtime-benchmarks "又源于同文件内导入的依赖

    f. 解决方案：在" bin/node/runtime/Cargo.toml "之" runtime-benchmarks = [ ... ] "追加相应模块并导入依赖
       # frame dependencies    // 已经存在的无需添加
       # frame-benchmarking = { version = "2.0.0", default-features = false, optional = true }
       # frame-system = { version = "2.0.0", default-features = false }
       # frame-system-benchmarking = { version = "2.0.0", default-features = false, optional = true }
       # 说明" pallet-session "及" pallet-offences "未使用到但其为依赖故必须追加
       pallet-session = { version = "2.0.0", features = ["historical"], default-features = false }
       pallet-session-benchmarking = { version = "2.0.0", default-features = false, optional = true }
       pallet-offences = { version = "2.0.0", default-features = false}
       pallet-offences-benchmarking = { version = "2.0.0", default-features = false, optional = true }

   f. 同理在" substrate/bin/node/cli "目录下测试 benchmarks
      // TODO 5.1 测试相应模块
      substrate/bin/node/cli$ cargo test -p pallet-balances --features runtime-benchmarks

   g. 生成具体实现 WeightInfo trait 的文件
      // TODO 5.2 生成相应文件
      substrate$ mkdir weights                      // 先创建存储生成文件的目录
        ./target/release/substrate benchmark \
        --chain dev \
        --execution=wasm \
        --wasm-execution=compiled \
        --pallet pallet_balances \
         --extrinsic '*' \
        --steps 50 \
        --repeat 20 \
        --output weights/

   // TODO chw 5.3 将生成的相应文件置于" runtime/src/weights "下并于同目录下的 mod.rs 中暴露

3. 将生成的相应文件集成到项目
    a. substrate/frame/poe/src/lib.rs
        (0).// TODO chw 6.0
            use frame_support::{weights::Weight,};
        (2). // TODO chw 6.1
             // 定义 WeightInfo trait
             pub trait WeightInfo {
                // 权重函数名：建议功能函数名一致(见文知义)
                fn create_claim() -> Weight;
                fn revoke_claim() -> Weight;
                fn transfer_claim() -> Weight;
             }
        (3). pub trait Trait: frame_system::Trait {
                type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
                // TODO chw 6.2
                type WeightInfo : WeightInfo;
             }
        (4). decl_module! {
                pub struct Module<T: Trait> for enum Call where origin: T::Origin {
                    // --snip--
                    // TODO chw 6.3
                    #[weight = T::WeightInfo::create_claim()]
                    pub fn create_claim(origin, claim: Vec<u8>) -> dispatch::DispatchResult{
                        ...
                    }
                 }
              }

    b. substrate/bin/node/runtime/src/lib.rs
        impl pallet_poe::Trait for Runtime {
            type Event = Event;
            // TODO chw 7.0
            // type WeightInfo = weights::pallet_poe::WeightInfo<Runtime>;
        }

 */

/*

3. 错误信息：
    ~/chw/bifrost/bin/node/cli$ cargo build --release --features runtime-benchmarks
    Updating `https://mirrors.tuna.tsinghua.edu.cn/git/crates.io-index.git` index
    error: failed to select a version for `node-runtime`.
        ... required by package `node-cli v2.0.0 (/home/bifrost/chw/bifrost/bin/node/cli)`
    versions that meet the requirements `=0.5.0` are: 0.5.0
    the package `node-cli` depends on `node-runtime`, with features: `runtime-benchmarks` but
    `node-runtime` does not have these features.
    failed to select a version for `node-runtime` which could resolve this conflict

    a. 解析：
       the package `node-cli` depends on `node-runtime`, with features: `runtime-benchmarks` but
       `node-runtime` does not have these features.
       // 客户端结点 node-cli (即 cli 模块)依赖运行时结点 node-runtime (即 runtime 模块) 的
          " runtime-benchmarks "属性，但是 runtime 结点没有这个属性

    b. 解决方案：
       在" bifrost/bin/node/runtime/Cargo.toml "下添加" runtime-benchmarks "属性如：
       runtime-benchmarks = [
            "frame-benchmarking",
            "pallet-babe/runtime-benchmarks",
            "pallet-balances/runtime-benchmarks",
            "pallet-proxy/runtime-benchmarks",
            "pallet-treasury/runtime-benchmarks",
            // -- 追加欲测试的模块 --
       ]

4. 某个模块未实现 trait 的错误：
    error[E0046]: not all trait items implemented, missing: `successful_origin`
        --> /home/bifrost/.cargo/registry/src/mirro...f42cdbd/pallet-society-2.0.0/src/lib.rs:1154:1
             |
        1154 | impl<T: Trait> EnsureOrigin<T::Origin> for EnsureFounder<T> {
             | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing `successful_origin` in implementation
             |
             = help: implement the missing item: `fn successful_origin() -> OuterOrigin { todo!() }`
        error: aborting due to previous error
        For more information about this error, try `rustc --explain E0046`.
        error: could not compile `pallet-society`.
    // 解析：据" error: could not compile `pallet-society`. "可知" pallet-society "模块未实现相应的 trait
    // 解决方案：为其加入相应依赖(即添加相应实现)
       即在" bin/node/runtime/Cargo.toml "之" runtime-benchmarks = [ ... ] "中加入相应依赖即：
        runtime-benchmarks = [
            # --snip--
            "pallet-society/runtime-benchmarks",
            # --snip--
        ]

 */