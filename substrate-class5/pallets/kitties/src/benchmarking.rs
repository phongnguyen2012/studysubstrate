//! Benchmarking setup for pallet-template
//! Benchmarking setup for pallet-template

use super::*;

#[allow(unused)]

use crate::Pallet as Kitties;
use frame_benchmarking::{benchmarks, whitelisted_caller, account};
use frame_system::RawOrigin;

benchmarks! {

	// create_kitty {
	// 	let dna = b"phongnvphongnv".to_vec();
	// 	let caller: T::AccountId = whitelisted_caller();
	// }: _(RawOrigin::Signed(caller), dna)

	// verify{
	// 	assert_eq!(KittyId::<T>::get(), 1);
	// }
	
	// impl_benchmark_test_suite!(Kitties, crate::mock::new_test_ext(), crate::mock::Test);


	transfer {
		let caller: T::AccountId = whitelisted_caller();
		let reciever: T::AccountId = whitelisted_caller();
		let to: T::AccountId = account("reciever", 0, 0);
		let dnas = b"phongnvphongnv".to_vec();
		Kitties::<T>::create_kitty(RawOrigin::Signed(caller.clone()).into(),dnas.clone() )?;
		let kitti_owner = Kitties::<T>::kitty_owned(caller.clone());
		let dna_hash = kitti_owner[0];
	}: _(RawOrigin::Signed(caller), to, dna_hash)


	impl_benchmark_test_suite!(Kitties, crate::mock::new_test_ext(), crate::mock::Test);
}