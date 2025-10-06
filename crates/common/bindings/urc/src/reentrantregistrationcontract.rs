///Module containing a contract's types and functions.
/**

```solidity
library BLS {
    struct G1Point { bytes32 x_a; bytes32 x_b; bytes32 y_a; bytes32 y_b; }
    struct G2Point { bytes32 x_c0_a; bytes32 x_c0_b; bytes32 x_c1_a; bytes32 x_c1_b; bytes32 y_c0_a; bytes32 y_c0_b; bytes32 y_c1_a; bytes32 y_c1_b; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod BLS {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**```solidity
struct G1Point { bytes32 x_a; bytes32 x_b; bytes32 y_a; bytes32 y_b; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct G1Point {
        #[allow(missing_docs)]
        pub x_a: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub x_b: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub y_a: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub y_b: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::FixedBytes<32>,
            alloy::sol_types::sol_data::FixedBytes<32>,
            alloy::sol_types::sol_data::FixedBytes<32>,
            alloy::sol_types::sol_data::FixedBytes<32>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::FixedBytes<32>,
            alloy::sol_types::private::FixedBytes<32>,
            alloy::sol_types::private::FixedBytes<32>,
            alloy::sol_types::private::FixedBytes<32>,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<G1Point> for UnderlyingRustTuple<'_> {
            fn from(value: G1Point) -> Self {
                (value.x_a, value.x_b, value.y_a, value.y_b)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for G1Point {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    x_a: tuple.0,
                    x_b: tuple.1,
                    y_a: tuple.2,
                    y_b: tuple.3,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for G1Point {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for G1Point {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.x_a),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.x_b),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.y_a),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.y_b),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(&tuple, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_packed_encoded_size(&tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for G1Point {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for G1Point {
            const NAME: &'static str = "G1Point";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "G1Point(bytes32 x_a,bytes32 x_b,bytes32 y_a,bytes32 y_b)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                alloy_sol_types::private::Vec::new()
            }
            #[inline]
            fn eip712_encode_type() -> alloy_sol_types::private::Cow<'static, str> {
                <Self as alloy_sol_types::SolStruct>::eip712_root_type()
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.x_a)
                        .0,
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.x_b)
                        .0,
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.y_a)
                        .0,
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.y_b)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for G1Point {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.x_a)
                    + <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.x_b)
                    + <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.y_a)
                    + <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.y_b)
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(
                    <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
                );
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(&rust.x_a, out);
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(&rust.x_b, out);
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(&rust.y_a, out);
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(&rust.y_b, out);
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    rust,
                    &mut out,
                );
                alloy_sol_types::abi::token::WordToken(
                    alloy_sol_types::private::keccak256(out),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**```solidity
struct G2Point { bytes32 x_c0_a; bytes32 x_c0_b; bytes32 x_c1_a; bytes32 x_c1_b; bytes32 y_c0_a; bytes32 y_c0_b; bytes32 y_c1_a; bytes32 y_c1_b; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct G2Point {
        #[allow(missing_docs)]
        pub x_c0_a: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub x_c0_b: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub x_c1_a: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub x_c1_b: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub y_c0_a: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub y_c0_b: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub y_c1_a: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub y_c1_b: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::FixedBytes<32>,
            alloy::sol_types::sol_data::FixedBytes<32>,
            alloy::sol_types::sol_data::FixedBytes<32>,
            alloy::sol_types::sol_data::FixedBytes<32>,
            alloy::sol_types::sol_data::FixedBytes<32>,
            alloy::sol_types::sol_data::FixedBytes<32>,
            alloy::sol_types::sol_data::FixedBytes<32>,
            alloy::sol_types::sol_data::FixedBytes<32>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::FixedBytes<32>,
            alloy::sol_types::private::FixedBytes<32>,
            alloy::sol_types::private::FixedBytes<32>,
            alloy::sol_types::private::FixedBytes<32>,
            alloy::sol_types::private::FixedBytes<32>,
            alloy::sol_types::private::FixedBytes<32>,
            alloy::sol_types::private::FixedBytes<32>,
            alloy::sol_types::private::FixedBytes<32>,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<G2Point> for UnderlyingRustTuple<'_> {
            fn from(value: G2Point) -> Self {
                (
                    value.x_c0_a,
                    value.x_c0_b,
                    value.x_c1_a,
                    value.x_c1_b,
                    value.y_c0_a,
                    value.y_c0_b,
                    value.y_c1_a,
                    value.y_c1_b,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for G2Point {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    x_c0_a: tuple.0,
                    x_c0_b: tuple.1,
                    x_c1_a: tuple.2,
                    x_c1_b: tuple.3,
                    y_c0_a: tuple.4,
                    y_c0_b: tuple.5,
                    y_c1_a: tuple.6,
                    y_c1_b: tuple.7,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for G2Point {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for G2Point {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.x_c0_a),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.x_c0_b),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.x_c1_a),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.x_c1_b),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.y_c0_a),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.y_c0_b),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.y_c1_a),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.y_c1_b),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(&tuple, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_packed_encoded_size(&tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for G2Point {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for G2Point {
            const NAME: &'static str = "G2Point";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "G2Point(bytes32 x_c0_a,bytes32 x_c0_b,bytes32 x_c1_a,bytes32 x_c1_b,bytes32 y_c0_a,bytes32 y_c0_b,bytes32 y_c1_a,bytes32 y_c1_b)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                alloy_sol_types::private::Vec::new()
            }
            #[inline]
            fn eip712_encode_type() -> alloy_sol_types::private::Cow<'static, str> {
                <Self as alloy_sol_types::SolStruct>::eip712_root_type()
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.x_c0_a)
                        .0,
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.x_c0_b)
                        .0,
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.x_c1_a)
                        .0,
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.x_c1_b)
                        .0,
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.y_c0_a)
                        .0,
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.y_c0_b)
                        .0,
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.y_c1_a)
                        .0,
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.y_c1_b)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for G2Point {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.x_c0_a,
                    )
                    + <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.x_c0_b,
                    )
                    + <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.x_c1_a,
                    )
                    + <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.x_c1_b,
                    )
                    + <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.y_c0_a,
                    )
                    + <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.y_c0_b,
                    )
                    + <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.y_c1_a,
                    )
                    + <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.y_c1_b,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(
                    <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
                );
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.x_c0_a,
                    out,
                );
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.x_c0_b,
                    out,
                );
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.x_c1_a,
                    out,
                );
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.x_c1_b,
                    out,
                );
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.y_c0_a,
                    out,
                );
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.y_c0_b,
                    out,
                );
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.y_c1_a,
                    out,
                );
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.y_c1_b,
                    out,
                );
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    rust,
                    &mut out,
                );
                alloy_sol_types::abi::token::WordToken(
                    alloy_sol_types::private::keccak256(out),
                )
            }
        }
    };
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`BLS`](self) contract instance.

See the [wrapper's documentation](`BLSInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(address: alloy_sol_types::private::Address, provider: P) -> BLSInstance<T, P, N> {
        BLSInstance::<T, P, N>::new(address, provider)
    }
    /**A [`BLS`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`BLS`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct BLSInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for BLSInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("BLSInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > BLSInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`BLS`](self) contract instance.

See the [wrapper's documentation](`BLSInstance`) for more details.*/
        #[inline]
        pub const fn new(
            address: alloy_sol_types::private::Address,
            provider: P,
        ) -> Self {
            Self {
                address,
                provider,
                _network_transport: ::core::marker::PhantomData,
            }
        }
        /// Returns a reference to the address.
        #[inline]
        pub const fn address(&self) -> &alloy_sol_types::private::Address {
            &self.address
        }
        /// Sets the address.
        #[inline]
        pub fn set_address(&mut self, address: alloy_sol_types::private::Address) {
            self.address = address;
        }
        /// Sets the address and returns `self`.
        pub fn at(mut self, address: alloy_sol_types::private::Address) -> Self {
            self.set_address(address);
            self
        }
        /// Returns a reference to the provider.
        #[inline]
        pub const fn provider(&self) -> &P {
            &self.provider
        }
    }
    impl<T, P: ::core::clone::Clone, N> BLSInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> BLSInstance<T, P, N> {
            BLSInstance {
                address: self.address,
                provider: ::core::clone::Clone::clone(&self.provider),
                _network_transport: ::core::marker::PhantomData,
            }
        }
    }
    /// Function calls.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > BLSInstance<T, P, N> {
        /// Creates a new call builder using this contract instance's provider and address.
        ///
        /// Note that the call can be any function call, not just those defined in this
        /// contract. Prefer using the other methods for building type-safe contract calls.
        pub fn call_builder<C: alloy_sol_types::SolCall>(
            &self,
            call: &C,
        ) -> alloy_contract::SolCallBuilder<T, &P, C, N> {
            alloy_contract::SolCallBuilder::new_sol(&self.provider, &self.address, call)
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > BLSInstance<T, P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<T, &P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
    }
}
///Module containing a contract's types and functions.
/**

```solidity
library IRegistry {
    struct SignedRegistration { BLS.G1Point pubkey; BLS.G2Point signature; uint64 nonce; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod IRegistry {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**```solidity
struct SignedRegistration { BLS.G1Point pubkey; BLS.G2Point signature; uint64 nonce; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct SignedRegistration {
        #[allow(missing_docs)]
        pub pubkey: <BLS::G1Point as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub signature: <BLS::G2Point as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub nonce: u64,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            BLS::G1Point,
            BLS::G2Point,
            alloy::sol_types::sol_data::Uint<64>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            <BLS::G1Point as alloy::sol_types::SolType>::RustType,
            <BLS::G2Point as alloy::sol_types::SolType>::RustType,
            u64,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<SignedRegistration> for UnderlyingRustTuple<'_> {
            fn from(value: SignedRegistration) -> Self {
                (value.pubkey, value.signature, value.nonce)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for SignedRegistration {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    pubkey: tuple.0,
                    signature: tuple.1,
                    nonce: tuple.2,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for SignedRegistration {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for SignedRegistration {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <BLS::G1Point as alloy_sol_types::SolType>::tokenize(&self.pubkey),
                    <BLS::G2Point as alloy_sol_types::SolType>::tokenize(
                        &self.signature,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.nonce),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(&tuple, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_packed_encoded_size(&tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for SignedRegistration {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for SignedRegistration {
            const NAME: &'static str = "SignedRegistration";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "SignedRegistration(BLS.G1Point pubkey,BLS.G2Point signature,uint64 nonce)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                let mut components = alloy_sol_types::private::Vec::with_capacity(2);
                components
                    .push(
                        <BLS::G1Point as alloy_sol_types::SolStruct>::eip712_root_type(),
                    );
                components
                    .extend(
                        <BLS::G1Point as alloy_sol_types::SolStruct>::eip712_components(),
                    );
                components
                    .push(
                        <BLS::G2Point as alloy_sol_types::SolStruct>::eip712_root_type(),
                    );
                components
                    .extend(
                        <BLS::G2Point as alloy_sol_types::SolStruct>::eip712_components(),
                    );
                components
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <BLS::G1Point as alloy_sol_types::SolType>::eip712_data_word(
                            &self.pubkey,
                        )
                        .0,
                    <BLS::G2Point as alloy_sol_types::SolType>::eip712_data_word(
                            &self.signature,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.nonce)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for SignedRegistration {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <BLS::G1Point as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.pubkey,
                    )
                    + <BLS::G2Point as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.signature,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.nonce)
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(
                    <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
                );
                <BLS::G1Point as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.pubkey,
                    out,
                );
                <BLS::G2Point as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.signature,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    64,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.nonce,
                    out,
                );
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    rust,
                    &mut out,
                );
                alloy_sol_types::abi::token::WordToken(
                    alloy_sol_types::private::keccak256(out),
                )
            }
        }
    };
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`IRegistry`](self) contract instance.

See the [wrapper's documentation](`IRegistryInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> IRegistryInstance<T, P, N> {
        IRegistryInstance::<T, P, N>::new(address, provider)
    }
    /**A [`IRegistry`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`IRegistry`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct IRegistryInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for IRegistryInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("IRegistryInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > IRegistryInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`IRegistry`](self) contract instance.

See the [wrapper's documentation](`IRegistryInstance`) for more details.*/
        #[inline]
        pub const fn new(
            address: alloy_sol_types::private::Address,
            provider: P,
        ) -> Self {
            Self {
                address,
                provider,
                _network_transport: ::core::marker::PhantomData,
            }
        }
        /// Returns a reference to the address.
        #[inline]
        pub const fn address(&self) -> &alloy_sol_types::private::Address {
            &self.address
        }
        /// Sets the address.
        #[inline]
        pub fn set_address(&mut self, address: alloy_sol_types::private::Address) {
            self.address = address;
        }
        /// Sets the address and returns `self`.
        pub fn at(mut self, address: alloy_sol_types::private::Address) -> Self {
            self.set_address(address);
            self
        }
        /// Returns a reference to the provider.
        #[inline]
        pub const fn provider(&self) -> &P {
            &self.provider
        }
    }
    impl<T, P: ::core::clone::Clone, N> IRegistryInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> IRegistryInstance<T, P, N> {
            IRegistryInstance {
                address: self.address,
                provider: ::core::clone::Clone::clone(&self.provider),
                _network_transport: ::core::marker::PhantomData,
            }
        }
    }
    /// Function calls.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > IRegistryInstance<T, P, N> {
        /// Creates a new call builder using this contract instance's provider and address.
        ///
        /// Note that the call can be any function call, not just those defined in this
        /// contract. Prefer using the other methods for building type-safe contract calls.
        pub fn call_builder<C: alloy_sol_types::SolCall>(
            &self,
            call: &C,
        ) -> alloy_contract::SolCallBuilder<T, &P, C, N> {
            alloy_contract::SolCallBuilder::new_sol(&self.provider, &self.address, call)
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > IRegistryInstance<T, P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<T, &P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
    }
}
///Module containing a contract's types and functions.
/**

```solidity
library ISlasher {
    struct Commitment { uint64 commitmentType; bytes payload; address slasher; }
    struct Delegation { BLS.G1Point proposer; BLS.G1Point delegate; address committer; uint64 slot; bytes metadata; }
    struct SignedCommitment { Commitment commitment; bytes signature; }
    struct SignedDelegation { Delegation delegation; uint64 nonce; bytes32 signingId; BLS.G2Point signature; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod ISlasher {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**```solidity
struct Commitment { uint64 commitmentType; bytes payload; address slasher; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct Commitment {
        #[allow(missing_docs)]
        pub commitmentType: u64,
        #[allow(missing_docs)]
        pub payload: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub slasher: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Uint<64>,
            alloy::sol_types::sol_data::Bytes,
            alloy::sol_types::sol_data::Address,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            u64,
            alloy::sol_types::private::Bytes,
            alloy::sol_types::private::Address,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<Commitment> for UnderlyingRustTuple<'_> {
            fn from(value: Commitment) -> Self {
                (value.commitmentType, value.payload, value.slasher)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for Commitment {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    commitmentType: tuple.0,
                    payload: tuple.1,
                    slasher: tuple.2,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for Commitment {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for Commitment {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.commitmentType),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.payload,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.slasher,
                    ),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(&tuple, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_packed_encoded_size(&tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for Commitment {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for Commitment {
            const NAME: &'static str = "Commitment";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "Commitment(uint64 commitmentType,bytes payload,address slasher)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                alloy_sol_types::private::Vec::new()
            }
            #[inline]
            fn eip712_encode_type() -> alloy_sol_types::private::Cow<'static, str> {
                <Self as alloy_sol_types::SolStruct>::eip712_root_type()
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.commitmentType,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::eip712_data_word(
                            &self.payload,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.slasher,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for Commitment {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.commitmentType,
                    )
                    + <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.payload,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.slasher,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(
                    <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
                );
                <alloy::sol_types::sol_data::Uint<
                    64,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.commitmentType,
                    out,
                );
                <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.payload,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.slasher,
                    out,
                );
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    rust,
                    &mut out,
                );
                alloy_sol_types::abi::token::WordToken(
                    alloy_sol_types::private::keccak256(out),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**```solidity
struct Delegation { BLS.G1Point proposer; BLS.G1Point delegate; address committer; uint64 slot; bytes metadata; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct Delegation {
        #[allow(missing_docs)]
        pub proposer: <BLS::G1Point as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub delegate: <BLS::G1Point as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub committer: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub slot: u64,
        #[allow(missing_docs)]
        pub metadata: alloy::sol_types::private::Bytes,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            BLS::G1Point,
            BLS::G1Point,
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Uint<64>,
            alloy::sol_types::sol_data::Bytes,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            <BLS::G1Point as alloy::sol_types::SolType>::RustType,
            <BLS::G1Point as alloy::sol_types::SolType>::RustType,
            alloy::sol_types::private::Address,
            u64,
            alloy::sol_types::private::Bytes,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<Delegation> for UnderlyingRustTuple<'_> {
            fn from(value: Delegation) -> Self {
                (
                    value.proposer,
                    value.delegate,
                    value.committer,
                    value.slot,
                    value.metadata,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for Delegation {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    proposer: tuple.0,
                    delegate: tuple.1,
                    committer: tuple.2,
                    slot: tuple.3,
                    metadata: tuple.4,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for Delegation {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for Delegation {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <BLS::G1Point as alloy_sol_types::SolType>::tokenize(&self.proposer),
                    <BLS::G1Point as alloy_sol_types::SolType>::tokenize(&self.delegate),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.committer,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.slot),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.metadata,
                    ),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(&tuple, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_packed_encoded_size(&tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for Delegation {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for Delegation {
            const NAME: &'static str = "Delegation";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "Delegation(BLS.G1Point proposer,BLS.G1Point delegate,address committer,uint64 slot,bytes metadata)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                let mut components = alloy_sol_types::private::Vec::with_capacity(2);
                components
                    .push(
                        <BLS::G1Point as alloy_sol_types::SolStruct>::eip712_root_type(),
                    );
                components
                    .extend(
                        <BLS::G1Point as alloy_sol_types::SolStruct>::eip712_components(),
                    );
                components
                    .push(
                        <BLS::G1Point as alloy_sol_types::SolStruct>::eip712_root_type(),
                    );
                components
                    .extend(
                        <BLS::G1Point as alloy_sol_types::SolStruct>::eip712_components(),
                    );
                components
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <BLS::G1Point as alloy_sol_types::SolType>::eip712_data_word(
                            &self.proposer,
                        )
                        .0,
                    <BLS::G1Point as alloy_sol_types::SolType>::eip712_data_word(
                            &self.delegate,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.committer,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.slot)
                        .0,
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::eip712_data_word(
                            &self.metadata,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for Delegation {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <BLS::G1Point as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.proposer,
                    )
                    + <BLS::G1Point as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.delegate,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.committer,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.slot)
                    + <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.metadata,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(
                    <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
                );
                <BLS::G1Point as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.proposer,
                    out,
                );
                <BLS::G1Point as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.delegate,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.committer,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    64,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.slot,
                    out,
                );
                <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.metadata,
                    out,
                );
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    rust,
                    &mut out,
                );
                alloy_sol_types::abi::token::WordToken(
                    alloy_sol_types::private::keccak256(out),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**```solidity
struct SignedCommitment { Commitment commitment; bytes signature; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct SignedCommitment {
        #[allow(missing_docs)]
        pub commitment: <Commitment as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub signature: alloy::sol_types::private::Bytes,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (Commitment, alloy::sol_types::sol_data::Bytes);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            <Commitment as alloy::sol_types::SolType>::RustType,
            alloy::sol_types::private::Bytes,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<SignedCommitment> for UnderlyingRustTuple<'_> {
            fn from(value: SignedCommitment) -> Self {
                (value.commitment, value.signature)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for SignedCommitment {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    commitment: tuple.0,
                    signature: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for SignedCommitment {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for SignedCommitment {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <Commitment as alloy_sol_types::SolType>::tokenize(&self.commitment),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.signature,
                    ),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(&tuple, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_packed_encoded_size(&tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for SignedCommitment {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for SignedCommitment {
            const NAME: &'static str = "SignedCommitment";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "SignedCommitment(Commitment commitment,bytes signature)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                let mut components = alloy_sol_types::private::Vec::with_capacity(1);
                components
                    .push(
                        <Commitment as alloy_sol_types::SolStruct>::eip712_root_type(),
                    );
                components
                    .extend(
                        <Commitment as alloy_sol_types::SolStruct>::eip712_components(),
                    );
                components
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <Commitment as alloy_sol_types::SolType>::eip712_data_word(
                            &self.commitment,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::eip712_data_word(
                            &self.signature,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for SignedCommitment {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <Commitment as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.commitment,
                    )
                    + <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.signature,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(
                    <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
                );
                <Commitment as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.commitment,
                    out,
                );
                <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.signature,
                    out,
                );
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    rust,
                    &mut out,
                );
                alloy_sol_types::abi::token::WordToken(
                    alloy_sol_types::private::keccak256(out),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**```solidity
struct SignedDelegation { Delegation delegation; uint64 nonce; bytes32 signingId; BLS.G2Point signature; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct SignedDelegation {
        #[allow(missing_docs)]
        pub delegation: <Delegation as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub nonce: u64,
        #[allow(missing_docs)]
        pub signingId: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub signature: <BLS::G2Point as alloy::sol_types::SolType>::RustType,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            Delegation,
            alloy::sol_types::sol_data::Uint<64>,
            alloy::sol_types::sol_data::FixedBytes<32>,
            BLS::G2Point,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            <Delegation as alloy::sol_types::SolType>::RustType,
            u64,
            alloy::sol_types::private::FixedBytes<32>,
            <BLS::G2Point as alloy::sol_types::SolType>::RustType,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<SignedDelegation> for UnderlyingRustTuple<'_> {
            fn from(value: SignedDelegation) -> Self {
                (value.delegation, value.nonce, value.signingId, value.signature)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for SignedDelegation {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    delegation: tuple.0,
                    nonce: tuple.1,
                    signingId: tuple.2,
                    signature: tuple.3,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for SignedDelegation {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for SignedDelegation {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <Delegation as alloy_sol_types::SolType>::tokenize(&self.delegation),
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.nonce),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.signingId),
                    <BLS::G2Point as alloy_sol_types::SolType>::tokenize(&self.signature),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(&tuple, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_packed_encoded_size(&tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for SignedDelegation {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for SignedDelegation {
            const NAME: &'static str = "SignedDelegation";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "SignedDelegation(Delegation delegation,uint64 nonce,bytes32 signingId,BLS.G2Point signature)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                let mut components = alloy_sol_types::private::Vec::with_capacity(2);
                components
                    .push(
                        <Delegation as alloy_sol_types::SolStruct>::eip712_root_type(),
                    );
                components
                    .extend(
                        <Delegation as alloy_sol_types::SolStruct>::eip712_components(),
                    );
                components
                    .push(
                        <BLS::G2Point as alloy_sol_types::SolStruct>::eip712_root_type(),
                    );
                components
                    .extend(
                        <BLS::G2Point as alloy_sol_types::SolStruct>::eip712_components(),
                    );
                components
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <Delegation as alloy_sol_types::SolType>::eip712_data_word(
                            &self.delegation,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.nonce)
                        .0,
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.signingId)
                        .0,
                    <BLS::G2Point as alloy_sol_types::SolType>::eip712_data_word(
                            &self.signature,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for SignedDelegation {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <Delegation as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.delegation,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.nonce)
                    + <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.signingId,
                    )
                    + <BLS::G2Point as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.signature,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(
                    <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
                );
                <Delegation as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.delegation,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    64,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.nonce,
                    out,
                );
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.signingId,
                    out,
                );
                <BLS::G2Point as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.signature,
                    out,
                );
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    rust,
                    &mut out,
                );
                alloy_sol_types::abi::token::WordToken(
                    alloy_sol_types::private::keccak256(out),
                )
            }
        }
    };
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`ISlasher`](self) contract instance.

See the [wrapper's documentation](`ISlasherInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> ISlasherInstance<T, P, N> {
        ISlasherInstance::<T, P, N>::new(address, provider)
    }
    /**A [`ISlasher`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`ISlasher`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct ISlasherInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for ISlasherInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("ISlasherInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > ISlasherInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`ISlasher`](self) contract instance.

See the [wrapper's documentation](`ISlasherInstance`) for more details.*/
        #[inline]
        pub const fn new(
            address: alloy_sol_types::private::Address,
            provider: P,
        ) -> Self {
            Self {
                address,
                provider,
                _network_transport: ::core::marker::PhantomData,
            }
        }
        /// Returns a reference to the address.
        #[inline]
        pub const fn address(&self) -> &alloy_sol_types::private::Address {
            &self.address
        }
        /// Sets the address.
        #[inline]
        pub fn set_address(&mut self, address: alloy_sol_types::private::Address) {
            self.address = address;
        }
        /// Sets the address and returns `self`.
        pub fn at(mut self, address: alloy_sol_types::private::Address) -> Self {
            self.set_address(address);
            self
        }
        /// Returns a reference to the provider.
        #[inline]
        pub const fn provider(&self) -> &P {
            &self.provider
        }
    }
    impl<T, P: ::core::clone::Clone, N> ISlasherInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> ISlasherInstance<T, P, N> {
            ISlasherInstance {
                address: self.address,
                provider: ::core::clone::Clone::clone(&self.provider),
                _network_transport: ::core::marker::PhantomData,
            }
        }
    }
    /// Function calls.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > ISlasherInstance<T, P, N> {
        /// Creates a new call builder using this contract instance's provider and address.
        ///
        /// Note that the call can be any function call, not just those defined in this
        /// contract. Prefer using the other methods for building type-safe contract calls.
        pub fn call_builder<C: alloy_sol_types::SolCall>(
            &self,
            call: &C,
        ) -> alloy_contract::SolCallBuilder<T, &P, C, N> {
            alloy_contract::SolCallBuilder::new_sol(&self.provider, &self.address, call)
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > ISlasherInstance<T, P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<T, &P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
    }
}
///Module containing a contract's types and functions.
/**

```solidity
library UnitTestHelper {
    struct RegisterAndDelegateParams { uint256 proposerSecretKey; uint256 collateral; address owner; uint256 delegateSecretKey; uint256 committerSecretKey; address committer; address slasher; bytes metadata; uint64 slot; bytes32 signingId; uint64 nonce; }
    struct RegisterAndDelegateResult { bytes32 registrationRoot; IRegistry.SignedRegistration[] registrations; ISlasher.SignedDelegation signedDelegation; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod UnitTestHelper {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**```solidity
struct RegisterAndDelegateParams { uint256 proposerSecretKey; uint256 collateral; address owner; uint256 delegateSecretKey; uint256 committerSecretKey; address committer; address slasher; bytes metadata; uint64 slot; bytes32 signingId; uint64 nonce; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct RegisterAndDelegateParams {
        #[allow(missing_docs)]
        pub proposerSecretKey: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub collateral: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub owner: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub delegateSecretKey: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub committerSecretKey: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub committer: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub slasher: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub metadata: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub slot: u64,
        #[allow(missing_docs)]
        pub signingId: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub nonce: u64,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Bytes,
            alloy::sol_types::sol_data::Uint<64>,
            alloy::sol_types::sol_data::FixedBytes<32>,
            alloy::sol_types::sol_data::Uint<64>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::Address,
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::Address,
            alloy::sol_types::private::Address,
            alloy::sol_types::private::Bytes,
            u64,
            alloy::sol_types::private::FixedBytes<32>,
            u64,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<RegisterAndDelegateParams>
        for UnderlyingRustTuple<'_> {
            fn from(value: RegisterAndDelegateParams) -> Self {
                (
                    value.proposerSecretKey,
                    value.collateral,
                    value.owner,
                    value.delegateSecretKey,
                    value.committerSecretKey,
                    value.committer,
                    value.slasher,
                    value.metadata,
                    value.slot,
                    value.signingId,
                    value.nonce,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for RegisterAndDelegateParams {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    proposerSecretKey: tuple.0,
                    collateral: tuple.1,
                    owner: tuple.2,
                    delegateSecretKey: tuple.3,
                    committerSecretKey: tuple.4,
                    committer: tuple.5,
                    slasher: tuple.6,
                    metadata: tuple.7,
                    slot: tuple.8,
                    signingId: tuple.9,
                    nonce: tuple.10,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for RegisterAndDelegateParams {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for RegisterAndDelegateParams {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.proposerSecretKey),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.collateral),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.owner,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.delegateSecretKey),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.committerSecretKey),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.committer,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.slasher,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.metadata,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.slot),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.signingId),
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.nonce),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(&tuple, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_packed_encoded_size(&tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for RegisterAndDelegateParams {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for RegisterAndDelegateParams {
            const NAME: &'static str = "RegisterAndDelegateParams";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "RegisterAndDelegateParams(uint256 proposerSecretKey,uint256 collateral,address owner,uint256 delegateSecretKey,uint256 committerSecretKey,address committer,address slasher,bytes metadata,uint64 slot,bytes32 signingId,uint64 nonce)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                alloy_sol_types::private::Vec::new()
            }
            #[inline]
            fn eip712_encode_type() -> alloy_sol_types::private::Cow<'static, str> {
                <Self as alloy_sol_types::SolStruct>::eip712_root_type()
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.proposerSecretKey,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.collateral)
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.owner,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.delegateSecretKey,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.committerSecretKey,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.committer,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.slasher,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::eip712_data_word(
                            &self.metadata,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.slot)
                        .0,
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.signingId)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.nonce)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for RegisterAndDelegateParams {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.proposerSecretKey,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.collateral,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.owner,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.delegateSecretKey,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.committerSecretKey,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.committer,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.slasher,
                    )
                    + <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.metadata,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.slot)
                    + <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.signingId,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.nonce)
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(
                    <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.proposerSecretKey,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.collateral,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.owner,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.delegateSecretKey,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.committerSecretKey,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.committer,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.slasher,
                    out,
                );
                <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.metadata,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    64,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.slot,
                    out,
                );
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.signingId,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    64,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.nonce,
                    out,
                );
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    rust,
                    &mut out,
                );
                alloy_sol_types::abi::token::WordToken(
                    alloy_sol_types::private::keccak256(out),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive()]
    /**```solidity
struct RegisterAndDelegateResult { bytes32 registrationRoot; IRegistry.SignedRegistration[] registrations; ISlasher.SignedDelegation signedDelegation; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct RegisterAndDelegateResult {
        #[allow(missing_docs)]
        pub registrationRoot: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub registrations: alloy::sol_types::private::Vec<
            <IRegistry::SignedRegistration as alloy::sol_types::SolType>::RustType,
        >,
        #[allow(missing_docs)]
        pub signedDelegation: <ISlasher::SignedDelegation as alloy::sol_types::SolType>::RustType,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::FixedBytes<32>,
            alloy::sol_types::sol_data::Array<IRegistry::SignedRegistration>,
            ISlasher::SignedDelegation,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::FixedBytes<32>,
            alloy::sol_types::private::Vec<
                <IRegistry::SignedRegistration as alloy::sol_types::SolType>::RustType,
            >,
            <ISlasher::SignedDelegation as alloy::sol_types::SolType>::RustType,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<RegisterAndDelegateResult>
        for UnderlyingRustTuple<'_> {
            fn from(value: RegisterAndDelegateResult) -> Self {
                (value.registrationRoot, value.registrations, value.signedDelegation)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for RegisterAndDelegateResult {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    registrationRoot: tuple.0,
                    registrations: tuple.1,
                    signedDelegation: tuple.2,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for RegisterAndDelegateResult {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for RegisterAndDelegateResult {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.registrationRoot),
                    <alloy::sol_types::sol_data::Array<
                        IRegistry::SignedRegistration,
                    > as alloy_sol_types::SolType>::tokenize(&self.registrations),
                    <ISlasher::SignedDelegation as alloy_sol_types::SolType>::tokenize(
                        &self.signedDelegation,
                    ),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(&tuple, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_packed_encoded_size(&tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for RegisterAndDelegateResult {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for RegisterAndDelegateResult {
            const NAME: &'static str = "RegisterAndDelegateResult";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "RegisterAndDelegateResult(bytes32 registrationRoot,IRegistry.SignedRegistration[] registrations,ISlasher.SignedDelegation signedDelegation)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                let mut components = alloy_sol_types::private::Vec::with_capacity(2);
                components
                    .push(
                        <IRegistry::SignedRegistration as alloy_sol_types::SolStruct>::eip712_root_type(),
                    );
                components
                    .extend(
                        <IRegistry::SignedRegistration as alloy_sol_types::SolStruct>::eip712_components(),
                    );
                components
                    .push(
                        <ISlasher::SignedDelegation as alloy_sol_types::SolStruct>::eip712_root_type(),
                    );
                components
                    .extend(
                        <ISlasher::SignedDelegation as alloy_sol_types::SolStruct>::eip712_components(),
                    );
                components
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.registrationRoot,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Array<
                        IRegistry::SignedRegistration,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.registrations)
                        .0,
                    <ISlasher::SignedDelegation as alloy_sol_types::SolType>::eip712_data_word(
                            &self.signedDelegation,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for RegisterAndDelegateResult {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.registrationRoot,
                    )
                    + <alloy::sol_types::sol_data::Array<
                        IRegistry::SignedRegistration,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.registrations,
                    )
                    + <ISlasher::SignedDelegation as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.signedDelegation,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(
                    <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
                );
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.registrationRoot,
                    out,
                );
                <alloy::sol_types::sol_data::Array<
                    IRegistry::SignedRegistration,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.registrations,
                    out,
                );
                <ISlasher::SignedDelegation as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.signedDelegation,
                    out,
                );
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    rust,
                    &mut out,
                );
                alloy_sol_types::abi::token::WordToken(
                    alloy_sol_types::private::keccak256(out),
                )
            }
        }
    };
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`UnitTestHelper`](self) contract instance.

See the [wrapper's documentation](`UnitTestHelperInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> UnitTestHelperInstance<T, P, N> {
        UnitTestHelperInstance::<T, P, N>::new(address, provider)
    }
    /**A [`UnitTestHelper`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`UnitTestHelper`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct UnitTestHelperInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for UnitTestHelperInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("UnitTestHelperInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > UnitTestHelperInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`UnitTestHelper`](self) contract instance.

See the [wrapper's documentation](`UnitTestHelperInstance`) for more details.*/
        #[inline]
        pub const fn new(
            address: alloy_sol_types::private::Address,
            provider: P,
        ) -> Self {
            Self {
                address,
                provider,
                _network_transport: ::core::marker::PhantomData,
            }
        }
        /// Returns a reference to the address.
        #[inline]
        pub const fn address(&self) -> &alloy_sol_types::private::Address {
            &self.address
        }
        /// Sets the address.
        #[inline]
        pub fn set_address(&mut self, address: alloy_sol_types::private::Address) {
            self.address = address;
        }
        /// Sets the address and returns `self`.
        pub fn at(mut self, address: alloy_sol_types::private::Address) -> Self {
            self.set_address(address);
            self
        }
        /// Returns a reference to the provider.
        #[inline]
        pub const fn provider(&self) -> &P {
            &self.provider
        }
    }
    impl<T, P: ::core::clone::Clone, N> UnitTestHelperInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> UnitTestHelperInstance<T, P, N> {
            UnitTestHelperInstance {
                address: self.address,
                provider: ::core::clone::Clone::clone(&self.provider),
                _network_transport: ::core::marker::PhantomData,
            }
        }
    }
    /// Function calls.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > UnitTestHelperInstance<T, P, N> {
        /// Creates a new call builder using this contract instance's provider and address.
        ///
        /// Note that the call can be any function call, not just those defined in this
        /// contract. Prefer using the other methods for building type-safe contract calls.
        pub fn call_builder<C: alloy_sol_types::SolCall>(
            &self,
            call: &C,
        ) -> alloy_contract::SolCallBuilder<T, &P, C, N> {
            alloy_contract::SolCallBuilder::new_sol(&self.provider, &self.address, call)
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > UnitTestHelperInstance<T, P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<T, &P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
    }
}
/**

Generated by the following Solidity interface...
```solidity
library BLS {
    struct G1Point {
        bytes32 x_a;
        bytes32 x_b;
        bytes32 y_a;
        bytes32 y_b;
    }
    struct G2Point {
        bytes32 x_c0_a;
        bytes32 x_c0_b;
        bytes32 x_c1_a;
        bytes32 x_c1_b;
        bytes32 y_c0_a;
        bytes32 y_c0_b;
        bytes32 y_c1_a;
        bytes32 y_c1_b;
    }
}

library IRegistry {
    struct SignedRegistration {
        BLS.G1Point pubkey;
        BLS.G2Point signature;
        uint64 nonce;
    }
}

library ISlasher {
    struct Commitment {
        uint64 commitmentType;
        bytes payload;
        address slasher;
    }
    struct Delegation {
        BLS.G1Point proposer;
        BLS.G1Point delegate;
        address committer;
        uint64 slot;
        bytes metadata;
    }
    struct SignedCommitment {
        Commitment commitment;
        bytes signature;
    }
    struct SignedDelegation {
        Delegation delegation;
        uint64 nonce;
        bytes32 signingId;
        BLS.G2Point signature;
    }
}

library UnitTestHelper {
    struct RegisterAndDelegateParams {
        uint256 proposerSecretKey;
        uint256 collateral;
        address owner;
        uint256 delegateSecretKey;
        uint256 committerSecretKey;
        address committer;
        address slasher;
        bytes metadata;
        uint64 slot;
        bytes32 signingId;
        uint64 nonce;
    }
    struct RegisterAndDelegateResult {
        bytes32 registrationRoot;
        IRegistry.SignedRegistration[] registrations;
        ISlasher.SignedDelegation signedDelegation;
    }
}

interface ReentrantRegistrationContract {
    constructor(address registryAddress);

    receive() external payable;

    function claimCollateral() external;
    function collateral() external view returns (uint256);
    function errors() external view returns (uint256);
    function register(IRegistry.SignedRegistration[] memory _registrations, bytes32 signingId) external;
    function registrationRoot() external view returns (bytes32);
    function registry() external view returns (address);
    function saveResult(UnitTestHelper.RegisterAndDelegateParams memory _params, UnitTestHelper.RegisterAndDelegateResult memory _result, ISlasher.SignedCommitment memory _signedCommitment, ISlasher.SignedDelegation memory _signedDelegationTwo) external;
    function unregister() external;
}
```

...which was generated by the following JSON ABI:
```json
[
  {
    "type": "constructor",
    "inputs": [
      {
        "name": "registryAddress",
        "type": "address",
        "internalType": "address"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "receive",
    "stateMutability": "payable"
  },
  {
    "type": "function",
    "name": "claimCollateral",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "collateral",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "errors",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "register",
    "inputs": [
      {
        "name": "_registrations",
        "type": "tuple[]",
        "internalType": "struct IRegistry.SignedRegistration[]",
        "components": [
          {
            "name": "pubkey",
            "type": "tuple",
            "internalType": "struct BLS.G1Point",
            "components": [
              {
                "name": "x_a",
                "type": "bytes32",
                "internalType": "bytes32"
              },
              {
                "name": "x_b",
                "type": "bytes32",
                "internalType": "bytes32"
              },
              {
                "name": "y_a",
                "type": "bytes32",
                "internalType": "bytes32"
              },
              {
                "name": "y_b",
                "type": "bytes32",
                "internalType": "bytes32"
              }
            ]
          },
          {
            "name": "signature",
            "type": "tuple",
            "internalType": "struct BLS.G2Point",
            "components": [
              {
                "name": "x_c0_a",
                "type": "bytes32",
                "internalType": "bytes32"
              },
              {
                "name": "x_c0_b",
                "type": "bytes32",
                "internalType": "bytes32"
              },
              {
                "name": "x_c1_a",
                "type": "bytes32",
                "internalType": "bytes32"
              },
              {
                "name": "x_c1_b",
                "type": "bytes32",
                "internalType": "bytes32"
              },
              {
                "name": "y_c0_a",
                "type": "bytes32",
                "internalType": "bytes32"
              },
              {
                "name": "y_c0_b",
                "type": "bytes32",
                "internalType": "bytes32"
              },
              {
                "name": "y_c1_a",
                "type": "bytes32",
                "internalType": "bytes32"
              },
              {
                "name": "y_c1_b",
                "type": "bytes32",
                "internalType": "bytes32"
              }
            ]
          },
          {
            "name": "nonce",
            "type": "uint64",
            "internalType": "uint64"
          }
        ]
      },
      {
        "name": "signingId",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "registrationRoot",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "registry",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract IRegistry"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "saveResult",
    "inputs": [
      {
        "name": "_params",
        "type": "tuple",
        "internalType": "struct UnitTestHelper.RegisterAndDelegateParams",
        "components": [
          {
            "name": "proposerSecretKey",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "collateral",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "owner",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "delegateSecretKey",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "committerSecretKey",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "committer",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "slasher",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "metadata",
            "type": "bytes",
            "internalType": "bytes"
          },
          {
            "name": "slot",
            "type": "uint64",
            "internalType": "uint64"
          },
          {
            "name": "signingId",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "nonce",
            "type": "uint64",
            "internalType": "uint64"
          }
        ]
      },
      {
        "name": "_result",
        "type": "tuple",
        "internalType": "struct UnitTestHelper.RegisterAndDelegateResult",
        "components": [
          {
            "name": "registrationRoot",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "registrations",
            "type": "tuple[]",
            "internalType": "struct IRegistry.SignedRegistration[]",
            "components": [
              {
                "name": "pubkey",
                "type": "tuple",
                "internalType": "struct BLS.G1Point",
                "components": [
                  {
                    "name": "x_a",
                    "type": "bytes32",
                    "internalType": "bytes32"
                  },
                  {
                    "name": "x_b",
                    "type": "bytes32",
                    "internalType": "bytes32"
                  },
                  {
                    "name": "y_a",
                    "type": "bytes32",
                    "internalType": "bytes32"
                  },
                  {
                    "name": "y_b",
                    "type": "bytes32",
                    "internalType": "bytes32"
                  }
                ]
              },
              {
                "name": "signature",
                "type": "tuple",
                "internalType": "struct BLS.G2Point",
                "components": [
                  {
                    "name": "x_c0_a",
                    "type": "bytes32",
                    "internalType": "bytes32"
                  },
                  {
                    "name": "x_c0_b",
                    "type": "bytes32",
                    "internalType": "bytes32"
                  },
                  {
                    "name": "x_c1_a",
                    "type": "bytes32",
                    "internalType": "bytes32"
                  },
                  {
                    "name": "x_c1_b",
                    "type": "bytes32",
                    "internalType": "bytes32"
                  },
                  {
                    "name": "y_c0_a",
                    "type": "bytes32",
                    "internalType": "bytes32"
                  },
                  {
                    "name": "y_c0_b",
                    "type": "bytes32",
                    "internalType": "bytes32"
                  },
                  {
                    "name": "y_c1_a",
                    "type": "bytes32",
                    "internalType": "bytes32"
                  },
                  {
                    "name": "y_c1_b",
                    "type": "bytes32",
                    "internalType": "bytes32"
                  }
                ]
              },
              {
                "name": "nonce",
                "type": "uint64",
                "internalType": "uint64"
              }
            ]
          },
          {
            "name": "signedDelegation",
            "type": "tuple",
            "internalType": "struct ISlasher.SignedDelegation",
            "components": [
              {
                "name": "delegation",
                "type": "tuple",
                "internalType": "struct ISlasher.Delegation",
                "components": [
                  {
                    "name": "proposer",
                    "type": "tuple",
                    "internalType": "struct BLS.G1Point",
                    "components": [
                      {
                        "name": "x_a",
                        "type": "bytes32",
                        "internalType": "bytes32"
                      },
                      {
                        "name": "x_b",
                        "type": "bytes32",
                        "internalType": "bytes32"
                      },
                      {
                        "name": "y_a",
                        "type": "bytes32",
                        "internalType": "bytes32"
                      },
                      {
                        "name": "y_b",
                        "type": "bytes32",
                        "internalType": "bytes32"
                      }
                    ]
                  },
                  {
                    "name": "delegate",
                    "type": "tuple",
                    "internalType": "struct BLS.G1Point",
                    "components": [
                      {
                        "name": "x_a",
                        "type": "bytes32",
                        "internalType": "bytes32"
                      },
                      {
                        "name": "x_b",
                        "type": "bytes32",
                        "internalType": "bytes32"
                      },
                      {
                        "name": "y_a",
                        "type": "bytes32",
                        "internalType": "bytes32"
                      },
                      {
                        "name": "y_b",
                        "type": "bytes32",
                        "internalType": "bytes32"
                      }
                    ]
                  },
                  {
                    "name": "committer",
                    "type": "address",
                    "internalType": "address"
                  },
                  {
                    "name": "slot",
                    "type": "uint64",
                    "internalType": "uint64"
                  },
                  {
                    "name": "metadata",
                    "type": "bytes",
                    "internalType": "bytes"
                  }
                ]
              },
              {
                "name": "nonce",
                "type": "uint64",
                "internalType": "uint64"
              },
              {
                "name": "signingId",
                "type": "bytes32",
                "internalType": "bytes32"
              },
              {
                "name": "signature",
                "type": "tuple",
                "internalType": "struct BLS.G2Point",
                "components": [
                  {
                    "name": "x_c0_a",
                    "type": "bytes32",
                    "internalType": "bytes32"
                  },
                  {
                    "name": "x_c0_b",
                    "type": "bytes32",
                    "internalType": "bytes32"
                  },
                  {
                    "name": "x_c1_a",
                    "type": "bytes32",
                    "internalType": "bytes32"
                  },
                  {
                    "name": "x_c1_b",
                    "type": "bytes32",
                    "internalType": "bytes32"
                  },
                  {
                    "name": "y_c0_a",
                    "type": "bytes32",
                    "internalType": "bytes32"
                  },
                  {
                    "name": "y_c0_b",
                    "type": "bytes32",
                    "internalType": "bytes32"
                  },
                  {
                    "name": "y_c1_a",
                    "type": "bytes32",
                    "internalType": "bytes32"
                  },
                  {
                    "name": "y_c1_b",
                    "type": "bytes32",
                    "internalType": "bytes32"
                  }
                ]
              }
            ]
          }
        ]
      },
      {
        "name": "_signedCommitment",
        "type": "tuple",
        "internalType": "struct ISlasher.SignedCommitment",
        "components": [
          {
            "name": "commitment",
            "type": "tuple",
            "internalType": "struct ISlasher.Commitment",
            "components": [
              {
                "name": "commitmentType",
                "type": "uint64",
                "internalType": "uint64"
              },
              {
                "name": "payload",
                "type": "bytes",
                "internalType": "bytes"
              },
              {
                "name": "slasher",
                "type": "address",
                "internalType": "address"
              }
            ]
          },
          {
            "name": "signature",
            "type": "bytes",
            "internalType": "bytes"
          }
        ]
      },
      {
        "name": "_signedDelegationTwo",
        "type": "tuple",
        "internalType": "struct ISlasher.SignedDelegation",
        "components": [
          {
            "name": "delegation",
            "type": "tuple",
            "internalType": "struct ISlasher.Delegation",
            "components": [
              {
                "name": "proposer",
                "type": "tuple",
                "internalType": "struct BLS.G1Point",
                "components": [
                  {
                    "name": "x_a",
                    "type": "bytes32",
                    "internalType": "bytes32"
                  },
                  {
                    "name": "x_b",
                    "type": "bytes32",
                    "internalType": "bytes32"
                  },
                  {
                    "name": "y_a",
                    "type": "bytes32",
                    "internalType": "bytes32"
                  },
                  {
                    "name": "y_b",
                    "type": "bytes32",
                    "internalType": "bytes32"
                  }
                ]
              },
              {
                "name": "delegate",
                "type": "tuple",
                "internalType": "struct BLS.G1Point",
                "components": [
                  {
                    "name": "x_a",
                    "type": "bytes32",
                    "internalType": "bytes32"
                  },
                  {
                    "name": "x_b",
                    "type": "bytes32",
                    "internalType": "bytes32"
                  },
                  {
                    "name": "y_a",
                    "type": "bytes32",
                    "internalType": "bytes32"
                  },
                  {
                    "name": "y_b",
                    "type": "bytes32",
                    "internalType": "bytes32"
                  }
                ]
              },
              {
                "name": "committer",
                "type": "address",
                "internalType": "address"
              },
              {
                "name": "slot",
                "type": "uint64",
                "internalType": "uint64"
              },
              {
                "name": "metadata",
                "type": "bytes",
                "internalType": "bytes"
              }
            ]
          },
          {
            "name": "nonce",
            "type": "uint64",
            "internalType": "uint64"
          },
          {
            "name": "signingId",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "signature",
            "type": "tuple",
            "internalType": "struct BLS.G2Point",
            "components": [
              {
                "name": "x_c0_a",
                "type": "bytes32",
                "internalType": "bytes32"
              },
              {
                "name": "x_c0_b",
                "type": "bytes32",
                "internalType": "bytes32"
              },
              {
                "name": "x_c1_a",
                "type": "bytes32",
                "internalType": "bytes32"
              },
              {
                "name": "x_c1_b",
                "type": "bytes32",
                "internalType": "bytes32"
              },
              {
                "name": "y_c0_a",
                "type": "bytes32",
                "internalType": "bytes32"
              },
              {
                "name": "y_c0_b",
                "type": "bytes32",
                "internalType": "bytes32"
              },
              {
                "name": "y_c1_a",
                "type": "bytes32",
                "internalType": "bytes32"
              },
              {
                "name": "y_c1_b",
                "type": "bytes32",
                "internalType": "bytes32"
              }
            ]
          }
        ]
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "unregister",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  }
]
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod ReentrantRegistrationContract {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x6080604052671bc16d674ec80000600155348015601a575f80fd5b506040516115aa3803806115aa833981016040819052603791605a565b5f80546001600160a01b0319166001600160a01b03929092169190911790556085565b5f602082840312156069575f80fd5b81516001600160a01b0381168114607e575f80fd5b9392505050565b611518806100925f395ff3fe60806040526004361061007c575f3560e01c80637b1039991161004c5780637b103999146104475780637e3780ac1461047d578063d8dfeb4514610492578063e79a198f146104a7575f80fd5b80631f3fc639146103cd57806321e97b09146103ec5780636471a8551461040b5780636f0b0c1c14610433575f80fd5b366103c9575f5460025460405163791ea01360e11b815260048101919091526001600160a01b039091169063f23d40269034906024015f604051808303818588803b1580156100c9575f80fd5b505af1935050505080156100db575060015b61012c573d808015610108576040519150601f19603f3d011682016040523d82523d5f602084013e61010d565b606091505b50600160035f8282546101209190610b8a565b90915550610185915050565b60405162461bcd60e51b8152602060048201526024808201527f73686f756c64206e6f742062652061626c6520746f2061646420636f6c6c6174604482015263195c985b60e21b60648201526084015b60405180910390fd5b5f54600254604051630682467760e21b81526001600160a01b0390921691631a0919dc916101b99160040190815260200190565b5f604051808303815f87803b1580156101d0575f80fd5b505af19250505080156101e1575060015b610232573d80801561020e576040519150601f19603f3d011682016040523d82523d5f602084013e610213565b606091505b50600160035f8282546102269190610b8a565b9091555061027a915050565b60405162461bcd60e51b815260206004820181905260248201527f73686f756c64206e6f742062652061626c6520746f20756e7265676973746572604482015260640161017c565b5f5460025460405163e3fc028d60e01b81526001600160a01b039092169163e3fc028d916102ae9160040190815260200190565b5f604051808303815f87803b1580156102c5575f80fd5b505af19250505080156102d6575060015b610327573d808015610303576040519150601f19603f3d011682016040523d82523d5f602084013e610308565b606091505b50600160035f82825461031b9190610b8a565b9091555061037e915050565b60405162461bcd60e51b815260206004820152602660248201527f73686f756c64206e6f742062652061626c6520746f20636c61696d20636f6c6c604482015265185d195c985b60d21b606482015260840161017c565b6003546003146103c75760405162461bcd60e51b815260206004820152601460248201527373686f756c6420686176652033206572726f727360601b604482015260640161017c565b005b5f80fd5b3480156103d8575f80fd5b506103c76103e7366004610e50565b6104bb565b3480156103f7575f80fd5b506103c761040636600461112a565b610677565b348015610416575f80fd5b5061042060035481565b6040519081526020015b60405180910390f35b34801561043e575f80fd5b506103c7610af3565b348015610452575f80fd5b505f54610465906001600160a01b031681565b6040516001600160a01b03909116815260200161042a565b348015610488575f80fd5b5061042060025481565b34801561049d575f80fd5b5061042060015481565b3480156104b2575f80fd5b506103c7610b56565b815160011461051e5760405162461bcd60e51b815260206004820152602960248201527f74657374206861726e65737320737570706f727473206f6e6c7920312072656760448201526834b9ba3930ba34b7b760b91b606482015260840161017c565b815f8151811061053057610530611286565b602002602001015160235f6001811061054b5761054b611286565b82518051600d9290920292909201908155602080830151600180840191909155604080850151600285015560609485015160038501558286015180516004808701919091559381015160058601558082015160068601559485015160078501556080850151600885015560a0850151600985015560c0850151600a85015560e090940151600b84015593830151600c909201805467ffffffffffffffff19166001600160401b03909316929092179091555f549254915163dce4369560e01b81526001600160a01b039093169263dce43695929161062f918791309188910161129a565b60206040518083038185885af115801561064b573d5f803e3d5ffd5b50505050506040513d601f19601f820116820180604052508101906106709190611388565b6002555050565b8351600490815560208501516005556040850151600680546001600160a01b039283166001600160a01b0319918216179091556060870151600755608087015160085560a08701516009805491841691831691909117905560c0870151600a805491909316911617905560e0850151859190600b906106f69082611423565b506101008201516008820180546001600160401b0392831667ffffffffffffffff1991821617909155610120840151600984015561014090930151600a90920180549282169290931691909117909155604080850151805180518051600f908155602080830151601055828601516011556060928301516012558084015180516013559081015160145580860151601555820151601655938201516017805492840151909616600160a01b026001600160e01b03199092166001600160a01b039091161717909355608083015190929082906018906107d59082611423565b505050602082810151600a8301805467ffffffffffffffff19166001600160401b03909216919091179055604080840151600b8401556060938401518051600c85015591820151600d840155810151600e83015591820151600f8201556080820151601082015560a0820151601182015560c0820151601282015560e0909101516013909101555f5b836020015151811015610950578360200151818151811061088157610881611286565b60200260200101516023826001811061089c5761089c611286565b82518051600d92909202929092019081556020808301516001808401919091556040808501516002850155606094850151600385015582860151805160048601559283015160058501558281015160068501559382015160078401556080820151600884015560a0820151600984015560c0820151600a84015560e090910151600b8301559190920151600c909201805467ffffffffffffffff19166001600160401b03909316929092179091550161085e565b50815180516031805467ffffffffffffffff19166001600160401b039092169190911781556020820151849290829060329061098c9082611423565b5060409190910151600290910180546001600160a01b0319166001600160a01b03909216919091179055602082015160038201906109ca9082611423565b5050815180518051603590815560208083015160365560408084015160375560609384015160385581850151805160395591820151603a5581810151603b5590830151603c55830151603d8054938501516001600160401b0316600160a01b026001600160e01b03199094166001600160a01b03909216919091179290921790915560808201518493509091908290603e90610a669082611423565b505050602082810151600a8301805467ffffffffffffffff19166001600160401b03909216919091179055604080840151600b8401556060938401518051600c85015591820151600d840155810151600e83015591820151600f8201556080820151601082015560a0820151601182015560c0820151601282015560e09091015160139091015550505050565b5f5460025460405163e3fc028d60e01b81526001600160a01b039092169163e3fc028d91610b279160040190815260200190565b5f604051808303815f87803b158015610b3e575f80fd5b505af1158015610b50573d5f803e3d5ffd5b50505050565b5f54600254604051630682467760e21b81526001600160a01b0390921691631a0919dc91610b279160040190815260200190565b80820180821115610ba957634e487b7160e01b5f52601160045260245ffd5b92915050565b634e487b7160e01b5f52604160045260245ffd5b604051608081016001600160401b0381118282101715610be557610be5610baf565b60405290565b604051606081016001600160401b0381118282101715610be557610be5610baf565b60405160a081016001600160401b0381118282101715610be557610be5610baf565b604080519081016001600160401b0381118282101715610be557610be5610baf565b60405161016081016001600160401b0381118282101715610be557610be5610baf565b604051601f8201601f191681016001600160401b0381118282101715610c9c57610c9c610baf565b604052919050565b5f60808284031215610cb4575f80fd5b610cbc610bc3565b90508135815260208201356020820152604082013560408201526060820135606082015292915050565b5f610100808385031215610cf8575f80fd5b604051908101906001600160401b0382118183101715610d1a57610d1a610baf565b81604052809250833581526020840135602082015260408401356040820152606084013560608201526080840135608082015260a084013560a082015260c084013560c082015260e084013560e0820152505092915050565b80356001600160401b0381168114610d89575f80fd5b919050565b5f82601f830112610d9d575f80fd5b813560206001600160401b03821115610db857610db8610baf565b610dc6818360051b01610c74565b8281526101a09283028501820192828201919087851115610de5575f80fd5b8387015b85811015610e435781818a031215610dff575f80fd5b610e07610beb565b610e118a83610ca4565b8152610e208a60808401610ce6565b86820152610e316101808301610d73565b60408201528452928401928101610de9565b5090979650505050505050565b5f8060408385031215610e61575f80fd5b82356001600160401b03811115610e76575f80fd5b610e8285828601610d8e565b95602094909401359450505050565b80356001600160a01b0381168114610d89575f80fd5b5f82601f830112610eb6575f80fd5b81356001600160401b03811115610ecf57610ecf610baf565b610ee2601f8201601f1916602001610c74565b818152846020838601011115610ef6575f80fd5b816020850160208301375f918101602001919091529392505050565b5f610160808385031215610f24575f80fd5b610f2c610bc3565b915082356001600160401b0380821115610f44575f80fd5b8185019150828287031215610f57575f80fd5b610f5f610c0d565b9250610f6b8683610ca4565b8352610f7a8660808401610ca4565b6020840152610f8c6101008301610e91565b6040840152610f9e6101208301610d73565b606084015261014082013581811115610fb5575f80fd5b610fc187828501610ea7565b60808501525050508152610fd760208301610d73565b602082015260408201356040820152610ff38360608401610ce6565b606082015292915050565b5f6060828403121561100e575f80fd5b611016610beb565b90508135815260208201356001600160401b0380821115611035575f80fd5b61104185838601610d8e565b60208401526040840135915080821115611059575f80fd5b5061106684828501610f12565b60408301525092915050565b5f60408284031215611082575f80fd5b61108a610c2f565b905081356001600160401b03808211156110a2575f80fd5b90830190606082860312156110b5575f80fd5b6110bd610beb565b6110c683610d73565b81526020830135828111156110d9575f80fd5b6110e587828601610ea7565b6020830152506110f760408401610e91565b604082015283526020840135915080821115611111575f80fd5b5061111e84828501610ea7565b60208301525092915050565b5f805f806080858703121561113d575f80fd5b84356001600160401b0380821115611153575f80fd5b908601906101608289031215611167575f80fd5b61116f610c51565b823581526020830135602082015261118960408401610e91565b604082015260608301356060820152608083013560808201526111ae60a08401610e91565b60a08201526111bf60c08401610e91565b60c082015260e0830135828111156111d5575f80fd5b6111e18a828601610ea7565b60e0830152506101006111f5818501610d73565b908201526101208381013590820152610140611212818501610d73565b908201529550602087013591508082111561122b575f80fd5b61123788838901610ffe565b9450604087013591508082111561124c575f80fd5b61125888838901611072565b9350606087013591508082111561126d575f80fd5b5061127a87828801610f12565b91505092959194509250565b634e487b7160e01b5f52603260045260245ffd5b606080825284518282018190525f9190608090818501906020808a01865b8381101561135c5781518051805187528481015185880152604080820151818901529089015189880152848201518051898901528086015160a0808a01919091528183015160c0808b0191909152828c015160e0808c01919091528b8401516101008c0152918301516101208b01528201516101408a0152015161016088015201516001600160401b03166101808601526101a090940193908201906001016112b8565b50508295506113758188018a6001600160a01b03169052565b5050505050826040830152949350505050565b5f60208284031215611398575f80fd5b5051919050565b600181811c908216806113b357607f821691505b6020821081036113d157634e487b7160e01b5f52602260045260245ffd5b50919050565b601f82111561141e57805f5260205f20601f840160051c810160208510156113fc5750805b601f840160051c820191505b8181101561141b575f8155600101611408565b50505b505050565b81516001600160401b0381111561143c5761143c610baf565b6114508161144a845461139f565b846113d7565b602080601f831160018114611483575f841561146c5750858301515b5f19600386901b1c1916600185901b1785556114da565b5f85815260208120601f198616915b828110156114b157888601518255948401946001909101908401611492565b50858210156114ce57878501515f19600388901b60f8161c191681555b505060018460011b0185555b50505050505056fea264697066735822122084d43e46cbd93ddb842c06bdf6299de303f30fc37a9736ee25adf50614ce056164736f6c63430008190033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@Rg\x1B\xC1mgN\xC8\0\0`\x01U4\x80\x15`\x1AW_\x80\xFD[P`@Qa\x15\xAA8\x03\x80a\x15\xAA\x839\x81\x01`@\x81\x90R`7\x91`ZV[_\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90U`\x85V[_` \x82\x84\x03\x12\x15`iW_\x80\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14`~W_\x80\xFD[\x93\x92PPPV[a\x15\x18\x80a\0\x92_9_\xF3\xFE`\x80`@R`\x046\x10a\0|W_5`\xE0\x1C\x80c{\x109\x99\x11a\0LW\x80c{\x109\x99\x14a\x04GW\x80c~7\x80\xAC\x14a\x04}W\x80c\xD8\xDF\xEBE\x14a\x04\x92W\x80c\xE7\x9A\x19\x8F\x14a\x04\xA7W_\x80\xFD[\x80c\x1F?\xC69\x14a\x03\xCDW\x80c!\xE9{\t\x14a\x03\xECW\x80cdq\xA8U\x14a\x04\x0BW\x80co\x0B\x0C\x1C\x14a\x043W_\x80\xFD[6a\x03\xC9W_T`\x02T`@Qcy\x1E\xA0\x13`\xE1\x1B\x81R`\x04\x81\x01\x91\x90\x91R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xF2=@&\x904\x90`$\x01_`@Q\x80\x83\x03\x81\x85\x88\x80;\x15\x80\x15a\0\xC9W_\x80\xFD[PZ\xF1\x93PPPP\x80\x15a\0\xDBWP`\x01[a\x01,W=\x80\x80\x15a\x01\x08W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x01\rV[``\x91P[P`\x01`\x03_\x82\x82Ta\x01 \x91\x90a\x0B\x8AV[\x90\x91UPa\x01\x85\x91PPV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`$\x80\x82\x01R\x7Fshould not be able to add collat`D\x82\x01Rc\x19\\\x98[`\xE2\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[_T`\x02T`@Qc\x06\x82Fw`\xE2\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91c\x1A\t\x19\xDC\x91a\x01\xB9\x91`\x04\x01\x90\x81R` \x01\x90V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x01\xD0W_\x80\xFD[PZ\xF1\x92PPP\x80\x15a\x01\xE1WP`\x01[a\x022W=\x80\x80\x15a\x02\x0EW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x02\x13V[``\x91P[P`\x01`\x03_\x82\x82Ta\x02&\x91\x90a\x0B\x8AV[\x90\x91UPa\x02z\x91PPV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7Fshould not be able to unregister`D\x82\x01R`d\x01a\x01|V[_T`\x02T`@Qc\xE3\xFC\x02\x8D`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91c\xE3\xFC\x02\x8D\x91a\x02\xAE\x91`\x04\x01\x90\x81R` \x01\x90V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x02\xC5W_\x80\xFD[PZ\xF1\x92PPP\x80\x15a\x02\xD6WP`\x01[a\x03'W=\x80\x80\x15a\x03\x03W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x03\x08V[``\x91P[P`\x01`\x03_\x82\x82Ta\x03\x1B\x91\x90a\x0B\x8AV[\x90\x91UPa\x03~\x91PPV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7Fshould not be able to claim coll`D\x82\x01Re\x18]\x19\\\x98[`\xD2\x1B`d\x82\x01R`\x84\x01a\x01|V[`\x03T`\x03\x14a\x03\xC7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01Rsshould have 3 errors``\x1B`D\x82\x01R`d\x01a\x01|V[\0[_\x80\xFD[4\x80\x15a\x03\xD8W_\x80\xFD[Pa\x03\xC7a\x03\xE76`\x04a\x0EPV[a\x04\xBBV[4\x80\x15a\x03\xF7W_\x80\xFD[Pa\x03\xC7a\x04\x066`\x04a\x11*V[a\x06wV[4\x80\x15a\x04\x16W_\x80\xFD[Pa\x04 `\x03T\x81V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x04>W_\x80\xFD[Pa\x03\xC7a\n\xF3V[4\x80\x15a\x04RW_\x80\xFD[P_Ta\x04e\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x04*V[4\x80\x15a\x04\x88W_\x80\xFD[Pa\x04 `\x02T\x81V[4\x80\x15a\x04\x9DW_\x80\xFD[Pa\x04 `\x01T\x81V[4\x80\x15a\x04\xB2W_\x80\xFD[Pa\x03\xC7a\x0BVV[\x81Q`\x01\x14a\x05\x1EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`)`$\x82\x01R\x7Ftest harness supports only 1 reg`D\x82\x01Rh4\xB9\xBA90\xBA4\xB7\xB7`\xB9\x1B`d\x82\x01R`\x84\x01a\x01|V[\x81_\x81Q\x81\x10a\x050Wa\x050a\x12\x86V[` \x02` \x01\x01Q`#_`\x01\x81\x10a\x05KWa\x05Ka\x12\x86V[\x82Q\x80Q`\r\x92\x90\x92\x02\x92\x90\x92\x01\x90\x81U` \x80\x83\x01Q`\x01\x80\x84\x01\x91\x90\x91U`@\x80\x85\x01Q`\x02\x85\x01U``\x94\x85\x01Q`\x03\x85\x01U\x82\x86\x01Q\x80Q`\x04\x80\x87\x01\x91\x90\x91U\x93\x81\x01Q`\x05\x86\x01U\x80\x82\x01Q`\x06\x86\x01U\x94\x85\x01Q`\x07\x85\x01U`\x80\x85\x01Q`\x08\x85\x01U`\xA0\x85\x01Q`\t\x85\x01U`\xC0\x85\x01Q`\n\x85\x01U`\xE0\x90\x94\x01Q`\x0B\x84\x01U\x93\x83\x01Q`\x0C\x90\x92\x01\x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`@\x1B\x03\x90\x93\x16\x92\x90\x92\x17\x90\x91U_T\x92T\x91Qc\xDC\xE46\x95`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x92c\xDC\xE46\x95\x92\x91a\x06/\x91\x87\x910\x91\x88\x91\x01a\x12\x9AV[` `@Q\x80\x83\x03\x81\x85\x88Z\xF1\x15\x80\x15a\x06KW=_\x80>=_\xFD[PPPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06p\x91\x90a\x13\x88V[`\x02UPPV[\x83Q`\x04\x90\x81U` \x85\x01Q`\x05U`@\x85\x01Q`\x06\x80T`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\x01`\x01`\xA0\x1B\x03\x19\x91\x82\x16\x17\x90\x91U``\x87\x01Q`\x07U`\x80\x87\x01Q`\x08U`\xA0\x87\x01Q`\t\x80T\x91\x84\x16\x91\x83\x16\x91\x90\x91\x17\x90U`\xC0\x87\x01Q`\n\x80T\x91\x90\x93\x16\x91\x16\x17\x90U`\xE0\x85\x01Q\x85\x91\x90`\x0B\x90a\x06\xF6\x90\x82a\x14#V[Pa\x01\0\x82\x01Q`\x08\x82\x01\x80T`\x01`\x01`@\x1B\x03\x92\x83\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x91\x82\x16\x17\x90\x91Ua\x01 \x84\x01Q`\t\x84\x01Ua\x01@\x90\x93\x01Q`\n\x90\x92\x01\x80T\x92\x82\x16\x92\x90\x93\x16\x91\x90\x91\x17\x90\x91U`@\x80\x85\x01Q\x80Q\x80Q\x80Q`\x0F\x90\x81U` \x80\x83\x01Q`\x10U\x82\x86\x01Q`\x11U``\x92\x83\x01Q`\x12U\x80\x84\x01Q\x80Q`\x13U\x90\x81\x01Q`\x14U\x80\x86\x01Q`\x15U\x82\x01Q`\x16U\x93\x82\x01Q`\x17\x80T\x92\x84\x01Q\x90\x96\x16`\x01`\xA0\x1B\x02`\x01`\x01`\xE0\x1B\x03\x19\x90\x92\x16`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x17\x17\x90\x93U`\x80\x83\x01Q\x90\x92\x90\x82\x90`\x18\x90a\x07\xD5\x90\x82a\x14#V[PPP` \x82\x81\x01Q`\n\x83\x01\x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`@\x1B\x03\x90\x92\x16\x91\x90\x91\x17\x90U`@\x80\x84\x01Q`\x0B\x84\x01U``\x93\x84\x01Q\x80Q`\x0C\x85\x01U\x91\x82\x01Q`\r\x84\x01U\x81\x01Q`\x0E\x83\x01U\x91\x82\x01Q`\x0F\x82\x01U`\x80\x82\x01Q`\x10\x82\x01U`\xA0\x82\x01Q`\x11\x82\x01U`\xC0\x82\x01Q`\x12\x82\x01U`\xE0\x90\x91\x01Q`\x13\x90\x91\x01U_[\x83` \x01QQ\x81\x10\x15a\tPW\x83` \x01Q\x81\x81Q\x81\x10a\x08\x81Wa\x08\x81a\x12\x86V[` \x02` \x01\x01Q`#\x82`\x01\x81\x10a\x08\x9CWa\x08\x9Ca\x12\x86V[\x82Q\x80Q`\r\x92\x90\x92\x02\x92\x90\x92\x01\x90\x81U` \x80\x83\x01Q`\x01\x80\x84\x01\x91\x90\x91U`@\x80\x85\x01Q`\x02\x85\x01U``\x94\x85\x01Q`\x03\x85\x01U\x82\x86\x01Q\x80Q`\x04\x86\x01U\x92\x83\x01Q`\x05\x85\x01U\x82\x81\x01Q`\x06\x85\x01U\x93\x82\x01Q`\x07\x84\x01U`\x80\x82\x01Q`\x08\x84\x01U`\xA0\x82\x01Q`\t\x84\x01U`\xC0\x82\x01Q`\n\x84\x01U`\xE0\x90\x91\x01Q`\x0B\x83\x01U\x91\x90\x92\x01Q`\x0C\x90\x92\x01\x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`@\x1B\x03\x90\x93\x16\x92\x90\x92\x17\x90\x91U\x01a\x08^V[P\x81Q\x80Q`1\x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`@\x1B\x03\x90\x92\x16\x91\x90\x91\x17\x81U` \x82\x01Q\x84\x92\x90\x82\x90`2\x90a\t\x8C\x90\x82a\x14#V[P`@\x91\x90\x91\x01Q`\x02\x90\x91\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x90\x91\x17\x90U` \x82\x01Q`\x03\x82\x01\x90a\t\xCA\x90\x82a\x14#V[PP\x81Q\x80Q\x80Q`5\x90\x81U` \x80\x83\x01Q`6U`@\x80\x84\x01Q`7U``\x93\x84\x01Q`8U\x81\x85\x01Q\x80Q`9U\x91\x82\x01Q`:U\x81\x81\x01Q`;U\x90\x83\x01Q`<U\x83\x01Q`=\x80T\x93\x85\x01Q`\x01`\x01`@\x1B\x03\x16`\x01`\xA0\x1B\x02`\x01`\x01`\xE0\x1B\x03\x19\x90\x94\x16`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x90\x91\x17\x92\x90\x92\x17\x90\x91U`\x80\x82\x01Q\x84\x93P\x90\x91\x90\x82\x90`>\x90a\nf\x90\x82a\x14#V[PPP` \x82\x81\x01Q`\n\x83\x01\x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`@\x1B\x03\x90\x92\x16\x91\x90\x91\x17\x90U`@\x80\x84\x01Q`\x0B\x84\x01U``\x93\x84\x01Q\x80Q`\x0C\x85\x01U\x91\x82\x01Q`\r\x84\x01U\x81\x01Q`\x0E\x83\x01U\x91\x82\x01Q`\x0F\x82\x01U`\x80\x82\x01Q`\x10\x82\x01U`\xA0\x82\x01Q`\x11\x82\x01U`\xC0\x82\x01Q`\x12\x82\x01U`\xE0\x90\x91\x01Q`\x13\x90\x91\x01UPPPPV[_T`\x02T`@Qc\xE3\xFC\x02\x8D`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91c\xE3\xFC\x02\x8D\x91a\x0B'\x91`\x04\x01\x90\x81R` \x01\x90V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x0B>W_\x80\xFD[PZ\xF1\x15\x80\x15a\x0BPW=_\x80>=_\xFD[PPPPV[_T`\x02T`@Qc\x06\x82Fw`\xE2\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91c\x1A\t\x19\xDC\x91a\x0B'\x91`\x04\x01\x90\x81R` \x01\x90V[\x80\x82\x01\x80\x82\x11\x15a\x0B\xA9WcNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x92\x91PPV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@Q`\x80\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x0B\xE5Wa\x0B\xE5a\x0B\xAFV[`@R\x90V[`@Q``\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x0B\xE5Wa\x0B\xE5a\x0B\xAFV[`@Q`\xA0\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x0B\xE5Wa\x0B\xE5a\x0B\xAFV[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x0B\xE5Wa\x0B\xE5a\x0B\xAFV[`@Qa\x01`\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x0B\xE5Wa\x0B\xE5a\x0B\xAFV[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x0C\x9CWa\x0C\x9Ca\x0B\xAFV[`@R\x91\x90PV[_`\x80\x82\x84\x03\x12\x15a\x0C\xB4W_\x80\xFD[a\x0C\xBCa\x0B\xC3V[\x90P\x815\x81R` \x82\x015` \x82\x01R`@\x82\x015`@\x82\x01R``\x82\x015``\x82\x01R\x92\x91PPV[_a\x01\0\x80\x83\x85\x03\x12\x15a\x0C\xF8W_\x80\xFD[`@Q\x90\x81\x01\x90`\x01`\x01`@\x1B\x03\x82\x11\x81\x83\x10\x17\x15a\r\x1AWa\r\x1Aa\x0B\xAFV[\x81`@R\x80\x92P\x835\x81R` \x84\x015` \x82\x01R`@\x84\x015`@\x82\x01R``\x84\x015``\x82\x01R`\x80\x84\x015`\x80\x82\x01R`\xA0\x84\x015`\xA0\x82\x01R`\xC0\x84\x015`\xC0\x82\x01R`\xE0\x84\x015`\xE0\x82\x01RPP\x92\x91PPV[\x805`\x01`\x01`@\x1B\x03\x81\x16\x81\x14a\r\x89W_\x80\xFD[\x91\x90PV[_\x82`\x1F\x83\x01\x12a\r\x9DW_\x80\xFD[\x815` `\x01`\x01`@\x1B\x03\x82\x11\x15a\r\xB8Wa\r\xB8a\x0B\xAFV[a\r\xC6\x81\x83`\x05\x1B\x01a\x0CtV[\x82\x81Ra\x01\xA0\x92\x83\x02\x85\x01\x82\x01\x92\x82\x82\x01\x91\x90\x87\x85\x11\x15a\r\xE5W_\x80\xFD[\x83\x87\x01[\x85\x81\x10\x15a\x0ECW\x81\x81\x8A\x03\x12\x15a\r\xFFW_\x80\xFD[a\x0E\x07a\x0B\xEBV[a\x0E\x11\x8A\x83a\x0C\xA4V[\x81Ra\x0E \x8A`\x80\x84\x01a\x0C\xE6V[\x86\x82\x01Ra\x0E1a\x01\x80\x83\x01a\rsV[`@\x82\x01R\x84R\x92\x84\x01\x92\x81\x01a\r\xE9V[P\x90\x97\x96PPPPPPPV[_\x80`@\x83\x85\x03\x12\x15a\x0EaW_\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0EvW_\x80\xFD[a\x0E\x82\x85\x82\x86\x01a\r\x8EV[\x95` \x94\x90\x94\x015\x94PPPPV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\r\x89W_\x80\xFD[_\x82`\x1F\x83\x01\x12a\x0E\xB6W_\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0E\xCFWa\x0E\xCFa\x0B\xAFV[a\x0E\xE2`\x1F\x82\x01`\x1F\x19\x16` \x01a\x0CtV[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\x0E\xF6W_\x80\xFD[\x81` \x85\x01` \x83\x017_\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[_a\x01`\x80\x83\x85\x03\x12\x15a\x0F$W_\x80\xFD[a\x0F,a\x0B\xC3V[\x91P\x825`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x0FDW_\x80\xFD[\x81\x85\x01\x91P\x82\x82\x87\x03\x12\x15a\x0FWW_\x80\xFD[a\x0F_a\x0C\rV[\x92Pa\x0Fk\x86\x83a\x0C\xA4V[\x83Ra\x0Fz\x86`\x80\x84\x01a\x0C\xA4V[` \x84\x01Ra\x0F\x8Ca\x01\0\x83\x01a\x0E\x91V[`@\x84\x01Ra\x0F\x9Ea\x01 \x83\x01a\rsV[``\x84\x01Ra\x01@\x82\x015\x81\x81\x11\x15a\x0F\xB5W_\x80\xFD[a\x0F\xC1\x87\x82\x85\x01a\x0E\xA7V[`\x80\x85\x01RPPP\x81Ra\x0F\xD7` \x83\x01a\rsV[` \x82\x01R`@\x82\x015`@\x82\x01Ra\x0F\xF3\x83``\x84\x01a\x0C\xE6V[``\x82\x01R\x92\x91PPV[_``\x82\x84\x03\x12\x15a\x10\x0EW_\x80\xFD[a\x10\x16a\x0B\xEBV[\x90P\x815\x81R` \x82\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x105W_\x80\xFD[a\x10A\x85\x83\x86\x01a\r\x8EV[` \x84\x01R`@\x84\x015\x91P\x80\x82\x11\x15a\x10YW_\x80\xFD[Pa\x10f\x84\x82\x85\x01a\x0F\x12V[`@\x83\x01RP\x92\x91PPV[_`@\x82\x84\x03\x12\x15a\x10\x82W_\x80\xFD[a\x10\x8Aa\x0C/V[\x90P\x815`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x10\xA2W_\x80\xFD[\x90\x83\x01\x90``\x82\x86\x03\x12\x15a\x10\xB5W_\x80\xFD[a\x10\xBDa\x0B\xEBV[a\x10\xC6\x83a\rsV[\x81R` \x83\x015\x82\x81\x11\x15a\x10\xD9W_\x80\xFD[a\x10\xE5\x87\x82\x86\x01a\x0E\xA7V[` \x83\x01RPa\x10\xF7`@\x84\x01a\x0E\x91V[`@\x82\x01R\x83R` \x84\x015\x91P\x80\x82\x11\x15a\x11\x11W_\x80\xFD[Pa\x11\x1E\x84\x82\x85\x01a\x0E\xA7V[` \x83\x01RP\x92\x91PPV[_\x80_\x80`\x80\x85\x87\x03\x12\x15a\x11=W_\x80\xFD[\x845`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x11SW_\x80\xFD[\x90\x86\x01\x90a\x01`\x82\x89\x03\x12\x15a\x11gW_\x80\xFD[a\x11oa\x0CQV[\x825\x81R` \x83\x015` \x82\x01Ra\x11\x89`@\x84\x01a\x0E\x91V[`@\x82\x01R``\x83\x015``\x82\x01R`\x80\x83\x015`\x80\x82\x01Ra\x11\xAE`\xA0\x84\x01a\x0E\x91V[`\xA0\x82\x01Ra\x11\xBF`\xC0\x84\x01a\x0E\x91V[`\xC0\x82\x01R`\xE0\x83\x015\x82\x81\x11\x15a\x11\xD5W_\x80\xFD[a\x11\xE1\x8A\x82\x86\x01a\x0E\xA7V[`\xE0\x83\x01RPa\x01\0a\x11\xF5\x81\x85\x01a\rsV[\x90\x82\x01Ra\x01 \x83\x81\x015\x90\x82\x01Ra\x01@a\x12\x12\x81\x85\x01a\rsV[\x90\x82\x01R\x95P` \x87\x015\x91P\x80\x82\x11\x15a\x12+W_\x80\xFD[a\x127\x88\x83\x89\x01a\x0F\xFEV[\x94P`@\x87\x015\x91P\x80\x82\x11\x15a\x12LW_\x80\xFD[a\x12X\x88\x83\x89\x01a\x10rV[\x93P``\x87\x015\x91P\x80\x82\x11\x15a\x12mW_\x80\xFD[Pa\x12z\x87\x82\x88\x01a\x0F\x12V[\x91PP\x92\x95\x91\x94P\x92PV[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[``\x80\x82R\x84Q\x82\x82\x01\x81\x90R_\x91\x90`\x80\x90\x81\x85\x01\x90` \x80\x8A\x01\x86[\x83\x81\x10\x15a\x13\\W\x81Q\x80Q\x80Q\x87R\x84\x81\x01Q\x85\x88\x01R`@\x80\x82\x01Q\x81\x89\x01R\x90\x89\x01Q\x89\x88\x01R\x84\x82\x01Q\x80Q\x89\x89\x01R\x80\x86\x01Q`\xA0\x80\x8A\x01\x91\x90\x91R\x81\x83\x01Q`\xC0\x80\x8B\x01\x91\x90\x91R\x82\x8C\x01Q`\xE0\x80\x8C\x01\x91\x90\x91R\x8B\x84\x01Qa\x01\0\x8C\x01R\x91\x83\x01Qa\x01 \x8B\x01R\x82\x01Qa\x01@\x8A\x01R\x01Qa\x01`\x88\x01R\x01Q`\x01`\x01`@\x1B\x03\x16a\x01\x80\x86\x01Ra\x01\xA0\x90\x94\x01\x93\x90\x82\x01\x90`\x01\x01a\x12\xB8V[PP\x82\x95Pa\x13u\x81\x88\x01\x8A`\x01`\x01`\xA0\x1B\x03\x16\x90RV[PPPPP\x82`@\x83\x01R\x94\x93PPPPV[_` \x82\x84\x03\x12\x15a\x13\x98W_\x80\xFD[PQ\x91\x90PV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x13\xB3W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x13\xD1WcNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[P\x91\x90PV[`\x1F\x82\x11\x15a\x14\x1EW\x80_R` _ `\x1F\x84\x01`\x05\x1C\x81\x01` \x85\x10\x15a\x13\xFCWP\x80[`\x1F\x84\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\x14\x1BW_\x81U`\x01\x01a\x14\x08V[PP[PPPV[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x14<Wa\x14<a\x0B\xAFV[a\x14P\x81a\x14J\x84Ta\x13\x9FV[\x84a\x13\xD7V[` \x80`\x1F\x83\x11`\x01\x81\x14a\x14\x83W_\x84\x15a\x14lWP\x85\x83\x01Q[_\x19`\x03\x86\x90\x1B\x1C\x19\x16`\x01\x85\x90\x1B\x17\x85Ua\x14\xDAV[_\x85\x81R` \x81 `\x1F\x19\x86\x16\x91[\x82\x81\x10\x15a\x14\xB1W\x88\x86\x01Q\x82U\x94\x84\x01\x94`\x01\x90\x91\x01\x90\x84\x01a\x14\x92V[P\x85\x82\x10\x15a\x14\xCEW\x87\x85\x01Q_\x19`\x03\x88\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PP`\x01\x84`\x01\x1B\x01\x85U[PPPPPPV\xFE\xA2dipfsX\"\x12 \x84\xD4>F\xCB\xD9=\xDB\x84,\x06\xBD\xF6)\x9D\xE3\x03\xF3\x0F\xC3z\x976\xEE%\xAD\xF5\x06\x14\xCE\x05adsolcC\0\x08\x19\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x60806040526004361061007c575f3560e01c80637b1039991161004c5780637b103999146104475780637e3780ac1461047d578063d8dfeb4514610492578063e79a198f146104a7575f80fd5b80631f3fc639146103cd57806321e97b09146103ec5780636471a8551461040b5780636f0b0c1c14610433575f80fd5b366103c9575f5460025460405163791ea01360e11b815260048101919091526001600160a01b039091169063f23d40269034906024015f604051808303818588803b1580156100c9575f80fd5b505af1935050505080156100db575060015b61012c573d808015610108576040519150601f19603f3d011682016040523d82523d5f602084013e61010d565b606091505b50600160035f8282546101209190610b8a565b90915550610185915050565b60405162461bcd60e51b8152602060048201526024808201527f73686f756c64206e6f742062652061626c6520746f2061646420636f6c6c6174604482015263195c985b60e21b60648201526084015b60405180910390fd5b5f54600254604051630682467760e21b81526001600160a01b0390921691631a0919dc916101b99160040190815260200190565b5f604051808303815f87803b1580156101d0575f80fd5b505af19250505080156101e1575060015b610232573d80801561020e576040519150601f19603f3d011682016040523d82523d5f602084013e610213565b606091505b50600160035f8282546102269190610b8a565b9091555061027a915050565b60405162461bcd60e51b815260206004820181905260248201527f73686f756c64206e6f742062652061626c6520746f20756e7265676973746572604482015260640161017c565b5f5460025460405163e3fc028d60e01b81526001600160a01b039092169163e3fc028d916102ae9160040190815260200190565b5f604051808303815f87803b1580156102c5575f80fd5b505af19250505080156102d6575060015b610327573d808015610303576040519150601f19603f3d011682016040523d82523d5f602084013e610308565b606091505b50600160035f82825461031b9190610b8a565b9091555061037e915050565b60405162461bcd60e51b815260206004820152602660248201527f73686f756c64206e6f742062652061626c6520746f20636c61696d20636f6c6c604482015265185d195c985b60d21b606482015260840161017c565b6003546003146103c75760405162461bcd60e51b815260206004820152601460248201527373686f756c6420686176652033206572726f727360601b604482015260640161017c565b005b5f80fd5b3480156103d8575f80fd5b506103c76103e7366004610e50565b6104bb565b3480156103f7575f80fd5b506103c761040636600461112a565b610677565b348015610416575f80fd5b5061042060035481565b6040519081526020015b60405180910390f35b34801561043e575f80fd5b506103c7610af3565b348015610452575f80fd5b505f54610465906001600160a01b031681565b6040516001600160a01b03909116815260200161042a565b348015610488575f80fd5b5061042060025481565b34801561049d575f80fd5b5061042060015481565b3480156104b2575f80fd5b506103c7610b56565b815160011461051e5760405162461bcd60e51b815260206004820152602960248201527f74657374206861726e65737320737570706f727473206f6e6c7920312072656760448201526834b9ba3930ba34b7b760b91b606482015260840161017c565b815f8151811061053057610530611286565b602002602001015160235f6001811061054b5761054b611286565b82518051600d9290920292909201908155602080830151600180840191909155604080850151600285015560609485015160038501558286015180516004808701919091559381015160058601558082015160068601559485015160078501556080850151600885015560a0850151600985015560c0850151600a85015560e090940151600b84015593830151600c909201805467ffffffffffffffff19166001600160401b03909316929092179091555f549254915163dce4369560e01b81526001600160a01b039093169263dce43695929161062f918791309188910161129a565b60206040518083038185885af115801561064b573d5f803e3d5ffd5b50505050506040513d601f19601f820116820180604052508101906106709190611388565b6002555050565b8351600490815560208501516005556040850151600680546001600160a01b039283166001600160a01b0319918216179091556060870151600755608087015160085560a08701516009805491841691831691909117905560c0870151600a805491909316911617905560e0850151859190600b906106f69082611423565b506101008201516008820180546001600160401b0392831667ffffffffffffffff1991821617909155610120840151600984015561014090930151600a90920180549282169290931691909117909155604080850151805180518051600f908155602080830151601055828601516011556060928301516012558084015180516013559081015160145580860151601555820151601655938201516017805492840151909616600160a01b026001600160e01b03199092166001600160a01b039091161717909355608083015190929082906018906107d59082611423565b505050602082810151600a8301805467ffffffffffffffff19166001600160401b03909216919091179055604080840151600b8401556060938401518051600c85015591820151600d840155810151600e83015591820151600f8201556080820151601082015560a0820151601182015560c0820151601282015560e0909101516013909101555f5b836020015151811015610950578360200151818151811061088157610881611286565b60200260200101516023826001811061089c5761089c611286565b82518051600d92909202929092019081556020808301516001808401919091556040808501516002850155606094850151600385015582860151805160048601559283015160058501558281015160068501559382015160078401556080820151600884015560a0820151600984015560c0820151600a84015560e090910151600b8301559190920151600c909201805467ffffffffffffffff19166001600160401b03909316929092179091550161085e565b50815180516031805467ffffffffffffffff19166001600160401b039092169190911781556020820151849290829060329061098c9082611423565b5060409190910151600290910180546001600160a01b0319166001600160a01b03909216919091179055602082015160038201906109ca9082611423565b5050815180518051603590815560208083015160365560408084015160375560609384015160385581850151805160395591820151603a5581810151603b5590830151603c55830151603d8054938501516001600160401b0316600160a01b026001600160e01b03199094166001600160a01b03909216919091179290921790915560808201518493509091908290603e90610a669082611423565b505050602082810151600a8301805467ffffffffffffffff19166001600160401b03909216919091179055604080840151600b8401556060938401518051600c85015591820151600d840155810151600e83015591820151600f8201556080820151601082015560a0820151601182015560c0820151601282015560e09091015160139091015550505050565b5f5460025460405163e3fc028d60e01b81526001600160a01b039092169163e3fc028d91610b279160040190815260200190565b5f604051808303815f87803b158015610b3e575f80fd5b505af1158015610b50573d5f803e3d5ffd5b50505050565b5f54600254604051630682467760e21b81526001600160a01b0390921691631a0919dc91610b279160040190815260200190565b80820180821115610ba957634e487b7160e01b5f52601160045260245ffd5b92915050565b634e487b7160e01b5f52604160045260245ffd5b604051608081016001600160401b0381118282101715610be557610be5610baf565b60405290565b604051606081016001600160401b0381118282101715610be557610be5610baf565b60405160a081016001600160401b0381118282101715610be557610be5610baf565b604080519081016001600160401b0381118282101715610be557610be5610baf565b60405161016081016001600160401b0381118282101715610be557610be5610baf565b604051601f8201601f191681016001600160401b0381118282101715610c9c57610c9c610baf565b604052919050565b5f60808284031215610cb4575f80fd5b610cbc610bc3565b90508135815260208201356020820152604082013560408201526060820135606082015292915050565b5f610100808385031215610cf8575f80fd5b604051908101906001600160401b0382118183101715610d1a57610d1a610baf565b81604052809250833581526020840135602082015260408401356040820152606084013560608201526080840135608082015260a084013560a082015260c084013560c082015260e084013560e0820152505092915050565b80356001600160401b0381168114610d89575f80fd5b919050565b5f82601f830112610d9d575f80fd5b813560206001600160401b03821115610db857610db8610baf565b610dc6818360051b01610c74565b8281526101a09283028501820192828201919087851115610de5575f80fd5b8387015b85811015610e435781818a031215610dff575f80fd5b610e07610beb565b610e118a83610ca4565b8152610e208a60808401610ce6565b86820152610e316101808301610d73565b60408201528452928401928101610de9565b5090979650505050505050565b5f8060408385031215610e61575f80fd5b82356001600160401b03811115610e76575f80fd5b610e8285828601610d8e565b95602094909401359450505050565b80356001600160a01b0381168114610d89575f80fd5b5f82601f830112610eb6575f80fd5b81356001600160401b03811115610ecf57610ecf610baf565b610ee2601f8201601f1916602001610c74565b818152846020838601011115610ef6575f80fd5b816020850160208301375f918101602001919091529392505050565b5f610160808385031215610f24575f80fd5b610f2c610bc3565b915082356001600160401b0380821115610f44575f80fd5b8185019150828287031215610f57575f80fd5b610f5f610c0d565b9250610f6b8683610ca4565b8352610f7a8660808401610ca4565b6020840152610f8c6101008301610e91565b6040840152610f9e6101208301610d73565b606084015261014082013581811115610fb5575f80fd5b610fc187828501610ea7565b60808501525050508152610fd760208301610d73565b602082015260408201356040820152610ff38360608401610ce6565b606082015292915050565b5f6060828403121561100e575f80fd5b611016610beb565b90508135815260208201356001600160401b0380821115611035575f80fd5b61104185838601610d8e565b60208401526040840135915080821115611059575f80fd5b5061106684828501610f12565b60408301525092915050565b5f60408284031215611082575f80fd5b61108a610c2f565b905081356001600160401b03808211156110a2575f80fd5b90830190606082860312156110b5575f80fd5b6110bd610beb565b6110c683610d73565b81526020830135828111156110d9575f80fd5b6110e587828601610ea7565b6020830152506110f760408401610e91565b604082015283526020840135915080821115611111575f80fd5b5061111e84828501610ea7565b60208301525092915050565b5f805f806080858703121561113d575f80fd5b84356001600160401b0380821115611153575f80fd5b908601906101608289031215611167575f80fd5b61116f610c51565b823581526020830135602082015261118960408401610e91565b604082015260608301356060820152608083013560808201526111ae60a08401610e91565b60a08201526111bf60c08401610e91565b60c082015260e0830135828111156111d5575f80fd5b6111e18a828601610ea7565b60e0830152506101006111f5818501610d73565b908201526101208381013590820152610140611212818501610d73565b908201529550602087013591508082111561122b575f80fd5b61123788838901610ffe565b9450604087013591508082111561124c575f80fd5b61125888838901611072565b9350606087013591508082111561126d575f80fd5b5061127a87828801610f12565b91505092959194509250565b634e487b7160e01b5f52603260045260245ffd5b606080825284518282018190525f9190608090818501906020808a01865b8381101561135c5781518051805187528481015185880152604080820151818901529089015189880152848201518051898901528086015160a0808a01919091528183015160c0808b0191909152828c015160e0808c01919091528b8401516101008c0152918301516101208b01528201516101408a0152015161016088015201516001600160401b03166101808601526101a090940193908201906001016112b8565b50508295506113758188018a6001600160a01b03169052565b5050505050826040830152949350505050565b5f60208284031215611398575f80fd5b5051919050565b600181811c908216806113b357607f821691505b6020821081036113d157634e487b7160e01b5f52602260045260245ffd5b50919050565b601f82111561141e57805f5260205f20601f840160051c810160208510156113fc5750805b601f840160051c820191505b8181101561141b575f8155600101611408565b50505b505050565b81516001600160401b0381111561143c5761143c610baf565b6114508161144a845461139f565b846113d7565b602080601f831160018114611483575f841561146c5750858301515b5f19600386901b1c1916600185901b1785556114da565b5f85815260208120601f198616915b828110156114b157888601518255948401946001909101908401611492565b50858210156114ce57878501515f19600388901b60f8161c191681555b505060018460011b0185555b50505050505056fea264697066735822122084d43e46cbd93ddb842c06bdf6299de303f30fc37a9736ee25adf50614ce056164736f6c63430008190033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`\x046\x10a\0|W_5`\xE0\x1C\x80c{\x109\x99\x11a\0LW\x80c{\x109\x99\x14a\x04GW\x80c~7\x80\xAC\x14a\x04}W\x80c\xD8\xDF\xEBE\x14a\x04\x92W\x80c\xE7\x9A\x19\x8F\x14a\x04\xA7W_\x80\xFD[\x80c\x1F?\xC69\x14a\x03\xCDW\x80c!\xE9{\t\x14a\x03\xECW\x80cdq\xA8U\x14a\x04\x0BW\x80co\x0B\x0C\x1C\x14a\x043W_\x80\xFD[6a\x03\xC9W_T`\x02T`@Qcy\x1E\xA0\x13`\xE1\x1B\x81R`\x04\x81\x01\x91\x90\x91R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xF2=@&\x904\x90`$\x01_`@Q\x80\x83\x03\x81\x85\x88\x80;\x15\x80\x15a\0\xC9W_\x80\xFD[PZ\xF1\x93PPPP\x80\x15a\0\xDBWP`\x01[a\x01,W=\x80\x80\x15a\x01\x08W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x01\rV[``\x91P[P`\x01`\x03_\x82\x82Ta\x01 \x91\x90a\x0B\x8AV[\x90\x91UPa\x01\x85\x91PPV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`$\x80\x82\x01R\x7Fshould not be able to add collat`D\x82\x01Rc\x19\\\x98[`\xE2\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[_T`\x02T`@Qc\x06\x82Fw`\xE2\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91c\x1A\t\x19\xDC\x91a\x01\xB9\x91`\x04\x01\x90\x81R` \x01\x90V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x01\xD0W_\x80\xFD[PZ\xF1\x92PPP\x80\x15a\x01\xE1WP`\x01[a\x022W=\x80\x80\x15a\x02\x0EW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x02\x13V[``\x91P[P`\x01`\x03_\x82\x82Ta\x02&\x91\x90a\x0B\x8AV[\x90\x91UPa\x02z\x91PPV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7Fshould not be able to unregister`D\x82\x01R`d\x01a\x01|V[_T`\x02T`@Qc\xE3\xFC\x02\x8D`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91c\xE3\xFC\x02\x8D\x91a\x02\xAE\x91`\x04\x01\x90\x81R` \x01\x90V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x02\xC5W_\x80\xFD[PZ\xF1\x92PPP\x80\x15a\x02\xD6WP`\x01[a\x03'W=\x80\x80\x15a\x03\x03W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x03\x08V[``\x91P[P`\x01`\x03_\x82\x82Ta\x03\x1B\x91\x90a\x0B\x8AV[\x90\x91UPa\x03~\x91PPV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7Fshould not be able to claim coll`D\x82\x01Re\x18]\x19\\\x98[`\xD2\x1B`d\x82\x01R`\x84\x01a\x01|V[`\x03T`\x03\x14a\x03\xC7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01Rsshould have 3 errors``\x1B`D\x82\x01R`d\x01a\x01|V[\0[_\x80\xFD[4\x80\x15a\x03\xD8W_\x80\xFD[Pa\x03\xC7a\x03\xE76`\x04a\x0EPV[a\x04\xBBV[4\x80\x15a\x03\xF7W_\x80\xFD[Pa\x03\xC7a\x04\x066`\x04a\x11*V[a\x06wV[4\x80\x15a\x04\x16W_\x80\xFD[Pa\x04 `\x03T\x81V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x04>W_\x80\xFD[Pa\x03\xC7a\n\xF3V[4\x80\x15a\x04RW_\x80\xFD[P_Ta\x04e\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x04*V[4\x80\x15a\x04\x88W_\x80\xFD[Pa\x04 `\x02T\x81V[4\x80\x15a\x04\x9DW_\x80\xFD[Pa\x04 `\x01T\x81V[4\x80\x15a\x04\xB2W_\x80\xFD[Pa\x03\xC7a\x0BVV[\x81Q`\x01\x14a\x05\x1EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`)`$\x82\x01R\x7Ftest harness supports only 1 reg`D\x82\x01Rh4\xB9\xBA90\xBA4\xB7\xB7`\xB9\x1B`d\x82\x01R`\x84\x01a\x01|V[\x81_\x81Q\x81\x10a\x050Wa\x050a\x12\x86V[` \x02` \x01\x01Q`#_`\x01\x81\x10a\x05KWa\x05Ka\x12\x86V[\x82Q\x80Q`\r\x92\x90\x92\x02\x92\x90\x92\x01\x90\x81U` \x80\x83\x01Q`\x01\x80\x84\x01\x91\x90\x91U`@\x80\x85\x01Q`\x02\x85\x01U``\x94\x85\x01Q`\x03\x85\x01U\x82\x86\x01Q\x80Q`\x04\x80\x87\x01\x91\x90\x91U\x93\x81\x01Q`\x05\x86\x01U\x80\x82\x01Q`\x06\x86\x01U\x94\x85\x01Q`\x07\x85\x01U`\x80\x85\x01Q`\x08\x85\x01U`\xA0\x85\x01Q`\t\x85\x01U`\xC0\x85\x01Q`\n\x85\x01U`\xE0\x90\x94\x01Q`\x0B\x84\x01U\x93\x83\x01Q`\x0C\x90\x92\x01\x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`@\x1B\x03\x90\x93\x16\x92\x90\x92\x17\x90\x91U_T\x92T\x91Qc\xDC\xE46\x95`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x92c\xDC\xE46\x95\x92\x91a\x06/\x91\x87\x910\x91\x88\x91\x01a\x12\x9AV[` `@Q\x80\x83\x03\x81\x85\x88Z\xF1\x15\x80\x15a\x06KW=_\x80>=_\xFD[PPPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06p\x91\x90a\x13\x88V[`\x02UPPV[\x83Q`\x04\x90\x81U` \x85\x01Q`\x05U`@\x85\x01Q`\x06\x80T`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\x01`\x01`\xA0\x1B\x03\x19\x91\x82\x16\x17\x90\x91U``\x87\x01Q`\x07U`\x80\x87\x01Q`\x08U`\xA0\x87\x01Q`\t\x80T\x91\x84\x16\x91\x83\x16\x91\x90\x91\x17\x90U`\xC0\x87\x01Q`\n\x80T\x91\x90\x93\x16\x91\x16\x17\x90U`\xE0\x85\x01Q\x85\x91\x90`\x0B\x90a\x06\xF6\x90\x82a\x14#V[Pa\x01\0\x82\x01Q`\x08\x82\x01\x80T`\x01`\x01`@\x1B\x03\x92\x83\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x91\x82\x16\x17\x90\x91Ua\x01 \x84\x01Q`\t\x84\x01Ua\x01@\x90\x93\x01Q`\n\x90\x92\x01\x80T\x92\x82\x16\x92\x90\x93\x16\x91\x90\x91\x17\x90\x91U`@\x80\x85\x01Q\x80Q\x80Q\x80Q`\x0F\x90\x81U` \x80\x83\x01Q`\x10U\x82\x86\x01Q`\x11U``\x92\x83\x01Q`\x12U\x80\x84\x01Q\x80Q`\x13U\x90\x81\x01Q`\x14U\x80\x86\x01Q`\x15U\x82\x01Q`\x16U\x93\x82\x01Q`\x17\x80T\x92\x84\x01Q\x90\x96\x16`\x01`\xA0\x1B\x02`\x01`\x01`\xE0\x1B\x03\x19\x90\x92\x16`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x17\x17\x90\x93U`\x80\x83\x01Q\x90\x92\x90\x82\x90`\x18\x90a\x07\xD5\x90\x82a\x14#V[PPP` \x82\x81\x01Q`\n\x83\x01\x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`@\x1B\x03\x90\x92\x16\x91\x90\x91\x17\x90U`@\x80\x84\x01Q`\x0B\x84\x01U``\x93\x84\x01Q\x80Q`\x0C\x85\x01U\x91\x82\x01Q`\r\x84\x01U\x81\x01Q`\x0E\x83\x01U\x91\x82\x01Q`\x0F\x82\x01U`\x80\x82\x01Q`\x10\x82\x01U`\xA0\x82\x01Q`\x11\x82\x01U`\xC0\x82\x01Q`\x12\x82\x01U`\xE0\x90\x91\x01Q`\x13\x90\x91\x01U_[\x83` \x01QQ\x81\x10\x15a\tPW\x83` \x01Q\x81\x81Q\x81\x10a\x08\x81Wa\x08\x81a\x12\x86V[` \x02` \x01\x01Q`#\x82`\x01\x81\x10a\x08\x9CWa\x08\x9Ca\x12\x86V[\x82Q\x80Q`\r\x92\x90\x92\x02\x92\x90\x92\x01\x90\x81U` \x80\x83\x01Q`\x01\x80\x84\x01\x91\x90\x91U`@\x80\x85\x01Q`\x02\x85\x01U``\x94\x85\x01Q`\x03\x85\x01U\x82\x86\x01Q\x80Q`\x04\x86\x01U\x92\x83\x01Q`\x05\x85\x01U\x82\x81\x01Q`\x06\x85\x01U\x93\x82\x01Q`\x07\x84\x01U`\x80\x82\x01Q`\x08\x84\x01U`\xA0\x82\x01Q`\t\x84\x01U`\xC0\x82\x01Q`\n\x84\x01U`\xE0\x90\x91\x01Q`\x0B\x83\x01U\x91\x90\x92\x01Q`\x0C\x90\x92\x01\x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`@\x1B\x03\x90\x93\x16\x92\x90\x92\x17\x90\x91U\x01a\x08^V[P\x81Q\x80Q`1\x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`@\x1B\x03\x90\x92\x16\x91\x90\x91\x17\x81U` \x82\x01Q\x84\x92\x90\x82\x90`2\x90a\t\x8C\x90\x82a\x14#V[P`@\x91\x90\x91\x01Q`\x02\x90\x91\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x90\x91\x17\x90U` \x82\x01Q`\x03\x82\x01\x90a\t\xCA\x90\x82a\x14#V[PP\x81Q\x80Q\x80Q`5\x90\x81U` \x80\x83\x01Q`6U`@\x80\x84\x01Q`7U``\x93\x84\x01Q`8U\x81\x85\x01Q\x80Q`9U\x91\x82\x01Q`:U\x81\x81\x01Q`;U\x90\x83\x01Q`<U\x83\x01Q`=\x80T\x93\x85\x01Q`\x01`\x01`@\x1B\x03\x16`\x01`\xA0\x1B\x02`\x01`\x01`\xE0\x1B\x03\x19\x90\x94\x16`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x90\x91\x17\x92\x90\x92\x17\x90\x91U`\x80\x82\x01Q\x84\x93P\x90\x91\x90\x82\x90`>\x90a\nf\x90\x82a\x14#V[PPP` \x82\x81\x01Q`\n\x83\x01\x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`@\x1B\x03\x90\x92\x16\x91\x90\x91\x17\x90U`@\x80\x84\x01Q`\x0B\x84\x01U``\x93\x84\x01Q\x80Q`\x0C\x85\x01U\x91\x82\x01Q`\r\x84\x01U\x81\x01Q`\x0E\x83\x01U\x91\x82\x01Q`\x0F\x82\x01U`\x80\x82\x01Q`\x10\x82\x01U`\xA0\x82\x01Q`\x11\x82\x01U`\xC0\x82\x01Q`\x12\x82\x01U`\xE0\x90\x91\x01Q`\x13\x90\x91\x01UPPPPV[_T`\x02T`@Qc\xE3\xFC\x02\x8D`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91c\xE3\xFC\x02\x8D\x91a\x0B'\x91`\x04\x01\x90\x81R` \x01\x90V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x0B>W_\x80\xFD[PZ\xF1\x15\x80\x15a\x0BPW=_\x80>=_\xFD[PPPPV[_T`\x02T`@Qc\x06\x82Fw`\xE2\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91c\x1A\t\x19\xDC\x91a\x0B'\x91`\x04\x01\x90\x81R` \x01\x90V[\x80\x82\x01\x80\x82\x11\x15a\x0B\xA9WcNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x92\x91PPV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@Q`\x80\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x0B\xE5Wa\x0B\xE5a\x0B\xAFV[`@R\x90V[`@Q``\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x0B\xE5Wa\x0B\xE5a\x0B\xAFV[`@Q`\xA0\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x0B\xE5Wa\x0B\xE5a\x0B\xAFV[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x0B\xE5Wa\x0B\xE5a\x0B\xAFV[`@Qa\x01`\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x0B\xE5Wa\x0B\xE5a\x0B\xAFV[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x0C\x9CWa\x0C\x9Ca\x0B\xAFV[`@R\x91\x90PV[_`\x80\x82\x84\x03\x12\x15a\x0C\xB4W_\x80\xFD[a\x0C\xBCa\x0B\xC3V[\x90P\x815\x81R` \x82\x015` \x82\x01R`@\x82\x015`@\x82\x01R``\x82\x015``\x82\x01R\x92\x91PPV[_a\x01\0\x80\x83\x85\x03\x12\x15a\x0C\xF8W_\x80\xFD[`@Q\x90\x81\x01\x90`\x01`\x01`@\x1B\x03\x82\x11\x81\x83\x10\x17\x15a\r\x1AWa\r\x1Aa\x0B\xAFV[\x81`@R\x80\x92P\x835\x81R` \x84\x015` \x82\x01R`@\x84\x015`@\x82\x01R``\x84\x015``\x82\x01R`\x80\x84\x015`\x80\x82\x01R`\xA0\x84\x015`\xA0\x82\x01R`\xC0\x84\x015`\xC0\x82\x01R`\xE0\x84\x015`\xE0\x82\x01RPP\x92\x91PPV[\x805`\x01`\x01`@\x1B\x03\x81\x16\x81\x14a\r\x89W_\x80\xFD[\x91\x90PV[_\x82`\x1F\x83\x01\x12a\r\x9DW_\x80\xFD[\x815` `\x01`\x01`@\x1B\x03\x82\x11\x15a\r\xB8Wa\r\xB8a\x0B\xAFV[a\r\xC6\x81\x83`\x05\x1B\x01a\x0CtV[\x82\x81Ra\x01\xA0\x92\x83\x02\x85\x01\x82\x01\x92\x82\x82\x01\x91\x90\x87\x85\x11\x15a\r\xE5W_\x80\xFD[\x83\x87\x01[\x85\x81\x10\x15a\x0ECW\x81\x81\x8A\x03\x12\x15a\r\xFFW_\x80\xFD[a\x0E\x07a\x0B\xEBV[a\x0E\x11\x8A\x83a\x0C\xA4V[\x81Ra\x0E \x8A`\x80\x84\x01a\x0C\xE6V[\x86\x82\x01Ra\x0E1a\x01\x80\x83\x01a\rsV[`@\x82\x01R\x84R\x92\x84\x01\x92\x81\x01a\r\xE9V[P\x90\x97\x96PPPPPPPV[_\x80`@\x83\x85\x03\x12\x15a\x0EaW_\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0EvW_\x80\xFD[a\x0E\x82\x85\x82\x86\x01a\r\x8EV[\x95` \x94\x90\x94\x015\x94PPPPV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\r\x89W_\x80\xFD[_\x82`\x1F\x83\x01\x12a\x0E\xB6W_\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0E\xCFWa\x0E\xCFa\x0B\xAFV[a\x0E\xE2`\x1F\x82\x01`\x1F\x19\x16` \x01a\x0CtV[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\x0E\xF6W_\x80\xFD[\x81` \x85\x01` \x83\x017_\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[_a\x01`\x80\x83\x85\x03\x12\x15a\x0F$W_\x80\xFD[a\x0F,a\x0B\xC3V[\x91P\x825`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x0FDW_\x80\xFD[\x81\x85\x01\x91P\x82\x82\x87\x03\x12\x15a\x0FWW_\x80\xFD[a\x0F_a\x0C\rV[\x92Pa\x0Fk\x86\x83a\x0C\xA4V[\x83Ra\x0Fz\x86`\x80\x84\x01a\x0C\xA4V[` \x84\x01Ra\x0F\x8Ca\x01\0\x83\x01a\x0E\x91V[`@\x84\x01Ra\x0F\x9Ea\x01 \x83\x01a\rsV[``\x84\x01Ra\x01@\x82\x015\x81\x81\x11\x15a\x0F\xB5W_\x80\xFD[a\x0F\xC1\x87\x82\x85\x01a\x0E\xA7V[`\x80\x85\x01RPPP\x81Ra\x0F\xD7` \x83\x01a\rsV[` \x82\x01R`@\x82\x015`@\x82\x01Ra\x0F\xF3\x83``\x84\x01a\x0C\xE6V[``\x82\x01R\x92\x91PPV[_``\x82\x84\x03\x12\x15a\x10\x0EW_\x80\xFD[a\x10\x16a\x0B\xEBV[\x90P\x815\x81R` \x82\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x105W_\x80\xFD[a\x10A\x85\x83\x86\x01a\r\x8EV[` \x84\x01R`@\x84\x015\x91P\x80\x82\x11\x15a\x10YW_\x80\xFD[Pa\x10f\x84\x82\x85\x01a\x0F\x12V[`@\x83\x01RP\x92\x91PPV[_`@\x82\x84\x03\x12\x15a\x10\x82W_\x80\xFD[a\x10\x8Aa\x0C/V[\x90P\x815`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x10\xA2W_\x80\xFD[\x90\x83\x01\x90``\x82\x86\x03\x12\x15a\x10\xB5W_\x80\xFD[a\x10\xBDa\x0B\xEBV[a\x10\xC6\x83a\rsV[\x81R` \x83\x015\x82\x81\x11\x15a\x10\xD9W_\x80\xFD[a\x10\xE5\x87\x82\x86\x01a\x0E\xA7V[` \x83\x01RPa\x10\xF7`@\x84\x01a\x0E\x91V[`@\x82\x01R\x83R` \x84\x015\x91P\x80\x82\x11\x15a\x11\x11W_\x80\xFD[Pa\x11\x1E\x84\x82\x85\x01a\x0E\xA7V[` \x83\x01RP\x92\x91PPV[_\x80_\x80`\x80\x85\x87\x03\x12\x15a\x11=W_\x80\xFD[\x845`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x11SW_\x80\xFD[\x90\x86\x01\x90a\x01`\x82\x89\x03\x12\x15a\x11gW_\x80\xFD[a\x11oa\x0CQV[\x825\x81R` \x83\x015` \x82\x01Ra\x11\x89`@\x84\x01a\x0E\x91V[`@\x82\x01R``\x83\x015``\x82\x01R`\x80\x83\x015`\x80\x82\x01Ra\x11\xAE`\xA0\x84\x01a\x0E\x91V[`\xA0\x82\x01Ra\x11\xBF`\xC0\x84\x01a\x0E\x91V[`\xC0\x82\x01R`\xE0\x83\x015\x82\x81\x11\x15a\x11\xD5W_\x80\xFD[a\x11\xE1\x8A\x82\x86\x01a\x0E\xA7V[`\xE0\x83\x01RPa\x01\0a\x11\xF5\x81\x85\x01a\rsV[\x90\x82\x01Ra\x01 \x83\x81\x015\x90\x82\x01Ra\x01@a\x12\x12\x81\x85\x01a\rsV[\x90\x82\x01R\x95P` \x87\x015\x91P\x80\x82\x11\x15a\x12+W_\x80\xFD[a\x127\x88\x83\x89\x01a\x0F\xFEV[\x94P`@\x87\x015\x91P\x80\x82\x11\x15a\x12LW_\x80\xFD[a\x12X\x88\x83\x89\x01a\x10rV[\x93P``\x87\x015\x91P\x80\x82\x11\x15a\x12mW_\x80\xFD[Pa\x12z\x87\x82\x88\x01a\x0F\x12V[\x91PP\x92\x95\x91\x94P\x92PV[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[``\x80\x82R\x84Q\x82\x82\x01\x81\x90R_\x91\x90`\x80\x90\x81\x85\x01\x90` \x80\x8A\x01\x86[\x83\x81\x10\x15a\x13\\W\x81Q\x80Q\x80Q\x87R\x84\x81\x01Q\x85\x88\x01R`@\x80\x82\x01Q\x81\x89\x01R\x90\x89\x01Q\x89\x88\x01R\x84\x82\x01Q\x80Q\x89\x89\x01R\x80\x86\x01Q`\xA0\x80\x8A\x01\x91\x90\x91R\x81\x83\x01Q`\xC0\x80\x8B\x01\x91\x90\x91R\x82\x8C\x01Q`\xE0\x80\x8C\x01\x91\x90\x91R\x8B\x84\x01Qa\x01\0\x8C\x01R\x91\x83\x01Qa\x01 \x8B\x01R\x82\x01Qa\x01@\x8A\x01R\x01Qa\x01`\x88\x01R\x01Q`\x01`\x01`@\x1B\x03\x16a\x01\x80\x86\x01Ra\x01\xA0\x90\x94\x01\x93\x90\x82\x01\x90`\x01\x01a\x12\xB8V[PP\x82\x95Pa\x13u\x81\x88\x01\x8A`\x01`\x01`\xA0\x1B\x03\x16\x90RV[PPPPP\x82`@\x83\x01R\x94\x93PPPPV[_` \x82\x84\x03\x12\x15a\x13\x98W_\x80\xFD[PQ\x91\x90PV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x13\xB3W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x13\xD1WcNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[P\x91\x90PV[`\x1F\x82\x11\x15a\x14\x1EW\x80_R` _ `\x1F\x84\x01`\x05\x1C\x81\x01` \x85\x10\x15a\x13\xFCWP\x80[`\x1F\x84\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\x14\x1BW_\x81U`\x01\x01a\x14\x08V[PP[PPPV[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x14<Wa\x14<a\x0B\xAFV[a\x14P\x81a\x14J\x84Ta\x13\x9FV[\x84a\x13\xD7V[` \x80`\x1F\x83\x11`\x01\x81\x14a\x14\x83W_\x84\x15a\x14lWP\x85\x83\x01Q[_\x19`\x03\x86\x90\x1B\x1C\x19\x16`\x01\x85\x90\x1B\x17\x85Ua\x14\xDAV[_\x85\x81R` \x81 `\x1F\x19\x86\x16\x91[\x82\x81\x10\x15a\x14\xB1W\x88\x86\x01Q\x82U\x94\x84\x01\x94`\x01\x90\x91\x01\x90\x84\x01a\x14\x92V[P\x85\x82\x10\x15a\x14\xCEW\x87\x85\x01Q_\x19`\x03\x88\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PP`\x01\x84`\x01\x1B\x01\x85U[PPPPPPV\xFE\xA2dipfsX\"\x12 \x84\xD4>F\xCB\xD9=\xDB\x84,\x06\xBD\xF6)\x9D\xE3\x03\xF3\x0F\xC3z\x976\xEE%\xAD\xF5\x06\x14\xCE\x05adsolcC\0\x08\x19\x003",
    );
    /**Constructor`.
```solidity
constructor(address registryAddress);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct constructorCall {
        #[allow(missing_docs)]
        pub registryAddress: alloy::sol_types::private::Address,
    }
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<constructorCall> for UnderlyingRustTuple<'_> {
                fn from(value: constructorCall) -> Self {
                    (value.registryAddress,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for constructorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { registryAddress: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolConstructor for constructorCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.registryAddress,
                    ),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `claimCollateral()` and selector `0x6f0b0c1c`.
```solidity
function claimCollateral() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct claimCollateralCall {}
    ///Container type for the return parameters of the [`claimCollateral()`](claimCollateralCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct claimCollateralReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<claimCollateralCall> for UnderlyingRustTuple<'_> {
                fn from(value: claimCollateralCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for claimCollateralCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<claimCollateralReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: claimCollateralReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for claimCollateralReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for claimCollateralCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = claimCollateralReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "claimCollateral()";
            const SELECTOR: [u8; 4] = [111u8, 11u8, 12u8, 28u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `collateral()` and selector `0xd8dfeb45`.
```solidity
function collateral() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct collateralCall {}
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`collateral()`](collateralCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct collateralReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<collateralCall> for UnderlyingRustTuple<'_> {
                fn from(value: collateralCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for collateralCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<collateralReturn> for UnderlyingRustTuple<'_> {
                fn from(value: collateralReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for collateralReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for collateralCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = collateralReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "collateral()";
            const SELECTOR: [u8; 4] = [216u8, 223u8, 235u8, 69u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `errors()` and selector `0x6471a855`.
```solidity
function errors() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct errorsCall {}
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`errors()`](errorsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct errorsReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<errorsCall> for UnderlyingRustTuple<'_> {
                fn from(value: errorsCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for errorsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<errorsReturn> for UnderlyingRustTuple<'_> {
                fn from(value: errorsReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for errorsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for errorsCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = errorsReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "errors()";
            const SELECTOR: [u8; 4] = [100u8, 113u8, 168u8, 85u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `register(((bytes32,bytes32,bytes32,bytes32),(bytes32,bytes32,bytes32,bytes32,bytes32,bytes32,bytes32,bytes32),uint64)[],bytes32)` and selector `0x1f3fc639`.
```solidity
function register(IRegistry.SignedRegistration[] memory _registrations, bytes32 signingId) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct registerCall {
        #[allow(missing_docs)]
        pub _registrations: alloy::sol_types::private::Vec<
            <IRegistry::SignedRegistration as alloy::sol_types::SolType>::RustType,
        >,
        #[allow(missing_docs)]
        pub signingId: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`register(((bytes32,bytes32,bytes32,bytes32),(bytes32,bytes32,bytes32,bytes32,bytes32,bytes32,bytes32,bytes32),uint64)[],bytes32)`](registerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct registerReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<IRegistry::SignedRegistration>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <IRegistry::SignedRegistration as alloy::sol_types::SolType>::RustType,
                >,
                alloy::sol_types::private::FixedBytes<32>,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<registerCall> for UnderlyingRustTuple<'_> {
                fn from(value: registerCall) -> Self {
                    (value._registrations, value.signingId)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for registerCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _registrations: tuple.0,
                        signingId: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<registerReturn> for UnderlyingRustTuple<'_> {
                fn from(value: registerReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for registerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for registerCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Array<IRegistry::SignedRegistration>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = registerReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "register(((bytes32,bytes32,bytes32,bytes32),(bytes32,bytes32,bytes32,bytes32,bytes32,bytes32,bytes32,bytes32),uint64)[],bytes32)";
            const SELECTOR: [u8; 4] = [31u8, 63u8, 198u8, 57u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        IRegistry::SignedRegistration,
                    > as alloy_sol_types::SolType>::tokenize(&self._registrations),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.signingId),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `registrationRoot()` and selector `0x7e3780ac`.
```solidity
function registrationRoot() external view returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct registrationRootCall {}
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`registrationRoot()`](registrationRootCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct registrationRootReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<registrationRootCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: registrationRootCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for registrationRootCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<registrationRootReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: registrationRootReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for registrationRootReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for registrationRootCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = registrationRootReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "registrationRoot()";
            const SELECTOR: [u8; 4] = [126u8, 55u8, 128u8, 172u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `registry()` and selector `0x7b103999`.
```solidity
function registry() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct registryCall {}
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`registry()`](registryCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct registryReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<registryCall> for UnderlyingRustTuple<'_> {
                fn from(value: registryCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for registryCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<registryReturn> for UnderlyingRustTuple<'_> {
                fn from(value: registryReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for registryReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for registryCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = registryReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "registry()";
            const SELECTOR: [u8; 4] = [123u8, 16u8, 57u8, 153u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive()]
    /**Function with signature `saveResult((uint256,uint256,address,uint256,uint256,address,address,bytes,uint64,bytes32,uint64),(bytes32,((bytes32,bytes32,bytes32,bytes32),(bytes32,bytes32,bytes32,bytes32,bytes32,bytes32,bytes32,bytes32),uint64)[],(((bytes32,bytes32,bytes32,bytes32),(bytes32,bytes32,bytes32,bytes32),address,uint64,bytes),uint64,bytes32,(bytes32,bytes32,bytes32,bytes32,bytes32,bytes32,bytes32,bytes32))),((uint64,bytes,address),bytes),(((bytes32,bytes32,bytes32,bytes32),(bytes32,bytes32,bytes32,bytes32),address,uint64,bytes),uint64,bytes32,(bytes32,bytes32,bytes32,bytes32,bytes32,bytes32,bytes32,bytes32)))` and selector `0x21e97b09`.
```solidity
function saveResult(UnitTestHelper.RegisterAndDelegateParams memory _params, UnitTestHelper.RegisterAndDelegateResult memory _result, ISlasher.SignedCommitment memory _signedCommitment, ISlasher.SignedDelegation memory _signedDelegationTwo) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct saveResultCall {
        #[allow(missing_docs)]
        pub _params: <UnitTestHelper::RegisterAndDelegateParams as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub _result: <UnitTestHelper::RegisterAndDelegateResult as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub _signedCommitment: <ISlasher::SignedCommitment as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub _signedDelegationTwo: <ISlasher::SignedDelegation as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`saveResult((uint256,uint256,address,uint256,uint256,address,address,bytes,uint64,bytes32,uint64),(bytes32,((bytes32,bytes32,bytes32,bytes32),(bytes32,bytes32,bytes32,bytes32,bytes32,bytes32,bytes32,bytes32),uint64)[],(((bytes32,bytes32,bytes32,bytes32),(bytes32,bytes32,bytes32,bytes32),address,uint64,bytes),uint64,bytes32,(bytes32,bytes32,bytes32,bytes32,bytes32,bytes32,bytes32,bytes32))),((uint64,bytes,address),bytes),(((bytes32,bytes32,bytes32,bytes32),(bytes32,bytes32,bytes32,bytes32),address,uint64,bytes),uint64,bytes32,(bytes32,bytes32,bytes32,bytes32,bytes32,bytes32,bytes32,bytes32)))`](saveResultCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct saveResultReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                UnitTestHelper::RegisterAndDelegateParams,
                UnitTestHelper::RegisterAndDelegateResult,
                ISlasher::SignedCommitment,
                ISlasher::SignedDelegation,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <UnitTestHelper::RegisterAndDelegateParams as alloy::sol_types::SolType>::RustType,
                <UnitTestHelper::RegisterAndDelegateResult as alloy::sol_types::SolType>::RustType,
                <ISlasher::SignedCommitment as alloy::sol_types::SolType>::RustType,
                <ISlasher::SignedDelegation as alloy::sol_types::SolType>::RustType,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<saveResultCall> for UnderlyingRustTuple<'_> {
                fn from(value: saveResultCall) -> Self {
                    (
                        value._params,
                        value._result,
                        value._signedCommitment,
                        value._signedDelegationTwo,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for saveResultCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _params: tuple.0,
                        _result: tuple.1,
                        _signedCommitment: tuple.2,
                        _signedDelegationTwo: tuple.3,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<saveResultReturn> for UnderlyingRustTuple<'_> {
                fn from(value: saveResultReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for saveResultReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for saveResultCall {
            type Parameters<'a> = (
                UnitTestHelper::RegisterAndDelegateParams,
                UnitTestHelper::RegisterAndDelegateResult,
                ISlasher::SignedCommitment,
                ISlasher::SignedDelegation,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = saveResultReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "saveResult((uint256,uint256,address,uint256,uint256,address,address,bytes,uint64,bytes32,uint64),(bytes32,((bytes32,bytes32,bytes32,bytes32),(bytes32,bytes32,bytes32,bytes32,bytes32,bytes32,bytes32,bytes32),uint64)[],(((bytes32,bytes32,bytes32,bytes32),(bytes32,bytes32,bytes32,bytes32),address,uint64,bytes),uint64,bytes32,(bytes32,bytes32,bytes32,bytes32,bytes32,bytes32,bytes32,bytes32))),((uint64,bytes,address),bytes),(((bytes32,bytes32,bytes32,bytes32),(bytes32,bytes32,bytes32,bytes32),address,uint64,bytes),uint64,bytes32,(bytes32,bytes32,bytes32,bytes32,bytes32,bytes32,bytes32,bytes32)))";
            const SELECTOR: [u8; 4] = [33u8, 233u8, 123u8, 9u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <UnitTestHelper::RegisterAndDelegateParams as alloy_sol_types::SolType>::tokenize(
                        &self._params,
                    ),
                    <UnitTestHelper::RegisterAndDelegateResult as alloy_sol_types::SolType>::tokenize(
                        &self._result,
                    ),
                    <ISlasher::SignedCommitment as alloy_sol_types::SolType>::tokenize(
                        &self._signedCommitment,
                    ),
                    <ISlasher::SignedDelegation as alloy_sol_types::SolType>::tokenize(
                        &self._signedDelegationTwo,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `unregister()` and selector `0xe79a198f`.
```solidity
function unregister() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct unregisterCall {}
    ///Container type for the return parameters of the [`unregister()`](unregisterCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct unregisterReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<unregisterCall> for UnderlyingRustTuple<'_> {
                fn from(value: unregisterCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for unregisterCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<unregisterReturn> for UnderlyingRustTuple<'_> {
                fn from(value: unregisterReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for unregisterReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for unregisterCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = unregisterReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "unregister()";
            const SELECTOR: [u8; 4] = [231u8, 154u8, 25u8, 143u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    ///Container for all the [`ReentrantRegistrationContract`](self) function calls.
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive()]
    pub enum ReentrantRegistrationContractCalls {
        #[allow(missing_docs)]
        claimCollateral(claimCollateralCall),
        #[allow(missing_docs)]
        collateral(collateralCall),
        #[allow(missing_docs)]
        errors(errorsCall),
        #[allow(missing_docs)]
        register(registerCall),
        #[allow(missing_docs)]
        registrationRoot(registrationRootCall),
        #[allow(missing_docs)]
        registry(registryCall),
        #[allow(missing_docs)]
        saveResult(saveResultCall),
        #[allow(missing_docs)]
        unregister(unregisterCall),
    }
    #[automatically_derived]
    impl ReentrantRegistrationContractCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [31u8, 63u8, 198u8, 57u8],
            [33u8, 233u8, 123u8, 9u8],
            [100u8, 113u8, 168u8, 85u8],
            [111u8, 11u8, 12u8, 28u8],
            [123u8, 16u8, 57u8, 153u8],
            [126u8, 55u8, 128u8, 172u8],
            [216u8, 223u8, 235u8, 69u8],
            [231u8, 154u8, 25u8, 143u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for ReentrantRegistrationContractCalls {
        const NAME: &'static str = "ReentrantRegistrationContractCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 8usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::claimCollateral(_) => {
                    <claimCollateralCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::collateral(_) => {
                    <collateralCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::errors(_) => <errorsCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::register(_) => <registerCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::registrationRoot(_) => {
                    <registrationRootCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::registry(_) => <registryCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::saveResult(_) => {
                    <saveResultCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::unregister(_) => {
                    <unregisterCall as alloy_sol_types::SolCall>::SELECTOR
                }
            }
        }
        #[inline]
        fn selector_at(i: usize) -> ::core::option::Option<[u8; 4]> {
            Self::SELECTORS.get(i).copied()
        }
        #[inline]
        fn valid_selector(selector: [u8; 4]) -> bool {
            Self::SELECTORS.binary_search(&selector).is_ok()
        }
        #[inline]
        #[allow(non_snake_case)]
        fn abi_decode_raw(
            selector: [u8; 4],
            data: &[u8],
            validate: bool,
        ) -> alloy_sol_types::Result<Self> {
            static DECODE_SHIMS: &[fn(
                &[u8],
                bool,
            ) -> alloy_sol_types::Result<ReentrantRegistrationContractCalls>] = &[
                {
                    fn register(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ReentrantRegistrationContractCalls> {
                        <registerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ReentrantRegistrationContractCalls::register)
                    }
                    register
                },
                {
                    fn saveResult(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ReentrantRegistrationContractCalls> {
                        <saveResultCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ReentrantRegistrationContractCalls::saveResult)
                    }
                    saveResult
                },
                {
                    fn errors(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ReentrantRegistrationContractCalls> {
                        <errorsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ReentrantRegistrationContractCalls::errors)
                    }
                    errors
                },
                {
                    fn claimCollateral(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ReentrantRegistrationContractCalls> {
                        <claimCollateralCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ReentrantRegistrationContractCalls::claimCollateral)
                    }
                    claimCollateral
                },
                {
                    fn registry(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ReentrantRegistrationContractCalls> {
                        <registryCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ReentrantRegistrationContractCalls::registry)
                    }
                    registry
                },
                {
                    fn registrationRoot(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ReentrantRegistrationContractCalls> {
                        <registrationRootCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ReentrantRegistrationContractCalls::registrationRoot)
                    }
                    registrationRoot
                },
                {
                    fn collateral(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ReentrantRegistrationContractCalls> {
                        <collateralCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ReentrantRegistrationContractCalls::collateral)
                    }
                    collateral
                },
                {
                    fn unregister(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ReentrantRegistrationContractCalls> {
                        <unregisterCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ReentrantRegistrationContractCalls::unregister)
                    }
                    unregister
                },
            ];
            let Ok(idx) = Self::SELECTORS.binary_search(&selector) else {
                return Err(
                    alloy_sol_types::Error::unknown_selector(
                        <Self as alloy_sol_types::SolInterface>::NAME,
                        selector,
                    ),
                );
            };
            DECODE_SHIMS[idx](data, validate)
        }
        #[inline]
        fn abi_encoded_size(&self) -> usize {
            match self {
                Self::claimCollateral(inner) => {
                    <claimCollateralCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::collateral(inner) => {
                    <collateralCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::errors(inner) => {
                    <errorsCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::register(inner) => {
                    <registerCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::registrationRoot(inner) => {
                    <registrationRootCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::registry(inner) => {
                    <registryCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::saveResult(inner) => {
                    <saveResultCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::unregister(inner) => {
                    <unregisterCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::claimCollateral(inner) => {
                    <claimCollateralCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::collateral(inner) => {
                    <collateralCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::errors(inner) => {
                    <errorsCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::register(inner) => {
                    <registerCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::registrationRoot(inner) => {
                    <registrationRootCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::registry(inner) => {
                    <registryCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::saveResult(inner) => {
                    <saveResultCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::unregister(inner) => {
                    <unregisterCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`ReentrantRegistrationContract`](self) contract instance.

See the [wrapper's documentation](`ReentrantRegistrationContractInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> ReentrantRegistrationContractInstance<T, P, N> {
        ReentrantRegistrationContractInstance::<T, P, N>::new(address, provider)
    }
    /**Deploys this contract using the given `provider` and constructor arguments, if any.

Returns a new instance of the contract, if the deployment was successful.

For more fine-grained control over the deployment process, use [`deploy_builder`] instead.*/
    #[inline]
    pub fn deploy<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        provider: P,
        registryAddress: alloy::sol_types::private::Address,
    ) -> impl ::core::future::Future<
        Output = alloy_contract::Result<ReentrantRegistrationContractInstance<T, P, N>>,
    > {
        ReentrantRegistrationContractInstance::<
            T,
            P,
            N,
        >::deploy(provider, registryAddress)
    }
    /**Creates a `RawCallBuilder` for deploying this contract using the given `provider`
and constructor arguments, if any.

This is a simple wrapper around creating a `RawCallBuilder` with the data set to
the bytecode concatenated with the constructor's ABI-encoded arguments.*/
    #[inline]
    pub fn deploy_builder<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        provider: P,
        registryAddress: alloy::sol_types::private::Address,
    ) -> alloy_contract::RawCallBuilder<T, P, N> {
        ReentrantRegistrationContractInstance::<
            T,
            P,
            N,
        >::deploy_builder(provider, registryAddress)
    }
    /**A [`ReentrantRegistrationContract`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`ReentrantRegistrationContract`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct ReentrantRegistrationContractInstance<
        T,
        P,
        N = alloy_contract::private::Ethereum,
    > {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for ReentrantRegistrationContractInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("ReentrantRegistrationContractInstance")
                .field(&self.address)
                .finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > ReentrantRegistrationContractInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`ReentrantRegistrationContract`](self) contract instance.

See the [wrapper's documentation](`ReentrantRegistrationContractInstance`) for more details.*/
        #[inline]
        pub const fn new(
            address: alloy_sol_types::private::Address,
            provider: P,
        ) -> Self {
            Self {
                address,
                provider,
                _network_transport: ::core::marker::PhantomData,
            }
        }
        /**Deploys this contract using the given `provider` and constructor arguments, if any.

Returns a new instance of the contract, if the deployment was successful.

For more fine-grained control over the deployment process, use [`deploy_builder`] instead.*/
        #[inline]
        pub async fn deploy(
            provider: P,
            registryAddress: alloy::sol_types::private::Address,
        ) -> alloy_contract::Result<ReentrantRegistrationContractInstance<T, P, N>> {
            let call_builder = Self::deploy_builder(provider, registryAddress);
            let contract_address = call_builder.deploy().await?;
            Ok(Self::new(contract_address, call_builder.provider))
        }
        /**Creates a `RawCallBuilder` for deploying this contract using the given `provider`
and constructor arguments, if any.

This is a simple wrapper around creating a `RawCallBuilder` with the data set to
the bytecode concatenated with the constructor's ABI-encoded arguments.*/
        #[inline]
        pub fn deploy_builder(
            provider: P,
            registryAddress: alloy::sol_types::private::Address,
        ) -> alloy_contract::RawCallBuilder<T, P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                provider,
                [
                    &BYTECODE[..],
                    &alloy_sol_types::SolConstructor::abi_encode(
                        &constructorCall { registryAddress },
                    )[..],
                ]
                    .concat()
                    .into(),
            )
        }
        /// Returns a reference to the address.
        #[inline]
        pub const fn address(&self) -> &alloy_sol_types::private::Address {
            &self.address
        }
        /// Sets the address.
        #[inline]
        pub fn set_address(&mut self, address: alloy_sol_types::private::Address) {
            self.address = address;
        }
        /// Sets the address and returns `self`.
        pub fn at(mut self, address: alloy_sol_types::private::Address) -> Self {
            self.set_address(address);
            self
        }
        /// Returns a reference to the provider.
        #[inline]
        pub const fn provider(&self) -> &P {
            &self.provider
        }
    }
    impl<T, P: ::core::clone::Clone, N> ReentrantRegistrationContractInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(
            self,
        ) -> ReentrantRegistrationContractInstance<T, P, N> {
            ReentrantRegistrationContractInstance {
                address: self.address,
                provider: ::core::clone::Clone::clone(&self.provider),
                _network_transport: ::core::marker::PhantomData,
            }
        }
    }
    /// Function calls.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > ReentrantRegistrationContractInstance<T, P, N> {
        /// Creates a new call builder using this contract instance's provider and address.
        ///
        /// Note that the call can be any function call, not just those defined in this
        /// contract. Prefer using the other methods for building type-safe contract calls.
        pub fn call_builder<C: alloy_sol_types::SolCall>(
            &self,
            call: &C,
        ) -> alloy_contract::SolCallBuilder<T, &P, C, N> {
            alloy_contract::SolCallBuilder::new_sol(&self.provider, &self.address, call)
        }
        ///Creates a new call builder for the [`claimCollateral`] function.
        pub fn claimCollateral(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, claimCollateralCall, N> {
            self.call_builder(&claimCollateralCall {})
        }
        ///Creates a new call builder for the [`collateral`] function.
        pub fn collateral(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, collateralCall, N> {
            self.call_builder(&collateralCall {})
        }
        ///Creates a new call builder for the [`errors`] function.
        pub fn errors(&self) -> alloy_contract::SolCallBuilder<T, &P, errorsCall, N> {
            self.call_builder(&errorsCall {})
        }
        ///Creates a new call builder for the [`register`] function.
        pub fn register(
            &self,
            _registrations: alloy::sol_types::private::Vec<
                <IRegistry::SignedRegistration as alloy::sol_types::SolType>::RustType,
            >,
            signingId: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<T, &P, registerCall, N> {
            self.call_builder(
                &registerCall {
                    _registrations,
                    signingId,
                },
            )
        }
        ///Creates a new call builder for the [`registrationRoot`] function.
        pub fn registrationRoot(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, registrationRootCall, N> {
            self.call_builder(&registrationRootCall {})
        }
        ///Creates a new call builder for the [`registry`] function.
        pub fn registry(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, registryCall, N> {
            self.call_builder(&registryCall {})
        }
        ///Creates a new call builder for the [`saveResult`] function.
        pub fn saveResult(
            &self,
            _params: <UnitTestHelper::RegisterAndDelegateParams as alloy::sol_types::SolType>::RustType,
            _result: <UnitTestHelper::RegisterAndDelegateResult as alloy::sol_types::SolType>::RustType,
            _signedCommitment: <ISlasher::SignedCommitment as alloy::sol_types::SolType>::RustType,
            _signedDelegationTwo: <ISlasher::SignedDelegation as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, saveResultCall, N> {
            self.call_builder(
                &saveResultCall {
                    _params,
                    _result,
                    _signedCommitment,
                    _signedDelegationTwo,
                },
            )
        }
        ///Creates a new call builder for the [`unregister`] function.
        pub fn unregister(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, unregisterCall, N> {
            self.call_builder(&unregisterCall {})
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > ReentrantRegistrationContractInstance<T, P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<T, &P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
    }
}
