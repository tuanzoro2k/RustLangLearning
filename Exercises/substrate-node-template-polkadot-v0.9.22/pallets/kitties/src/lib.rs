#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/v3/runtime/frame>
pub use pallet::*;

// #[cfg(test)]
// mod mock;

// #[cfg(test)]
// mod tests;

// #[cfg(feature = "runtime-benchmarks")]
// mod benchmarking;
use frame_support::dispatch::fmt;
use frame_support::inherent::Vec;
use frame_support::pallet_prelude::*;
use frame_system::pallet_prelude::*;
#[frame_support::pallet]
pub mod pallet {
	pub use super::*;
	#[derive(TypeInfo, Default, Encode, Decode)]
	#[scale_info(skip_type_params(T))]
	pub struct Kitty<T: Config> {
		dna: Vec<u8>,
		owner: T::AccountId,
		price: u8,
		gender: Gender,
	}
	#[derive(TypeInfo, Encode, Decode, Debug)]
	pub enum Gender {
		Male,
		Female,
	}
	impl Default for Gender {
		fn default() -> Self {
			Gender::Male
		}
	}

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	// The pallet's runtime storage items.
	// https://docs.substrate.io/v3/runtime/storage
	#[pallet::storage]
	#[pallet::getter(fn num_of_kitty)]
	// Learn more about declaring storage items:
	// https://docs.substrate.io/v3/runtime/storage#declaring-storage-items
	pub type NumOfKitty<T> = StorageValue<_, u32, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn kitty)]
	pub(super) type Kittys<T: Config> =
		StorageMap<_, Blake2_128Concat, Vec<u8>, Kitty<T>, OptionQuery>;
	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/v3/runtime/events-and-errors
	#[pallet::storage]
	#[pallet::getter(fn owner_of_kitty)]
	pub(super) type Owner<T: Config> =
		StorageMap<_, Blake2_128Concat, T::AccountId, Vec<Vec<u8>>, ValueQuery>;
	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/v3/runtime/events-and-errors
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		KittyStored(Vec<u8>, u8),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		AlreadyOwned,
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.

	//extrinsic
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn create_kitty(origin: OriginFor<T>, dna: Vec<u8>, price: u8) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/v3/runtime/origins
			let who = ensure_signed(origin)?;
			let gender = Self::gen_gender(dna.clone())?;
			let _kitty = Kitty::<T> { dna: dna.clone(), price, gender, owner: who.clone() };
			let mut current_id = <NumOfKitty<T>>::get();
			let mut list_dna = <Owner<T>>::get(who.clone());
			list_dna.push(dna.clone());
			<Owner<T>>::insert(who.clone(), list_dna);
			// Student::<T>::insert(current_id, student);
			<Kittys<T>>::insert(dna.clone(), _kitty);
			current_id += 1;
			NumOfKitty::<T>::put(current_id);

			// Emit an event.
			Self::deposit_event(Event::KittyStored(dna, price));
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn change_owner(
			origin: OriginFor<T>,
			dna: Vec<u8>,
			new_ower: T::AccountId,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let kitty = <Kittys<T>>::get(dna.clone());
			let kitty = kitty.unwrap();
			ensure!(kitty.owner == who, Error::<T>::AlreadyOwned);
			let kitty = Kitty::<T> {
				dna: kitty.dna,
				price: kitty.price,
				gender: kitty.gender,
				owner: new_ower,
			};
			<Kittys<T>>::insert(kitty.dna.clone(), kitty);
			Ok(())
		}
	}
}
impl<T> Pallet<T> {
	fn gen_gender(dna: Vec<u8>) -> Result<Gender, Error<T>> {
		let mut res = Gender::Female;
		if dna.len() % 2 == 0 {
			res = Gender::Male;
		}
		Ok(res)
	}
}
