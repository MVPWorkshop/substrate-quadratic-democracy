// Copyright 2017-2020 Parity Technologies (UK) Ltd.
// This file is part of Substrate.

// Substrate is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Substrate is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Substrate.  If not, see <http://www.gnu.org/licenses/>.

//! The tests for the public proposal queue.

use super::*;

#[test]
fn backing_for_should_work() {
	new_test_ext().execute_with(|| {
		assert_ok!(propose_set_balance_and_note(1, 2, 2));
		assert_ok!(propose_set_balance_and_note(1, 4, 4));
		assert_ok!(propose_set_balance_and_note(1, 3, 3));
		assert_eq!(Democracy::backing_for(0), Some(2));
		assert_eq!(Democracy::backing_for(1), Some(4));
		assert_eq!(Democracy::backing_for(2), Some(3));
	});
}

#[test]
fn deposit_for_proposals_should_be_taken() {
	new_test_ext().execute_with(|| {
		assert_ok!(propose_set_balance_and_note(1, 2, 5));
		assert_ok!(Democracy::second(Origin::signed(2), 0));
		assert_ok!(Democracy::second(Origin::signed(5), 0));
		assert_ok!(Democracy::second(Origin::signed(5), 0));
		assert_ok!(Democracy::second(Origin::signed(5), 0));
		assert_eq!(Balances::free_balance(1), 95);
		assert_eq!(Balances::free_balance(2), 195);
		assert_eq!(Balances::free_balance(5), 485);
	});
}

#[test]
fn deposit_for_proposals_should_be_returned() {
	new_test_ext().execute_with(|| {
		assert_ok!(propose_set_balance_and_note(1, 2, 5));
		assert_ok!(Democracy::second(Origin::signed(2), 0));
		assert_ok!(Democracy::second(Origin::signed(5), 0));
		assert_ok!(Democracy::second(Origin::signed(5), 0));
		assert_ok!(Democracy::second(Origin::signed(5), 0));
		fast_forward_to(3);
		assert_eq!(Balances::free_balance(1), 100);
		assert_eq!(Balances::free_balance(2), 200);
		assert_eq!(Balances::free_balance(5), 500);
	});
}

#[test]
fn proposal_with_deposit_below_minimum_should_not_work() {
	new_test_ext().execute_with(|| {
		assert_noop!(propose_set_balance(1, 2, 0), Error::<Test>::ValueLow);
	});
}

#[test]
fn poor_proposer_should_not_work() {
	new_test_ext().execute_with(|| {
		assert_noop!(propose_set_balance(1, 2, 101), BalancesError::<Test, _>::InsufficientBalance);
	});
}

#[test]
fn poor_seconder_should_not_work() {
	new_test_ext().execute_with(|| {
		assert_ok!(propose_set_balance_and_note(2, 2, 101));
		assert_noop!(Democracy::second(Origin::signed(1), 0), BalancesError::<Test, _>::InsufficientBalance);
	});
}

#[test]
fn runners_up_should_come_after() {
	new_test_ext().execute_with(|| {
		System::set_block_number(0);
		assert_ok!(propose_set_balance_and_note(1, 2, 20));
		assert_ok!(propose_set_balance_and_note(1, 4, 40));
		assert_ok!(propose_set_balance_and_note(1, 3, 30));
		fast_forward_to(2);
		assert_ok!(Democracy::vote(Origin::signed(1), 0, aye(1))); // 40
		fast_forward_to(4);
		assert_ok!(Democracy::vote(Origin::signed(1), 1, aye(1))); // 30
		fast_forward_to(6);
		assert_ok!(Democracy::vote(Origin::signed(1), 2, aye(1))); // 20
	});
}
