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

//! Autogenerated weights for `pallet_scheduler`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 31.0.0
//! DATE: 2024-06-25, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `d58d64237d18`, CPU: `AMD EPYC 7571`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("dev")`, DB CACHE: `1024`

// Executed Command:
// /usr/local/bin/zkv-node
// benchmark
// pallet
// --chain
// dev
// --pallet
// pallet-scheduler
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
// /data/benchmark/runtime/src/weights/pallet_scheduler.rs
// --template
// /data/benchmark/node/zkv-deploy-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weights for `pallet_scheduler` using the zkVerify node and recommended hardware.
pub struct ZKVWeight<T>(PhantomData<T>);

impl<T: frame_system::Config> pallet_scheduler::WeightInfo for ZKVWeight<T> {
    /// Storage: `Scheduler::IncompleteSince` (r:1 w:1)
    /// Proof: `Scheduler::IncompleteSince` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
    fn service_agendas_base() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `30`
        //  Estimated: `1489`
        // Minimum execution time: 6_240_000 picoseconds.
        Weight::from_parts(6_580_000, 1489)
            .saturating_add(T::DbWeight::get().reads(1_u64))
            .saturating_add(T::DbWeight::get().writes(1_u64))
    }
    /// Storage: `Scheduler::Agenda` (r:1 w:1)
    /// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(10463), added: 12938, mode: `MaxEncodedLen`)
    /// The range of component `s` is `[0, 50]`.
    fn service_agenda_base(s: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `77 + s * (177 ±0)`
        //  Estimated: `13928`
        // Minimum execution time: 7_320_000 picoseconds.
        Weight::from_parts(15_412_073, 13928)
            // Standard Error: 6_409
            .saturating_add(Weight::from_parts(724_432, 0).saturating_mul(s.into()))
            .saturating_add(T::DbWeight::get().reads(1_u64))
            .saturating_add(T::DbWeight::get().writes(1_u64))
    }
    fn service_task_base() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 6_740_000 picoseconds.
        Weight::from_parts(6_930_000, 0)
    }
    /// Storage: `Preimage::PreimageFor` (r:1 w:1)
    /// Proof: `Preimage::PreimageFor` (`max_values`: None, `max_size`: Some(4194344), added: 4196819, mode: `Measured`)
    /// Storage: `Preimage::StatusFor` (r:1 w:0)
    /// Proof: `Preimage::StatusFor` (`max_values`: None, `max_size`: Some(91), added: 2566, mode: `MaxEncodedLen`)
    /// Storage: `Preimage::RequestStatusFor` (r:1 w:1)
    /// Proof: `Preimage::RequestStatusFor` (`max_values`: None, `max_size`: Some(91), added: 2566, mode: `MaxEncodedLen`)
    /// The range of component `s` is `[128, 4194304]`.
    fn service_task_fetched(s: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `217 + s * (1 ±0)`
        //  Estimated: `3682 + s * (1 ±0)`
        // Minimum execution time: 38_180_000 picoseconds.
        Weight::from_parts(38_641_000, 3682)
            // Standard Error: 14
            .saturating_add(Weight::from_parts(2_356, 0).saturating_mul(s.into()))
            .saturating_add(T::DbWeight::get().reads(3_u64))
            .saturating_add(T::DbWeight::get().writes(2_u64))
            .saturating_add(Weight::from_parts(0, 1).saturating_mul(s.into()))
    }
    /// Storage: `Scheduler::Lookup` (r:0 w:1)
    /// Proof: `Scheduler::Lookup` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
    fn service_task_named() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 10_070_000 picoseconds.
        Weight::from_parts(11_700_000, 0)
            .saturating_add(T::DbWeight::get().writes(1_u64))
    }
    fn service_task_periodic() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 6_740_000 picoseconds.
        Weight::from_parts(7_240_000, 0)
    }
    fn execute_dispatch_signed() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 5_060_000 picoseconds.
        Weight::from_parts(5_470_000, 0)
    }
    fn execute_dispatch_unsigned() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 4_950_000 picoseconds.
        Weight::from_parts(5_261_000, 0)
    }
    /// Storage: `Scheduler::Agenda` (r:1 w:1)
    /// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(10463), added: 12938, mode: `MaxEncodedLen`)
    /// The range of component `s` is `[0, 49]`.
    fn schedule(s: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `77 + s * (177 ±0)`
        //  Estimated: `13928`
        // Minimum execution time: 20_970_000 picoseconds.
        Weight::from_parts(28_672_050, 13928)
            // Standard Error: 8_340
            .saturating_add(Weight::from_parts(815_748, 0).saturating_mul(s.into()))
            .saturating_add(T::DbWeight::get().reads(1_u64))
            .saturating_add(T::DbWeight::get().writes(1_u64))
    }
    /// Storage: `Scheduler::Agenda` (r:1 w:1)
    /// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(10463), added: 12938, mode: `MaxEncodedLen`)
    /// Storage: `Scheduler::Lookup` (r:0 w:1)
    /// Proof: `Scheduler::Lookup` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
    /// The range of component `s` is `[1, 50]`.
    fn cancel(s: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `77 + s * (177 ±0)`
        //  Estimated: `13928`
        // Minimum execution time: 30_851_000 picoseconds.
        Weight::from_parts(32_327_823, 13928)
            // Standard Error: 16_050
            .saturating_add(Weight::from_parts(1_061_488, 0).saturating_mul(s.into()))
            .saturating_add(T::DbWeight::get().reads(1_u64))
            .saturating_add(T::DbWeight::get().writes(2_u64))
    }
    /// Storage: `Scheduler::Lookup` (r:1 w:1)
    /// Proof: `Scheduler::Lookup` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
    /// Storage: `Scheduler::Agenda` (r:1 w:1)
    /// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(10463), added: 12938, mode: `MaxEncodedLen`)
    /// The range of component `s` is `[0, 49]`.
    fn schedule_named(s: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `254 + s * (185 ±0)`
        //  Estimated: `13928`
        // Minimum execution time: 26_810_000 picoseconds.
        Weight::from_parts(39_447_522, 13928)
            // Standard Error: 9_791
            .saturating_add(Weight::from_parts(725_432, 0).saturating_mul(s.into()))
            .saturating_add(T::DbWeight::get().reads(2_u64))
            .saturating_add(T::DbWeight::get().writes(2_u64))
    }
    /// Storage: `Scheduler::Lookup` (r:1 w:1)
    /// Proof: `Scheduler::Lookup` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
    /// Storage: `Scheduler::Agenda` (r:1 w:1)
    /// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(10463), added: 12938, mode: `MaxEncodedLen`)
    /// The range of component `s` is `[1, 50]`.
    fn cancel_named(s: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `280 + s * (185 ±0)`
        //  Estimated: `13928`
        // Minimum execution time: 34_750_000 picoseconds.
        Weight::from_parts(34_403_252, 13928)
            // Standard Error: 11_250
            .saturating_add(Weight::from_parts(1_246_814, 0).saturating_mul(s.into()))
            .saturating_add(T::DbWeight::get().reads(2_u64))
            .saturating_add(T::DbWeight::get().writes(2_u64))
    }

    fn schedule_retry(_foo: u32) -> Weight {
      Weight::from_parts(0, 0) // will be overwritten soon
    }

    fn set_retry() -> Weight {
      Weight::from_parts(0, 0) // will be overwritten soon
    }

    fn set_retry_named() -> Weight {
      Weight::from_parts(0, 0) // will be overwritten soon
    }

    fn cancel_retry() -> Weight {
      Weight::from_parts(0, 0) // will be overwritten soon
    }

    fn cancel_retry_named() -> Weight {
      Weight::from_parts(0, 0) // will be overwritten soon
    }
}
