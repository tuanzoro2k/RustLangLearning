use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};

#[test]
fn error_age_too_young() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		let mut name = Vec::<u8>::new();
		name.push(123);
		assert_noop!(Demo::create_student(Origin::signed(1), name, 12), Error::<Test>::TooYoung);
		// Read pallet storage and assert an expected result.
	});
}
#[test]
fn ok_with_young() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		let mut name = Vec::<u8>::new();
		name.push(123);
		assert_ok!(Demo::create_student(Origin::signed(1), name, 21));
		// Read pallet storage and assert an expected result.
	});
}
