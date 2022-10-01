use crate::{mock::*, Error, Student, StudentId};
use frame_support::{assert_noop, assert_ok};
use crate::Pallet as Demo;
#[test]
fn createstudent() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		let name = b"nguyenvanteo".to_vec();
		let age = 22u8.into();
		//let student = Demo::create_student(Origin::signed(1), name, age);
		assert_ok!(Demo::<Test>::create_student(Origin::signed(1), name, age));

		assert_eq!(Demo::<Test>::student_id(), 1);
		// Read pallet storage and assert an expected result.
		//assert_eq!(Demo::StudentId(), 1);
	});
}
#[test]
fn updateage(){
	new_test_ext().execute_with(|| {
		let name = b"nguyenvanteo".to_vec();
		let age = 22u8.into();
		//let student = Demo::create_student(Origin::signed(1), name, age);
		assert_ok!(Demo::<Test>::create_student(Origin::signed(1), name, age));

		let new_age = 24u8.into();
		//let id = StudentId::<Test>::get();
		//let student = Demo::create_student(Origin::signed(1), name, age);
		assert_ok!(Demo::<Test>::update_age(Origin::signed(1), 0, new_age));

		assert_eq!(Demo::<Test>::student_id(), 1);
		// Read pallet storage and assert an expected result.
		//assert_eq!(Demo::StudentId(), 1);
	});
}
#[test]
fn updatename(){
	new_test_ext().execute_with(|| {
		let name = b"nguyenvanteo".to_vec();
		let age = 22u8.into();
		//let student = Demo::create_student(Origin::signed(1), name, age);
		assert_ok!(Demo::<Test>::create_student(Origin::signed(1), name, age));

		let new_name = b"nguyenvanty".to_vec();
		//let id = StudentId::<Test>::get();
		//let student = Demo::create_student(Origin::signed(1), name, age);
		assert_ok!(Demo::<Test>::update_name(Origin::signed(1), 0, new_name));

		assert_eq!(Demo::<Test>::student_id(), 1);
		// Read pallet storage and assert an expected result.
		//assert_eq!(Demo::StudentId(), 1);
	});
}
// #[test]
// fn correct_error_for_none_value() {
// 	new_test_ext().execute_with(|| {
// 		// Ensure the expected error is thrown when no value is present.
// 		assert_noop!(Demo::cause_error(Origin::signed(1)), Error::<Test>::NoneValue);
// 	});
// }