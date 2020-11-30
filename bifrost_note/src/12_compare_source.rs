

// manifest[ˈmænɪfest]adj&n.明显的,清单
/*
0.  使用 diff 工具
    对比 weightInfo
    diff /Users/chw/project/bifrost/bin/node/runtime/src/weights \
         /Users/chw/project/substrate/bin/node/runtime/src/weights


// ratio[ˈreɪʃiəʊ]n.比率,比例       // exceed[ɪkˈsiːd]v.超过
1. 解决 system_setcode 升级成功但 scheduler + setcode 升级失败的方案
    a. 从错误信息" Transaction would exhaust the block limits "入手，全局搜索查找到相对应文件
       其"  substrate/primitives/runtime/src/transaction_validity.rs "部分源码如下：
       // -- snip --
       impl From<InvalidTransaction> for &'static str {
            fn from(invalid: InvalidTransaction) -> &'static str {
                match invalid {
                    InvalidTransaction::Call => "Transaction call is not expected",
                    InvalidTransaction::Future => "Transaction will be valid in the future",
                    InvalidTransaction::Stale => "Transaction is outdated",
                    InvalidTransaction::BadProof => "Transaction has a bad signature",
                    InvalidTransaction::AncientBirthBlock => "Transaction has an ancient birth block",
                    // 对应的错误信息
                    InvalidTransaction::ExhaustsResources =>
                        "Transaction would exhaust the block limits",
                    InvalidTransaction::Payment =>
                        "Inability to pay some fees (e.g. account balance too low)",
                    InvalidTransaction::BadMandatory =>
                        "A call was labelled as mandatory, but resulted in an Error.",
                    InvalidTransaction::MandatoryDispatch =>
                        "Transaction dispatch is mandatory; transactions may not have mandatory dispatches.",
                    InvalidTransaction::Custom(_) => "InvalidTransaction custom error",
                }
            }
        }
    c. 再根据" InvalidTransaction::ExhaustsResources "全局搜索查找到相应文件
       其" substrate/frame/system/src/extensions/check_weight.rs "部分源码如下：
       // --snip--
       impl<T: Trait + Send + Sync> CheckWeight<T> where
            T::Call: Dispatchable<Info=DispatchInfo, PostInfo=PostDispatchInfo>
        {
            /// Get the quota ratio of each dispatch class type. This indicates that all operational and mandatory
            /// dispatches can use the full capacity of any resource, while user-triggered ones can consume
            /// a portion.
            fn get_dispatch_limit_ratio(class: DispatchClass) -> Perbill {
                match class {
                    DispatchClass::Operational | DispatchClass::Mandatory
                        => <Perbill as sp_runtime::PerThing>::one(),
                    DispatchClass::Normal => T::AvailableBlockRatio::get(),
                }
            }
            // 检查当前外部是否不超过“MaximumExtrinsicWeight”限制。
            /// Checks if the current extrinsic does not exceed `MaximumExtrinsicWeight` limit.
            fn check_extrinsic_weight(
                info: &DispatchInfoOf<T::Call>,
            ) -> Result<(), TransactionValidityError> {
                match info.class {
                    // Mandatory transactions are included in a block unconditionally, so
                    // we don't verify weight.
                    DispatchClass::Mandatory => Ok(()),
                    // Normal transactions must not exceed `MaximumExtrinsicWeight`.
                    DispatchClass::Normal => {
                        let maximum_weight = T::MaximumExtrinsicWeight::get();
                        let extrinsic_weight = info.weight.saturating_add(T::ExtrinsicBaseWeight::get());
                        if extrinsic_weight > maximum_weight {
                            // 匹配代码
                            Err(InvalidTransaction::ExhaustsResources.into())
                        } else {
                            Ok(())
                        }
                    },
                    // For operational transactions we make sure it doesn't exceed
                    // the space alloted for `Operational` class.
                    DispatchClass::Operational => {
                        let maximum_weight = T::MaximumBlockWeight::get();
                        let operational_limit =
                            Self::get_dispatch_limit_ratio(DispatchClass::Operational) * maximum_weight;
                        let operational_limit =
                            operational_limit.saturating_sub(T::BlockExecutionWeight::get());
                        let extrinsic_weight = info.weight.saturating_add(T::ExtrinsicBaseWeight::get());
                        if extrinsic_weight > operational_limit {
                        // 匹配代码
                            Err(InvalidTransaction::ExhaustsResources.into())
                        } else {
                            Ok(())
                        }
                    },
                }
            }

            /// Checks if the current extrinsic can fit into the block with respect to block weight limits.
            ///
            /// Upon successes, it returns the new block weight as a `Result`.
            fn check_block_weight(
                info: &DispatchInfoOf<T::Call>,
            ) -> Result<crate::weights::ExtrinsicsWeight, TransactionValidityError> {
                let maximum_weight = T::MaximumBlockWeight::get();
                let mut all_weight = Module::<T>::block_weight();
                match info.class {
                    // If we have a dispatch that must be included in the block, it ignores all the limits.
                    DispatchClass::Mandatory => {
                        let extrinsic_weight = info.weight.saturating_add(T::ExtrinsicBaseWeight::get());
                        all_weight.add(extrinsic_weight, DispatchClass::Mandatory);
                        Ok(all_weight)
                    },
                    // If we have a normal dispatch, we follow all the normal rules and limits.
                    DispatchClass::Normal => {
                        let normal_limit = Self::get_dispatch_limit_ratio(DispatchClass::Normal) * maximum_weight;
                        let extrinsic_weight = info.weight.checked_add(T::ExtrinsicBaseWeight::get())
                            .ok_or(InvalidTransaction::ExhaustsResources)?;
                        all_weight.checked_add(extrinsic_weight, DispatchClass::Normal)
                            .map_err(|_| InvalidTransaction::ExhaustsResources)?;
                        if all_weight.get(DispatchClass::Normal) > normal_limit {
                            // 匹配代码
                            Err(InvalidTransaction::ExhaustsResources.into())
                        } else {
                            Ok(all_weight)
                        }
                    },
                    // If we have an operational dispatch, allow it if we have not used our full
                    // "operational space" (independent of existing fullness).
                    DispatchClass::Operational => {
                        let operational_limit = Self::get_dispatch_limit_ratio(DispatchClass::Operational) * maximum_weight;
                        let normal_limit = Self::get_dispatch_limit_ratio(DispatchClass::Normal) * maximum_weight;
                        let operational_space = operational_limit.saturating_sub(normal_limit);

                        let extrinsic_weight = info.weight.checked_add(T::ExtrinsicBaseWeight::get())
                            .ok_or(InvalidTransaction::ExhaustsResources)?;
                        all_weight.checked_add(extrinsic_weight, DispatchClass::Operational)
                            .map_err(|_| InvalidTransaction::ExhaustsResources)?;

                        // If it would fit in normally, its okay
                        if all_weight.total() <= maximum_weight ||
                        // If we have not used our operational space
                        all_weight.get(DispatchClass::Operational) <= operational_space {
                            Ok(all_weight)
                        } else {
                            // 匹配代码
                            Err(InvalidTransaction::ExhaustsResources.into())
                        }
                    }
                }
            }

            /// Checks if the current extrinsic can fit into the block with respect to block length limits.
            ///
            /// Upon successes, it returns the new block length as a `Result`.
            fn check_block_length(
                info: &DispatchInfoOf<T::Call>,
                len: usize,
            ) -> Result<u32, TransactionValidityError> {
                let current_len = Module::<T>::all_extrinsics_len();
                let maximum_len = T::MaximumBlockLength::get();
                let limit = Self::get_dispatch_limit_ratio(info.class) * maximum_len;
                let added_len = len as u32;
                let next_len = current_len.saturating_add(added_len);
                if next_len > limit {
                    // 匹配代码
                    Err(InvalidTransaction::ExhaustsResources.into())
                } else {
                    Ok(next_len)
                }
            }

       }

    d. 查看关联" substrate/bin/node/runtime/src/lib.rs "相应代码
       // -- snip --
       parameter_types! {
            pub const BlockHashCount: BlockNumber = 2400;
            /// We allow for 2 seconds of compute with a 6 second average block time.
            pub const MaximumBlockWeight: Weight = 2 * WEIGHT_PER_SECOND;
            pub const AvailableBlockRatio: Perbill = Perbill::from_percent(75);
            /// Assume 10% of weight for average on_initialize calls.
            pub MaximumExtrinsicWeight: Weight =
                AvailableBlockRatio::get().saturating_sub(AVERAGE_ON_INITIALIZE_WEIGHT)
                * MaximumBlockWeight::get();
            pub const MaximumBlockLength: u32 = 5 * 1024 * 1024;
            pub const Version: RuntimeVersion = VERSION;
      }
      // 文件大小约为" MaximumBlockLength * AvailableBlockRatio ≈ 4 M "故文件比较大

    e. 解决方案：压缩文件





1. 二进制文件皆可压缩
    a.
    b.


2. "bifrost/bin/node/runtime/build.rs"源码解析
    a. "bifrost/bin/node/runtime/Cargo.toml"配置依赖
        [build-dependencies]
        wasm-builder-runner = { version = "1.0.5", package = "substrate-wasm-builder-runner" }
    b. 源码
        fn main() {
            WasmBuilder::new()
                // 获取包裹项目下 Cargo.toml 二进制文件路径的元组
                .with_current_project()
                // 从 crates.io 上获取指定版本 wasm-builder 及其 Cargo.toml 文件内所依赖 crate 的源码
                .with_wasm_builder_from_crates("2.0.0")
                /* Enable exporting `__heap_base` as global variable in the WASM binary.
                   This adds `-Clink-arg=--export=__heap_base` to `RUST_FLAGS`.
                 */
                .export_heap_base()
                /* Instruct the linker to import the memory into the WASM binary.
                   This adds `-C link-arg=--import-memory` to `RUST_FLAGS`.
                 */
                .import_memory()
                // 此参数用于压缩生成的二进制文件(解决 scheduler + setcode 升级失败之一方案)
                .append_to_rust_flags("-C opt-level=z")
                // Build the WASM binary
                .build()
        }

 */

