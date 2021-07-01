#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://substrate.dev/docs/en/knowledgebase/runtime/frame>

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	use frame_system::pallet_prelude::*;
	use frame_support::{dispatch::DispatchResult, pallet_prelude::*, inherent::Vec, codec::{Encode, Decode}};

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[derive(Encode, Decode, Default, Clone, PartialEq)]
	pub struct AnchoredDocument<T: Config> {
		file_hash: Vec<u8>,
		proposer_account: T::AccountId,
		proposer_did: Vec<u8>,
		required_quorum: u128,
		template_id: u128,
		metadata: Vec<u8>,
		valid_until: u128,
		status: u8,
		counterparty_addresses: Vec<T::AccountId>,
		accepted_counterparties: u128,
		declined_counterparties: u128,
	}

	// The pallet's runtime storage items.
	// https://substrate.dev/docs/en/knowledgebase/runtime/storage
	#[pallet::storage]
	#[pallet::getter(fn get_anchor)]
	pub(super) type Anchor<T> = StorageMap<_, Twox64Concat, Vec<u8>, AnchoredDocument<T>>;

	// Pallets use events to inform users when important changes are made.
	// https://substrate.dev/docs/en/knowledgebase/runtime/events
	#[pallet::event]
	#[pallet::metadata(T::AccountId = "AccountId")]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// [proposer, proposerDid, FileHash]
		DocumentAnchored(T::AccountId, Vec<u8>, Vec<u8>)
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T:Config> Pallet<T> {
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn add_document(origin: OriginFor<T>, file_hash: Vec<u8>, proposer_did: Vec<u8>) -> DispatchResult {
			let who = ensure_signed(origin)?;
			Self::deposit_event(Event::DocumentAnchored(who.clone(), file_hash.clone(), file_hash.clone()));

			let new_document = AnchoredDocument {
				file_hash: file_hash.clone(),
				proposer_account: who.clone(),
				proposer_did: proposer_did.clone(),
				required_quorum: 0,
				template_id: 0,
				metadata: Vec::new(),
				valid_until: 0,
				status: 0,
				counterparty_addresses: Vec::new(),
				accepted_counterparties: 0,
				declined_counterparties: 0
			};

			<Anchor<T>>::insert(proposer_did.clone(), new_document);

			Ok(())
		}
	}
}
