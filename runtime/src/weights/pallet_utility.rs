// Copyright 2024, Horizen Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Autogenerated weights for `pallet_utility`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 36.0.0
//! DATE: 2024-09-10, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `a2206544fd37`, CPU: `AMD Ryzen 7 7700X 8-Core Processor`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("dev")`, DB CACHE: `1024`

// Executed Command:
// /usr/local/bin/zkv-node
// benchmark
// pallet
// --chain
// dev
// --pallet
// pallet-utility
// --extrinsic
// *
// --steps
// 50
// --repeat
// 20
// --heap-pages=4096
// --header
// /data/benchmark/HEADER-APACHE2
// --output
// /data/benchmark/runtime/src/weights/pallet_utility.rs
// --template
// /data/benchmark/node/zkv-deploy-weight-template.hbs
// --base-path=/tmp/tmp.lTvc0D8w2X

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weights for `pallet_utility` using the zkVerify node and recommended hardware.
pub struct ZKVWeight<T>(PhantomData<T>);

impl<T: frame_system::Config> pallet_utility::WeightInfo for ZKVWeight<T> {
    /// The range of component `c` is `[0, 1000]`.
    fn batch(c: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 3_480_000 picoseconds.
        Weight::from_parts(9_529_974, 0)
            // Standard Error: 1_212
            .saturating_add(Weight::from_parts(2_321_137, 0).saturating_mul(c.into()))
    }
    fn as_derivative() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 3_660_000 picoseconds.
        Weight::from_parts(3_920_000, 0)
    }
    /// The range of component `c` is `[0, 1000]`.
    fn batch_all(c: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 3_500_000 picoseconds.
        Weight::from_parts(7_248_467, 0)
            // Standard Error: 2_535
            .saturating_add(Weight::from_parts(2_558_842, 0).saturating_mul(c.into()))
    }
    fn dispatch_as() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 4_940_000 picoseconds.
        Weight::from_parts(5_230_000, 0)
    }
    /// The range of component `c` is `[0, 1000]`.
    fn force_batch(c: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 3_420_000 picoseconds.
        Weight::from_parts(20_734_236, 0)
            // Standard Error: 1_238
            .saturating_add(Weight::from_parts(2_256_388, 0).saturating_mul(c.into()))
    }
}