#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;
use frame_support::pallet_prelude::*;
use frame_system::pallet_prelude::*;
use frame_support::dispatch::DispatchResult;
use frame_support::sp_runtime::traits::Hash;
use frame_support::sp_runtime::{SaturatedConversion, ArithmeticError};
use sp_std::vec::Vec;
use scale_info::TypeInfo;
pub type Id = u32;

use frame_support::traits::{Currency, Time, Randomness};
type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;


#[frame_support::pallet]
pub mod pallet {
	

pub use super::*;
	#[derive(TypeInfo, Encode, Decode, Clone, PartialEq, RuntimeDebug)]
	#[scale_info(skip_type_params(T))]
	pub struct Kitty<T:Config> {
		pub dna: T::Hash,
		pub price: BalanceOf<T>,
		pub gender: Gender,
		pub owner: T::AccountId,
		pub created_date: <<T as Config>::KittyTime as Time>::Moment,
	}


	#[derive(TypeInfo, Encode ,Decode, Clone, RuntimeDebug, PartialEq, MaxEncodedLen, Copy)]
	pub enum Gender {
		Male,
		Female,
	}

	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type Currency: Currency<Self::AccountId>;
		type KittyTime: Time;
		type KittyRandom: Randomness<Self::Hash, Self::BlockNumber>;
		type Max: Get<u32>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	// The pallet's runtime storage items.
	// https://docs.substrate.io/v3/runtime/storage
	#[pallet::storage]
	#[pallet::getter(fn kitty_id)]
	// Learn more about declaring storage items:
	// https://docs.substrate.io/v3/runtime/storage#declaring-storage-items
	pub type KittyId<T> = StorageValue<_, Id,ValueQuery>;


	// key : id
	//value : student
	#[pallet::storage]
	#[pallet::getter(fn kitty)]
	pub(super) type Kitties<T: Config> = StorageMap<_, Blake2_128Concat, T::Hash, Kitty<T>, OptionQuery>;


	#[pallet::storage]
	#[pallet::getter(fn ownerkitty)]
	pub(super) type KittiesOwned<T: Config> = StorageMap<_, Blake2_128Concat,T::AccountId , BoundedVec<T::Hash, T::Max>, ValueQuery>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/v3/runtime/events-and-errors
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T:Config> {
		Created {kitty: T::Hash, owner: T::AccountId},
		Transferred {from: T::AccountId, to: T::AccountId, kitty: T::Hash},
		
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		DuplicateKitty,
		TooManyOwned,
		NoKitty,
		NotOwner,
		TransferToSelf,
		TooCheap,
		CannotConvert,
		ExceedKittyNumber,
	}


	//extrinsic
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(0)]
		pub fn create_kitty(origin: OriginFor<T>, dna: Vec<u8>) -> DispatchResult {
			// Make sure the caller is from a signed origin
			let owner = ensure_signed(origin)?;
			log::info!("total balance:{:?}", T::Currency::total_balance(&owner));
			let gender = Self::gen_gender(&dna)?;
			let now = T::KittyTime::now();
			let dna = Self::gen_dna();
			let kitty =
				Kitty::<T> { dna: dna.clone(), price: 0u32.into(), gender, owner: owner.clone(), created_date:now };

			let max = T::Max::get();
			let get_kitties = KittiesOwned::<T>::get(&owner);
			ensure!((get_kitties.len() as u32) < max, Error::<T>::ExceedKittyNumber); 
			let convert = T::KittyTime::now().saturated_into::<u64>();
			let convert_moment: <<T as Config>::KittyTime as Time>::Moment = now.try_into().map_err(|_| Error::<T>::CannotConvert)?;
			ensure!(!Kitties::<T>::contains_key(&kitty.dna), Error::<T>::DuplicateKitty);

			let current_id = KittyId::<T>::get();
			let next_id = current_id.checked_add(1).ok_or(ArithmeticError::Overflow)?;

			// Append kitty to KittiesOwned
			KittiesOwned::<T>::try_append(&owner, kitty.dna.clone()).map_err(|_| Error::<T>::NoKitty)?;
			Kitties::<T>::insert(kitty.dna.clone(), kitty);
			KittyId::<T>::put(next_id);
			Self::deposit_event(Event::Created { kitty: dna, owner: owner.clone() });

			Ok(())
		}
		#[pallet::weight(0)]
		pub fn transfer(origin: OriginFor<T>, to: T::AccountId, dna: T::Hash) -> DispatchResult {
			// Make sure the caller is from a signed origin
			let from = ensure_signed(origin)?;
			let mut kitty = Kitties::<T>::get(&dna).ok_or(Error::<T>::NoKitty)?;
			ensure!(kitty.owner == from, Error::<T>::NotOwner);
			ensure!(from != to, Error::<T>::TransferToSelf);

			let mut from_owned = KittiesOwned::<T>::get(&from);

			// Remove kitty from list of owned kitties.
			if let Some(ind) = from_owned.iter().position(|ids| *ids == dna) {
				from_owned.swap_remove(ind);
			} else {
				return Err(Error::<T>::NoKitty.into())
			}

			let mut to_owned = KittiesOwned::<T>::get(&to);
			to_owned.try_push(dna.clone()).map_err(|_| Error::<T>::ExceedKittyNumber)?;
			kitty.owner = to.clone();

			// Write updates to storage
			Kitties::<T>::insert(&dna, kitty);
			KittiesOwned::<T>::insert(&to, to_owned);
			KittiesOwned::<T>::insert(&from, from_owned);

			Self::deposit_event(Event::Transferred { from, to, kitty: dna });

			Ok(())
		}
	}
}

impl<T:Config> Pallet<T> {
	fn gen_gender(dna: &Vec<u8>) -> Result<Gender, Error<T>> {
		let mut res = Gender::Female;
		if dna.len() % 2 == 0 {
			res = Gender::Male;
		}
		Ok(res)
	}


	fn gen_dna() -> T::Hash {
		let (seed,_) = T::KittyRandom::random_seed();
		let block_number = <frame_system::Pallet<T>>::block_number();
		T::Hashing::hash_of(&(seed, block_number))
	}
}
