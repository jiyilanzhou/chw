
/*
0. decl_storage!{ ... }

1. 链接信息
    https://www.youtube.com/watch?v=5PSllaWbYag
    时间：" 18 : 33 "

*/
/*
2. cargo expand 扩展部分解析
   a. 结构
        decl_storage! {
            trait Store for Module<T: Trait> as Convert {
                // --snip--
                Proofs get(fn proofs):map hasher(blake2_128_concat) Vec<u8> => (T::AccountId, T::BlockNumber);
                Key get(fn key) config() : T::AccountId;
            }
        }
   b. 扩展部分源码解析
    trait Store {
        type Proofs;
        type Key;
    }
    impl<T: Trait + 'static> Store for Module<T> {
        type Proofs = Proofs<T>;
        type Key = Key<T>;
    }
    impl<T: Trait + 'static> Module<T> {
        pub fn proofs<
            K: self::sp_api_hidden_includes_decl_storage::hidden_include::codec::EncodeLike<Vec<u8>>,
        >(
            key: K,
        ) -> (T::AccountId, T::BlockNumber) {
            < Proofs < T > as self :: sp_api_hidden_includes_decl_storage :: hidden_include :: storage :: StorageMap < Vec < u8 > , (T :: AccountId , T :: BlockNumber) > > :: get (key)
        }
        pub fn key() -> T::AccountId {
            < Key < T > as self :: sp_api_hidden_includes_decl_storage :: hidden_include :: storage :: StorageValue < T :: AccountId > > :: get ()
        }
    }

    impl<T: Trait + 'static> Module<T> {
        #[doc(hidden)]
        pub fn storage_metadata(
        ) -> self::sp_api_hidden_includes_decl_storage::hidden_include::metadata::StorageMetadata {
            self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageMetadata {
                // --snip--
                ty : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageEntryType :: Map { hasher : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageHasher :: Blake2_128Concat ,
                key : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ("Vec<u8>") ,
                value : self :: ... :: DecodeDifferent :: Encode ("(T::AccountId, T::BlockNumber)") ,
                // --snip--
            }
        }
    }
    struct Proofs<T: Trait>(
        self::sp_api_hidden_includes_decl_storage::hidden_include::sp_std::marker::PhantomData<(T,)>,
    );
    impl<T: Trait>
        self::sp_api_hidden_includes_decl_storage::hidden_include::storage::StoragePrefixedMap<(
            T::AccountId,
            T::BlockNumber,
        )> for Proofs<T>
    {
        fn module_prefix() -> &'static [u8] {
            < __InherentHiddenInstance as self :: sp_api_hidden_includes_decl_storage :: hidden_include :: traits :: Instance > :: PREFIX . as_bytes ()
        }
        fn storage_prefix() -> &'static [u8] {
            b"Proofs"
        }
    }
    impl<T: Trait>
        self::sp_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageMap<
            Vec<u8>,
            (T::AccountId, T::BlockNumber),
        > for Proofs<T>
    {
        type Query = (T::AccountId, T::BlockNumber);
        type Hasher = self::sp_api_hidden_includes_decl_storage::hidden_include::Blake2_128Concat;
        fn module_prefix() -> &'static [u8] {
            < __InherentHiddenInstance as self :: sp_api_hidden_includes_decl_storage :: hidden_include :: traits :: Instance > :: PREFIX . as_bytes ()
        }
        fn storage_prefix() -> &'static [u8] {
            b"Proofs"
        }
        fn from_optional_value_to_query(v: Option<(T::AccountId, T::BlockNumber)>) -> Self::Query {
            v.unwrap_or_else(|| Default::default())
        }
        fn from_query_to_optional_value(v: Self::Query) -> Option<(T::AccountId, T::BlockNumber)> {
            Some(v)
        }
    }
    struct Key<T: Trait>(
        self::sp_api_hidden_includes_decl_storage::hidden_include::sp_std::marker::PhantomData<(T,)>,
    );
    impl<T: Trait>
        self::sp_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageValue<
            T::AccountId,
        > for Key<T>
    {
        type Query = T::AccountId;
        fn module_prefix() -> &'static [u8] {
            < __InherentHiddenInstance as self :: sp_api_hidden_includes_decl_storage :: hidden_include :: traits :: Instance > :: PREFIX . as_bytes ()
        }
        fn storage_prefix() -> &'static [u8] {
            b"Key"
        }
        fn from_optional_value_to_query(v: Option<T::AccountId>) -> Self::Query {
            v.unwrap_or_else(|| Default::default())
        }
        fn from_query_to_optional_value(v: Self::Query) -> Option<T::AccountId> {
            Some(v)
        }
    }

 */