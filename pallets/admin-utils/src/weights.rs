
//! Autogenerated weights for `pallet_admin_utils`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-12-01, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `rustys-MBP`, CPU: `<UNKNOWN>`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("local")`, DB CACHE: `1024`

// Executed Command:
// ./target/release/node-subtensor
// benchmark
// pallet
// --chain=local
// --execution=wasm
// --wasm-execution=compiled
// --pallet=pallet_admin_utils
// --extrinsic=*
// --steps
// 50
// --repeat
// 20
// --output=pallets/admin-utils/src/weights.rs
// --template=./.maintain/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for `pallet_admin_utils`.
pub trait WeightInfo {
	fn swap_authorities(a: u32, ) -> Weight;
}

/// Weights for `pallet_admin_utils` using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: System Digest (r:1 w:1)
	/// Proof Skipped: System Digest (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Aura Authorities (r:0 w:1)
	/// Proof: Aura Authorities (max_values: Some(1), max_size: Some(1025), added: 1520, mode: MaxEncodedLen)
	/// The range of component `a` is `[0, 32]`.
	fn swap_authorities(a: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `632`
		//  Estimated: `1127`
		// Minimum execution time: 6_000_000 picoseconds.
		Weight::from_parts(10_899_867, 1127)
			// Standard Error: 6_004
			.saturating_add(Weight::from_parts(189_570, 0).saturating_mul(a.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
}

// For backwards compatibility and tests.
impl WeightInfo for () {
	/// Storage: System Digest (r:1 w:1)
	/// Proof Skipped: System Digest (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Aura Authorities (r:0 w:1)
	/// Proof: Aura Authorities (max_values: Some(1), max_size: Some(1025), added: 1520, mode: MaxEncodedLen)
	/// The range of component `a` is `[0, 32]`.
	fn swap_authorities(a: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `632`
		//  Estimated: `1127`
		// Minimum execution time: 6_000_000 picoseconds.
		Weight::from_parts(10_899_867, 1127)
			// Standard Error: 6_004
			.saturating_add(Weight::from_parts(189_570, 0).saturating_mul(a.into()))
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
}