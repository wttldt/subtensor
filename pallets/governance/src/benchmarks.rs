//! Benchmarking setup
#![cfg(feature = "runtime-benchmarks")]
use super::*;

#[allow(unused)]
use crate::Pallet as Registry;
use frame_benchmarking::v2::*;
use frame_benchmarking::v1::account;
use frame_system::RawOrigin;

use sp_runtime::traits::{StaticLookup, Bounded};
use frame_support::traits::Get;
use sp_std::mem::size_of;

fn assert_last_event<T: Config>(generic_event: <T as Config>::RuntimeEvent) {
	frame_system::Pallet::<T>::assert_last_event(generic_event.into());
}

#[benchmarks]
mod benchmarks {
	use super::*;

	#[benchmark]
	fn set_identity() {
		// The target user
		let caller: T::AccountId = whitelisted_caller();
		let _ = T::Currency::make_free_balance_be(&caller, BalanceOf::<T>::max_value());

		#[extrinsic_call]
		_(RawOrigin::Signed(caller.clone()), account::<T::AccountId>("account", 0, 0u32), Box::new(create_identity_info::<T>(0)));

		assert_last_event::<T>(Event::<T>::IdentitySet { who: caller }.into());
	}
}
