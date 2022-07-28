//! Benchmarking setup for pallet-template

use super::*;

#[allow(unused)]
use crate::Pallet as Kitties;
use frame_benchmarking::{account, benchmarks, whitelisted_caller};
use frame_system::RawOrigin;

benchmarks! {
	transfer {
		let dna = b"tuan".to_vec();
		let buyer: T::AccountId = whitelisted_caller();
		let buyer_origin = <T as frame_system::Config>::Origin::from(RawOrigin::Signed(buyer.clone()));
		let receiver: T::AccountId = account("tuan",0,0);
		Kitties::<T>::create_kitty(buyer_origin,dna.clone());
	}: _(RawOrigin::Signed(buyer), receiver,dna)
	verify {
		assert_eq!(KittyId::<T>::get(), 1);
	}

	impl_benchmark_test_suite!(Template, crate::mock::new_test_ext(), crate::mock::Test);
}
