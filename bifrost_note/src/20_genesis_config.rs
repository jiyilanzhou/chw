
/*
0. 创世块配置
	decl_storage! {
		trait Store for Module<T: Trait> as Mint {
			// " config() "配置生成对应的" genesis_price "字段(用于 add_extra_genesis 配置)
			BncProduce get(fn genesis_price) config(): (T::BlockNumber, T::Balance);
		}
		add_extra_genesis {
			build(|config: &GenesisConfig<T>| {
				// --snip--
				// 只能使用" config() "配置生成的对应字段" genesis_price "
				// 配置数据源于" bin/node/service/src/chain_spec/bifrost.rs "
			});
		}
	}
	// " config() "生成的对应源码
	pub struct GenesisConfig<T: Trait> {
	    pub genesis_price: (T::BlockNumber, T::Balance),
	}

1. 是否需要配置创世块(及创世块是否需要声明泛型)
    # " bifrost/bin/node/runtime/src/lib.rs "
    construct_runtime!(
        pub enum Runtime where
            Block = Block,
            NodeBlock = node_primitives::Block,
            UncheckedExtrinsic = UncheckedExtrinsic
        {
            // -- snip --
            // Rebate       // 据此 Config<T> 可知需要配置创世块及配置泛型 <T>
		    Rebate: brml_rebate::{Module, Call, Storage, Event, Config<T>},
            // -- snip --
        }
     );

2. pallet 创世块配置
    # " bifrost/brml/rebate/src/lib.rs "
    decl_storage! {
        trait Store for Module<T: Trait> as Convert {
            // --snip--
        }
        // 更新初始化数据
        add_extra_genesis {
            build(|config: &GenesisConfig<T>| {
                  // --snip--
            });
        }
    }

3. 创世块初始化数据
    a. substrate 配置
        #" bifrost/bin/node/cli/src/chain_spec.rs "
        use node_runtime::{
             SystemConfig, ... RebateConfig,
        };
        /// Helper function to create GenesisConfig for testing
        pub fn testnet_genesis(
            initial_authorities: Vec<(
                AccountId,
                AccountId,
                GrandpaId,
                BabeId,
                ImOnlineId,
                AuthorityDiscoveryId,
            )>,
            root_key: AccountId,
            endowed_accounts: Option<Vec<AccountId>>,
        ) -> GenesisConfig {
            // --snip--
            GenesisConfig {
                frame_system: Some(SystemConfig {
                    code: wasm_binary_unwrap().to_vec(),
                    changes_trie_config: Default::default(),
                }),
                pallet_rebate: Some(BalancesConfig {
                    // --snip--
                }),
                // --snip--
                brml_swap: initialize_swap_module(root_key),
            }
        }
    b. bifrost 在 3.a 的基础上还追加的配置
        # 以下仅为 bifrost 配置(原 substrate 只有 3.a 配置)
        /// Helper function to create GenesisConfig for bifrost
        pub fn bifrost_genesis(
            initial_authorities: Vec<(AccountId, AccountId, GrandpaId, BabeId, ImOnlineId, AuthorityDiscoveryId)>,
            root_key: AccountId,
            endowed_accounts: Vec<AccountId>,
        ) -> GenesisConfig {
         // --snip--
            GenesisConfig {
                frame_system: Some(SystemConfig {
                    code: wasm_binary_unwrap().to_vec(),
                    changes_trie_config: Default::default(),
                }),
                pallet_rebate: Some(BalancesConfig {
                    // --snip--
                }),
                // --snip--
                brml_swap: initialize_swap_module(root_key),
            }
        }

 */