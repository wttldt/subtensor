#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarks;
pub mod weights;

pub use pallet::*;
pub use weights::WeightInfo;

use sp_runtime::traits::{Dispatchable};
use frame_support::dispatch::{PostDispatchInfo, GetDispatchInfo};

struct Vote {

}

enum Member {
	Triumvirate(),
	SubnetOwner,
	RootNetwork
}

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	#[pallet::pallet]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		/// The runtime call dispatch type.
		type Proposal: Parameter
		+ Dispatchable<
			RuntimeOrigin = <Self as frame_system::Config>::RuntimeOrigin,
			PostInfo = PostDispatchInfo,
		> + From<frame_system::Call<Self>>
		+ GetDispatchInfo;
	
		// Weight information for extrinsics in this pallet.
		type WeightInfo: WeightInfo;
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
	}

	#[pallet::error]
	pub enum Error<T> {
	}


	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight((
			{0}, 
			DispatchClass::Operational
		))]
		pub fn vote(origin: OriginFor<T>) -> DispatchResult {
			Ok(())
		}

	}
}