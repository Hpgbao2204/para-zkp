#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

pub trait WeightInfo {
	fn set_zk_keys_benchmark() -> Weight;
	fn verify_benchmark() -> Weight;
}

/// Weight functions for `pallet_verifier`.
pub struct WeightInfoImpl<T>(PhantomData<T>);


impl<T: frame_system::Config> WeightInfo for WeightInfoImpl<T> {
	// Storage: Verifier Vkey (r:0 w:1)
	// Storage: Verifier PubSignal (r:0 w:1)
	fn set_zk_keys_benchmark() -> Weight {
		// Minimum execution time: 2_520_000 nanoseconds.
		Weight::from_parts(2_540_000_000, 0)
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Verifier PubSignal (r:1 w:0)
	// Storage: Verifier Vkey (r:1 w:0)
	fn verify_benchmark() -> Weight {
		// Minimum execution time: 18_808_000 nanoseconds.
		Weight::from_parts(18_983_000_000, 0)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
	}
}