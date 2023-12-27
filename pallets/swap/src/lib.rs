#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;
pub mod weights;
pub use weights::WeightInfo;

use sp_runtime::RuntimeDebug;
use frame_support::pallet_prelude::{Encode, Decode, TypeInfo, MaxEncodedLen};

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[derive(PartialEq, Eq, Clone, RuntimeDebug, Encode, Decode, TypeInfo, MaxEncodedLen)]
pub enum Token {
	TAO(u64),
	SubnetToken(u16, u64)
}

#[derive(PartialEq, Eq, Clone, Encode, Decode, RuntimeDebug, TypeInfo)]
pub struct Pool
{
	reserve0: 		Token,
	reserve1: 		Token,
	outstanding: 	Token,
	ratio: 			u64
}

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use frame_support::traits::tokens::Balance;
	use frame_support::dispatch::DispatchResult;

	#[pallet::pallet]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		// Weight information for extrinsics in this pallet.
		type WeightInfo: WeightInfo;

		type Balance: Balance;

		type MaxPools: Get<u16>;
	}

	/// Storage map for existing pools
	#[pallet::storage]
    pub type Pools<T: Config> = StorageMap<_, Identity, u16, Pool, OptionQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config>
	{}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T>
	{
		PoolAlreadyExists,
		InvalidPool,
		NotEnoughBalance
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(0)]
		pub fn swap(origin: OriginFor<T>, from: Token, to: Token) -> DispatchResult {

			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

		/// Contribute tokens to the pool without swapping
		#[pallet::call_index(1)]
		#[pallet::weight(0)]
		pub fn add_token(origin: OriginFor<T>, token: Token) -> DispatchResult {
			Ok(())
		}

		/// Contribute TAO to a pool without swapping
		#[pallet::call_index(2)]
		#[pallet::weight(0)]
		pub fn add_tao(origin: OriginFor<T>, token: Token, tao_amount: u64) -> DispatchResult {
			Ok(())
		}
    }

	impl<T: Config> Pallet<T> {
		/// Price of in_token per out_token
		pub fn price(from: Token, to: Token) -> u64 {
			0
		}

		/// Market cap of in_token as a function of out_token
		pub fn marketcap(from: Token, to: Token) -> u64 {
			0
		}

		pub fn new_pool(from: Token, to: Token) -> Result<u16, Error<T>> {
			Ok(0) // pool id
		}

		pub fn liquidate_pool(pool_id: u16) {

		}
	}
}