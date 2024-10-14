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

//! Autogenerated weights for `pallet_proxy`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 36.0.0
//! DATE: 2024-10-14, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `f883c4de6237`, CPU: `AMD Ryzen 7 7700X 8-Core Processor`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("dev")`, DB CACHE: `1024`

// Executed Command:
// /usr/local/bin/zkv-node
// benchmark
// pallet
// --chain
// dev
// --pallet
// pallet-proxy
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
// /data/benchmark/runtime/src/weights/pallet_proxy.rs
// --template
// /data/benchmark/node/zkv-deploy-weight-template.hbs
// --base-path=/tmp/tmp.ceq7Nq25bJ

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weights for `pallet_proxy` using the zkVerify node and recommended hardware.
pub struct ZKVWeight<T>(PhantomData<T>);

impl<T: frame_system::Config> pallet_proxy::WeightInfo for ZKVWeight<T> {
    /// Storage: `Proxy::Proxies` (r:1 w:0)
    /// Proof: `Proxy::Proxies` (`max_values`: None, `max_size`: Some(1241), added: 3716, mode: `MaxEncodedLen`)
    /// The range of component `p` is `[1, 31]`.
    fn proxy(p: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `161 + p * (37 ±0)`
        //  Estimated: `4706`
        // Minimum execution time: 11_110_000 picoseconds.
        Weight::from_parts(11_864_595, 4706)
            // Standard Error: 1_133
            .saturating_add(Weight::from_parts(31_794, 0).saturating_mul(p.into()))
            .saturating_add(T::DbWeight::get().reads(1_u64))
    }
    /// Storage: `Proxy::Proxies` (r:1 w:0)
    /// Proof: `Proxy::Proxies` (`max_values`: None, `max_size`: Some(1241), added: 3716, mode: `MaxEncodedLen`)
    /// Storage: `Proxy::Announcements` (r:1 w:1)
    /// Proof: `Proxy::Announcements` (`max_values`: None, `max_size`: Some(2233), added: 4708, mode: `MaxEncodedLen`)
    /// Storage: `System::Account` (r:1 w:1)
    /// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
    /// The range of component `a` is `[0, 31]`.
    /// The range of component `p` is `[1, 31]`.
    fn proxy_announced(a: u32, p: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `451 + a * (68 ±0) + p * (37 ±0)`
        //  Estimated: `5698`
        // Minimum execution time: 28_990_000 picoseconds.
        Weight::from_parts(28_400_591, 5698)
            // Standard Error: 2_261
            .saturating_add(Weight::from_parts(191_798, 0).saturating_mul(a.into()))
            // Standard Error: 2_336
            .saturating_add(Weight::from_parts(56_811, 0).saturating_mul(p.into()))
            .saturating_add(T::DbWeight::get().reads(3_u64))
            .saturating_add(T::DbWeight::get().writes(2_u64))
    }
    /// Storage: `Proxy::Announcements` (r:1 w:1)
    /// Proof: `Proxy::Announcements` (`max_values`: None, `max_size`: Some(2233), added: 4708, mode: `MaxEncodedLen`)
    /// Storage: `System::Account` (r:1 w:1)
    /// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
    /// The range of component `a` is `[0, 31]`.
    /// The range of component `p` is `[1, 31]`.
    fn remove_announcement(a: u32, p: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `366 + a * (68 ±0)`
        //  Estimated: `5698`
        // Minimum execution time: 19_860_000 picoseconds.
        Weight::from_parts(20_160_927, 5698)
            // Standard Error: 1_500
            .saturating_add(Weight::from_parts(186_567, 0).saturating_mul(a.into()))
            // Standard Error: 1_550
            .saturating_add(Weight::from_parts(9_546, 0).saturating_mul(p.into()))
            .saturating_add(T::DbWeight::get().reads(2_u64))
            .saturating_add(T::DbWeight::get().writes(2_u64))
    }
    /// Storage: `Proxy::Announcements` (r:1 w:1)
    /// Proof: `Proxy::Announcements` (`max_values`: None, `max_size`: Some(2233), added: 4708, mode: `MaxEncodedLen`)
    /// Storage: `System::Account` (r:1 w:1)
    /// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
    /// The range of component `a` is `[0, 31]`.
    /// The range of component `p` is `[1, 31]`.
    fn reject_announcement(a: u32, p: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `366 + a * (68 ±0)`
        //  Estimated: `5698`
        // Minimum execution time: 20_270_000 picoseconds.
        Weight::from_parts(20_615_135, 5698)
            // Standard Error: 1_676
            .saturating_add(Weight::from_parts(174_973, 0).saturating_mul(a.into()))
            // Standard Error: 1_731
            .saturating_add(Weight::from_parts(5_490, 0).saturating_mul(p.into()))
            .saturating_add(T::DbWeight::get().reads(2_u64))
            .saturating_add(T::DbWeight::get().writes(2_u64))
    }
    /// Storage: `Proxy::Proxies` (r:1 w:0)
    /// Proof: `Proxy::Proxies` (`max_values`: None, `max_size`: Some(1241), added: 3716, mode: `MaxEncodedLen`)
    /// Storage: `Proxy::Announcements` (r:1 w:1)
    /// Proof: `Proxy::Announcements` (`max_values`: None, `max_size`: Some(2233), added: 4708, mode: `MaxEncodedLen`)
    /// Storage: `System::Account` (r:1 w:1)
    /// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
    /// The range of component `a` is `[0, 31]`.
    /// The range of component `p` is `[1, 31]`.
    fn announce(a: u32, p: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `383 + a * (68 ±0) + p * (37 ±0)`
        //  Estimated: `5698`
        // Minimum execution time: 26_940_000 picoseconds.
        Weight::from_parts(26_785_138, 5698)
            // Standard Error: 1_461
            .saturating_add(Weight::from_parts(173_007, 0).saturating_mul(a.into()))
            // Standard Error: 1_509
            .saturating_add(Weight::from_parts(45_955, 0).saturating_mul(p.into()))
            .saturating_add(T::DbWeight::get().reads(3_u64))
            .saturating_add(T::DbWeight::get().writes(2_u64))
    }
    /// Storage: `Proxy::Proxies` (r:1 w:1)
    /// Proof: `Proxy::Proxies` (`max_values`: None, `max_size`: Some(1241), added: 3716, mode: `MaxEncodedLen`)
    /// The range of component `p` is `[1, 31]`.
    fn add_proxy(p: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `161 + p * (37 ±0)`
        //  Estimated: `4706`
        // Minimum execution time: 19_260_000 picoseconds.
        Weight::from_parts(19_963_360, 4706)
            // Standard Error: 1_530
            .saturating_add(Weight::from_parts(41_944, 0).saturating_mul(p.into()))
            .saturating_add(T::DbWeight::get().reads(1_u64))
            .saturating_add(T::DbWeight::get().writes(1_u64))
    }
    /// Storage: `Proxy::Proxies` (r:1 w:1)
    /// Proof: `Proxy::Proxies` (`max_values`: None, `max_size`: Some(1241), added: 3716, mode: `MaxEncodedLen`)
    /// The range of component `p` is `[1, 31]`.
    fn remove_proxy(p: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `161 + p * (37 ±0)`
        //  Estimated: `4706`
        // Minimum execution time: 19_051_000 picoseconds.
        Weight::from_parts(20_088_605, 4706)
            // Standard Error: 2_084
            .saturating_add(Weight::from_parts(60_180, 0).saturating_mul(p.into()))
            .saturating_add(T::DbWeight::get().reads(1_u64))
            .saturating_add(T::DbWeight::get().writes(1_u64))
    }
    /// Storage: `Proxy::Proxies` (r:1 w:1)
    /// Proof: `Proxy::Proxies` (`max_values`: None, `max_size`: Some(1241), added: 3716, mode: `MaxEncodedLen`)
    /// The range of component `p` is `[1, 31]`.
    fn remove_proxies(p: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `161 + p * (37 ±0)`
        //  Estimated: `4706`
        // Minimum execution time: 18_460_000 picoseconds.
        Weight::from_parts(19_258_942, 4706)
            // Standard Error: 1_105
            .saturating_add(Weight::from_parts(35_626, 0).saturating_mul(p.into()))
            .saturating_add(T::DbWeight::get().reads(1_u64))
            .saturating_add(T::DbWeight::get().writes(1_u64))
    }
    /// Storage: `Proxy::Proxies` (r:1 w:1)
    /// Proof: `Proxy::Proxies` (`max_values`: None, `max_size`: Some(1241), added: 3716, mode: `MaxEncodedLen`)
    /// The range of component `p` is `[1, 31]`.
    fn create_pure(_p: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `173`
        //  Estimated: `4706`
        // Minimum execution time: 20_150_000 picoseconds.
        Weight::from_parts(21_545_660, 4706)
            .saturating_add(T::DbWeight::get().reads(1_u64))
            .saturating_add(T::DbWeight::get().writes(1_u64))
    }
    /// Storage: `Proxy::Proxies` (r:1 w:1)
    /// Proof: `Proxy::Proxies` (`max_values`: None, `max_size`: Some(1241), added: 3716, mode: `MaxEncodedLen`)
    /// The range of component `p` is `[0, 30]`.
    fn kill_pure(p: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `198 + p * (37 ±0)`
        //  Estimated: `4706`
        // Minimum execution time: 19_210_000 picoseconds.
        Weight::from_parts(20_018_615, 4706)
            // Standard Error: 1_432
            .saturating_add(Weight::from_parts(45_995, 0).saturating_mul(p.into()))
            .saturating_add(T::DbWeight::get().reads(1_u64))
            .saturating_add(T::DbWeight::get().writes(1_u64))
    }
}
