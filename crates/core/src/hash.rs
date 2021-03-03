// Copyright 2020 Parity Technologies
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::{U128, U256, U512};

use fixed_hash::*;

pub trait BigEndianHash {
    type Uint;

    fn from_uint(val: &Self::Uint) -> Self;
    fn into_uint(&self) -> Self::Uint;
}

construct_fixed_hash! { pub struct H128(16); }
construct_fixed_hash! { pub struct H160(20); }
construct_fixed_hash! { pub struct H256(32); }
construct_fixed_hash! { pub struct H264(33); }
construct_fixed_hash! { pub struct H512(64); }
construct_fixed_hash! { pub struct H520(65); }

macro_rules! impl_uint_conversions {
    ($hash: ident, $uint: ident) => {
        impl BigEndianHash for $hash {
            type Uint = $uint;

            fn from_uint(value: &$uint) -> Self {
                let mut ret = $hash::zero();
                value.to_big_endian(ret.as_bytes_mut());
                ret
            }

            fn into_uint(&self) -> $uint {
                $uint::from(self.as_ref() as &[u8])
            }
        }
    };
}

impl_uint_conversions!(H128, U128);
impl_uint_conversions!(H256, U256);
impl_uint_conversions!(H512, U512);

// #[cfg(test)]
// mod tests {
//     use super::{H160, H256};
//     use serde_json as ser;

//     #[test]
//     #[ignore]
//     fn test_serialize_h160() {
//         let tests = vec![
//             (
//                 H160::from_low_u64_be(0),
//                 "0x0000000000000000000000000000000000000000",
//             ),
//             (
//                 H160::from_low_u64_be(2),
//                 "0x0000000000000000000000000000000000000002",
//             ),
//             (
//                 H160::from_low_u64_be(15),
//                 "0x000000000000000000000000000000000000000f",
//             ),
//             (
//                 H160::from_low_u64_be(16),
//                 "0x0000000000000000000000000000000000000010",
//             ),
//             (
//                 H160::from_low_u64_be(1_000),
//                 "0x00000000000000000000000000000000000003e8",
//             ),
//             (
//                 H160::from_low_u64_be(100_000),
//                 "0x00000000000000000000000000000000000186a0",
//             ),
//             (
//                 H160::from_low_u64_be(u64::max_value()),
//                 "0x000000000000000000000000ffffffffffffffff",
//             ),
//         ];

//         for (number, expected) in tests {
//             assert_eq!(
//                 format!("{:?}", expected),
//                 ser::to_string_pretty(&number).unwrap()
//             );
//             assert_eq!(number, ser::from_str(&format!("{:?}", expected)).unwrap());
//         }
//     }

//     #[test]
//     #[ignore]
//     fn test_serialize_h256() {
//         let tests = vec![
//             (
//                 H256::from_low_u64_be(0),
//                 "0x0000000000000000000000000000000000000000000000000000000000000000",
//             ),
//             (
//                 H256::from_low_u64_be(2),
//                 "0x0000000000000000000000000000000000000000000000000000000000000002",
//             ),
//             (
//                 H256::from_low_u64_be(15),
//                 "0x000000000000000000000000000000000000000000000000000000000000000f",
//             ),
//             (
//                 H256::from_low_u64_be(16),
//                 "0x0000000000000000000000000000000000000000000000000000000000000010",
//             ),
//             (
//                 H256::from_low_u64_be(1_000),
//                 "0x00000000000000000000000000000000000000000000000000000000000003e8",
//             ),
//             (
//                 H256::from_low_u64_be(100_000),
//                 "0x00000000000000000000000000000000000000000000000000000000000186a0",
//             ),
//             (
//                 H256::from_low_u64_be(u64::max_value()),
//                 "0x000000000000000000000000000000000000000000000000ffffffffffffffff",
//             ),
//         ];

//         for (number, expected) in tests {
//             assert_eq!(
//                 format!("{:?}", expected),
//                 ser::to_string_pretty(&number).unwrap()
//             );
//             assert_eq!(number, ser::from_str(&format!("{:?}", expected)).unwrap());
//         }
//     }

//     #[test]
//     #[ignore]
//     fn test_parse_0x() {
//         assert!(
//             "0x0000000000000000000000000000000000000000000000000000000000000000"
//                 .parse::<H256>()
//                 .is_ok()
//         )
//     }

//     #[test]
//     #[ignore]
//     fn test_serialize_invalid() {
//         assert!(ser::from_str::<H256>(
//             "\"0x000000000000000000000000000000000000000000000000000000000000000\""
//         )
//         .unwrap_err()
//         .is_data());
//         assert!(ser::from_str::<H256>(
//             "\"0x000000000000000000000000000000000000000000000000000000000000000g\""
//         )
//         .unwrap_err()
//         .is_data());
//         assert!(ser::from_str::<H256>(
//             "\"0x00000000000000000000000000000000000000000000000000000000000000000\""
//         )
//         .unwrap_err()
//         .is_data());
//         assert!(ser::from_str::<H256>("\"\"").unwrap_err().is_data());
//         assert!(ser::from_str::<H256>("\"0\"").unwrap_err().is_data());
//         assert!(ser::from_str::<H256>("\"10\"").unwrap_err().is_data());
//     }
// }
