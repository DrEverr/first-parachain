use frame::{
	deps::sp_runtime,
	testing_prelude::{assert_noop, assert_ok},
};

use crate::{
	mock::{new_test_ext, CounterPallet, RuntimeOrigin, System, Test},
	Error, Event, UserInteractions,
};

#[test]
fn set_value_works() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		assert_ok!(CounterPallet::set_counter_value(RuntimeOrigin::root(), 8));
	});
}

#[test]
fn increments_works() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		assert_ok!(CounterPallet::set_counter_value(RuntimeOrigin::root(), 0));
		assert_ok!(CounterPallet::increment(RuntimeOrigin::signed(1), 5));
		System::assert_last_event(
			Event::CounterIncremented { counter_value: 5, who: 1, incremented_amount: 5 }.into(),
		);
	});
}

#[test]
fn increment_fails_for_max_value_exceeded() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		assert_ok!(CounterPallet::set_counter_value(RuntimeOrigin::root(), 7));
		assert_noop!(
			CounterPallet::increment(RuntimeOrigin::signed(1), 4),
			Error::<Test>::CounterValueExceedsMax
		);
	});
}

#[test]
fn increment_fail_for_overflow() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		assert_ok!(CounterPallet::set_counter_value(RuntimeOrigin::root(), 1));
		assert_noop!(
			CounterPallet::increment(RuntimeOrigin::signed(1), u32::MAX),
			Error::<Test>::CounterOverflow
		);
	});
}

#[test]
fn decrements_works() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		assert_ok!(CounterPallet::set_counter_value(RuntimeOrigin::root(), 5));
		assert_ok!(CounterPallet::decrement(RuntimeOrigin::signed(1), 3));
		System::assert_last_event(
			Event::CounterDecremented { counter_value: 2, who: 1, decremented_amount: 3 }.into(),
		);
	});
}

#[test]
fn decrement_fails_for_below_zero() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		assert_ok!(CounterPallet::set_counter_value(RuntimeOrigin::root(), 2));
		assert_noop!(
			CounterPallet::decrement(RuntimeOrigin::signed(1), u32::MAX),
			Error::<Test>::CounterValueBelowZero
		);
	});
}

#[test]
fn set_counter_fails_for_non_root() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		assert_noop!(
			CounterPallet::set_counter_value(RuntimeOrigin::signed(1), 1),
			sp_runtime::traits::BadOrigin
		);
	});
}

#[test]
fn set_counter_fails_for_max_value_exceeded() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		assert_noop!(
			CounterPallet::set_counter_value(RuntimeOrigin::root(), u32::MAX),
			Error::<Test>::CounterValueExceedsMax
		);
	})
}

#[test]
fn user_interaction_increment() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		assert_ok!(CounterPallet::set_counter_value(RuntimeOrigin::root(), 0));
		assert_ok!(CounterPallet::increment(RuntimeOrigin::signed(1), 5));
		assert_ok!(CounterPallet::decrement(RuntimeOrigin::signed(1), 2));
		assert_eq!(UserInteractions::<Test>::get(1).unwrap_or(0), 2);
	});
}

#[test]
fn user_interaction_overflow() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		assert_ok!(CounterPallet::set_counter_value(RuntimeOrigin::root(), 5));
		UserInteractions::<Test>::set(1, Some(u32::MAX));
		assert_noop!(
			CounterPallet::increment(RuntimeOrigin::signed(1), 1),
			Error::<Test>::UserInteractionOverflow
		);
	});
}
