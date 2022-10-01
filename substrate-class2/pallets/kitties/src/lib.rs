#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

use frame_support::pallet_prelude::*;
use frame_system::pallet_prelude::*;
use frame_support::sp_runtime::*;
use frame_support::dispatch::DispatchResult;
//use frame_support::inherent::Vec;
use sp_std::vec::Vec;
use scale_info::TypeInfo;
pub type Id = u32;
//use sp_runtime::ArithmeticError;
use frame_support::traits::Currency;
// use frame_support::dispatch::fmt;

#[frame_support::pallet]
pub mod pallet {
	pub use super::*;
	#[derive(TypeInfo, Encode, Decode, Clone, PartialEq, RuntimeDebug)]
	#[scale_info(skip_type_params(T))]
	pub struct Kitty<T:Config> {
		dna: Vec<u8>,
		price:u32,
		gender: Gender,
		owner: T::AccountId,
	}
	pub type Id = u32;

	#[derive(TypeInfo, Encode ,Decode, Clone, RuntimeDebug, PartialEq, MaxEncodedLen, Copy)]
	pub enum Gender {
		Male,
		Female,
	}

	impl Default for Gender{
		fn default()-> Self{
			Gender::Male
		}
	}
	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type Currency: Currency<Self::AccountId>;
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
	pub(super) type Kitties<T: Config> = StorageMap<_, Blake2_128Concat, Vec<u8>, Kitty<T>, OptionQuery>;


	#[pallet::storage]
	#[pallet::getter(fn ownerkitty)]
	pub(super) type KittiesOwned<T: Config> = StorageMap<_, Blake2_128Concat,T::AccountId , Vec<Vec<u8>>, ValueQuery>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/v3/runtime/events-and-errors
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T:Config> {
		Created {kitty: Vec<u8>, owner: T::AccountId},
		Transferred {from: T::AccountId, to: T::AccountId, kitty: Vec<u8>},
		
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
	}


	//extrinsic
	#[pallet::call]
	impl<T:Config> Pallet<T> {
		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn create_kitty(origin: OriginFor<T>,dna: Vec<u8>) -> DispatchResult {
			
			let owner = ensure_signed(origin)?;
			log::info!("total balance : {:?}", T::Currency::total_balance(&owner));
			let gender = Self::gen_gender(&dna)?;
			let kitty = Kitty::<T> {dna: dna.clone(), price: 0u32.into(), gender, owner: owner.clone()};

			ensure!(!Kitties::<T>::contains_key(&kitty.dna), Error::<T>::DuplicateKitty);

			let current_id = KittyId::<T>::get();
			let next_id = current_id.checked_add(1).ok_or(ArithmeticError::Overflow)?;

			
			Kitties::<T>::insert(kitty.dna.clone(), kitty);
			KittyId::<T>::put(next_id);
			Self::deposit_event(Event::Created {kitty: dna, owner: owner.clone()});
			Ok(())
		}
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn transfer(origin: OriginFor<T>, to: T::AccountId, dna: Vec<u8>) -> DispatchResult {
			let from = ensure_signed(origin)?;
			let mut kitty = Kitties::<T>::get(&dna).ok_or(Error::<T>::NoKitty)?;
			
			ensure!(kitty.owner == from, Error::<T>::NotOwner);
			ensure!(from != to, Error::<T>::TransferToSelf);
			let mut from_owned = KittiesOwned::<T>::get(&from);

			//remove kitty from list of owned kitties
			if let Some(ind) = from_owned.iter().position(|ids| *ids == dna) {
				from_owned.swap_remove(ind);
			}
			else{
				return Err(Error::<T>::NoKitty.into());
			}

			let mut to_owned = KittiesOwned::<T>::get(&to);
			to_owned.push(dna.clone());
			kitty.owner = to.clone();

			//write updates to storage
			Kitties::<T>::insert(&dna, kitty);
			KittiesOwned::<T>::insert(&from, from_owned);
			KittiesOwned::<T>::insert(&to, to_owned);

			Self::deposit_event(Event::Transferred { from, to, kitty: dna }); 

			Ok(())
		}
	}
}

// helper function

impl<T> Pallet<T> {
	fn gen_gender(dna: &Vec<u8>) -> Result<Gender,Error<T>>{
		let mut res = Gender::Female;
		if dna.len() % 2 ==0 {
			res = Gender::Male;
		}
		
		Ok(res)
	}

}
