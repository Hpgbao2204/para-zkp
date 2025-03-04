#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;
#[allow(unused_imports)]

pub mod parser;
pub mod types;
pub mod weights;

use parser::{parse_proof, parse_vkey};
use types::{ProofStr, VkeyStr};

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[frame_support::pallet]
pub mod pallet {
	use bls12_381::Bls12;
	use crate::{parse_proof, parse_vkey, ProofStr, VkeyStr};
	use crate::weights::WeightInfo;
	use frame_support::{
    pallet_prelude::*,
    dispatch::DispatchResult,
	};
	use frame_system::pallet_prelude::*;
	use core::str::from_utf8;
	use sp_std::vec::Vec;

	use bellman_verifier::verifier::{prepare_verifying_key, verify_proof};
	use ff::PrimeField as Fr;

	type PublicSignalStr = Vec<u8>;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type WeightInfo: crate::weights::WeightInfo;
	}

	#[pallet::pallet]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	/// Store the proof
	#[pallet::storage]
	pub type Pof<T: Config> = StorageValue<_, ProofStr, ValueQuery>;

	/// store the verification key
	#[pallet::storage]
	pub type Vkey<T: Config> = StorageValue<_, VkeyStr, ValueQuery>;

	/// store the public signal
	#[pallet::storage]
	pub type PubSignal<T: Config> = StorageValue<_, PublicSignalStr, ValueQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		VerificationKeyStored(T::AccountId, VkeyStr),
		PublicSignalStored(T::AccountId, PublicSignalStr),
		ProofStored(T::AccountId, ProofStr),
		VerificationPassed(T::AccountId),
	}


	#[pallet::error]
	pub enum Error<T> {
		/// incorrect verification key format
		ErrorVerificationKey,
		/// incorrect proof format
		ErrorProof,
		/// incorrect public signal format
		ErrorPublicSignal,
		/// If you want to verify the proof, but there is no Verification key
		NoVerificationKey,
		/// no public signal to verify
		NoPublicSignal,
		/// parse error with public signal
		ParsePulbicSignalError,
		/// invalid proof
		InvalidProof,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		// #[pallet::weight(Weight::default())]
		#[pallet::weight(T::WeightInfo::set_zk_keys_benchmark())]
		pub fn set_zk_keys(
			origin: OriginFor<T>,
			public_signal: Vec<u8>,
			vk_alpha1: Vec<u8>,
			vk_beta_2: Vec<u8>,
			vk_gamma_2: Vec<u8>,
			vk_delta_2: Vec<u8>,
			vk_ic0: Vec<u8>,
			vk_ic1: Vec<u8>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			let vkey = VkeyStr {
				alpha_1: vk_alpha1,
				beta_2: vk_beta_2,
				gamma_2: vk_gamma_2,
				delta_2: vk_delta_2,
				ic0: vk_ic0,
				ic1: vk_ic1,
			};
			// ensure valid public signal, public should not be none
			ensure!(!public_signal.is_empty(), Error::<T>::ErrorPublicSignal);

			// ensure the verification key's format is correct
			let _ = parse_vkey::<Bls12>(vkey.clone()).map_err(|_| Error::<T>::ErrorVerificationKey)?;

			<PubSignal<T>>::put(&public_signal);
			<Vkey<T>>::put(&vkey);

			Self::deposit_event(Event::<T>::PublicSignalStored(who.clone(), public_signal));
			Self::deposit_event(Event::<T>::VerificationKeyStored(who, vkey));
			Ok(())
			}
		/// verify the proof of snarkjs with groth16(bellman verification)
		#[pallet::call_index(1)]
		// #[pallet::weight(Weight::default())]
		#[pallet::weight(T::WeightInfo::verify_benchmark())]
		pub fn verify(
			origin: OriginFor<T>,
			proof_a: Vec<u8>,
			proof_b: Vec<u8>,
			proof_c: Vec<u8>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let public_signal = PubSignal::<T>::get();
			let vkeystr = Vkey::<T>::get();

			let pof = ProofStr { pi_a: proof_a, pi_b: proof_b, pi_c: proof_c };

			// parse proof and verification key
			let proof = parse_proof::<Bls12>(pof.clone()).map_err(|_| Error::<T>::ErrorProof)?;
			let vkey = parse_vkey::<Bls12>(vkeystr).map_err(|_| Error::<T>::ErrorVerificationKey)?;
			
			// prepare pre-verification key
			let pvk = prepare_verifying_key(&vkey);

			// prepare signal
			let public_str = from_utf8(&public_signal).map_err(|_| Error::<T>::ParsePulbicSignalError)?;

			// verify the proof
			ensure!(verify_proof(&pvk, &proof, &[Fr::from_str_vartime(public_str).unwrap()]).is_ok(), Error::<T>::InvalidProof);

			Self::deposit_event(Event::<T>::VerificationPassed(who.clone()));
			Self::deposit_event(Event::<T>::ProofStored(who, pof));

			Ok(())
		}
	}
}
