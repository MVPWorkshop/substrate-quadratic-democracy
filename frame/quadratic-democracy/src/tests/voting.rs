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

//! The tests for normal voting functionality.

use super::*;

#[test]
fn overvoting_should_fail() {
	new_test_ext().execute_with(|| {
		let r = begin_referendum();
		assert_noop!(Democracy::vote(Origin::signed(1), r, aye(2)), Error::<Test>::InsufficientFunds);
	});
}

#[test]
fn split_voting_should_work() {
	new_test_ext().execute_with(|| {
		let r = begin_referendum();
		let v = AccountVote::Split { aye: 400, nay: 200};
		assert_noop!(Democracy::vote(Origin::signed(5), r, v), Error::<Test>::InsufficientFunds);
		let v = AccountVote::Split { aye: 300, nay: 200 };
		assert_ok!(Democracy::vote(Origin::signed(5), r, v));

		// sqrt(300) -> ~17 * 0.1 (Conviction) -> 1
		// sqrt(200) -> ~14 * 0.1 (Conviction) -> 1
		assert_eq!(tally(r), Tally { ayes: 1, nays: 1, turnout: 500 });
	});
}

#[test]
fn split_vote_cancellation_should_work() {
	new_test_ext().execute_with(|| {
		let r = begin_referendum();
		let v = AccountVote::Split { aye: 300, nay: 200 };
		assert_ok!(Democracy::vote(Origin::signed(5), r, v));
		assert_ok!(Democracy::remove_vote(Origin::signed(5), r));
		assert_eq!(tally(r), Tally { ayes: 0, nays: 0, turnout: 0 });
		assert_ok!(Democracy::unlock(Origin::signed(5), 5));
		assert_eq!(Balances::locks(5), vec![]);
	});
}

#[test]
fn single_proposal_should_work() {
	new_test_ext().execute_with(|| {
		System::set_block_number(0);
		assert_ok!(propose_set_balance_and_note(1, 2, 1));
		let r = 0;
		assert!(Democracy::referendum_info(r).is_none());

		// start of 2 => next referendum scheduled.
		fast_forward_to(2);
		assert_ok!(Democracy::vote(Origin::signed(1), r, aye(1)));

		assert_eq!(Democracy::referendum_count(), 1);
		assert_eq!(
			Democracy::referendum_status(0),
			Ok(ReferendumStatus {
				end: 4,
				proposal_hash: set_balance_proposal_hash_and_note(2),
				threshold: VoteThreshold::SuperMajorityApprove,
				weight: VoteWeight::Quadratic,
				delay: 2,
				tally: Tally { ayes: 1, nays: 0, turnout: 100 },
			})
		);

		fast_forward_to(3);

		// referendum still running
		assert!(Democracy::referendum_status(0).is_ok());

		// referendum runs during 2 and 3, ends @ start of 4.
		fast_forward_to(4);

		assert!(Democracy::referendum_status(0).is_err());
		assert!(pallet_scheduler::Agenda::<Test>::get(6)[0].is_some());

		// referendum passes and wait another two blocks for enactment.
		fast_forward_to(6);

		assert_eq!(Balances::free_balance(42), 2);
	});
}

#[test]
fn controversial_voting_should_work() {
	new_test_ext().execute_with(|| {
		let r = Democracy::inject_referendum(
			2,
			set_balance_proposal_hash_and_note(2),
			VoteThreshold::SuperMajorityApprove,
			VoteWeight::Quadratic,
			0
		);

		assert_ok!(Democracy::vote(Origin::signed(1), r, big_aye(1))); // 100 * 1 => 10
		assert_ok!(Democracy::vote(Origin::signed(2), r, big_nay(2))); // 200 * 1 => ~14
		assert_ok!(Democracy::vote(Origin::signed(3), r, big_nay(3))); // 300 * 1 => ~17
		assert_ok!(Democracy::vote(Origin::signed(4), r, big_aye(4))); // 400 * 1 => ~20
		assert_ok!(Democracy::vote(Origin::signed(5), r, big_nay(5))); // 500 * 1 => ~22
		assert_ok!(Democracy::vote(Origin::signed(6), r, big_aye(6))); // 600 * 1 => ~24

		// aye = 10 + 20 + 24 = 54
		// nay = 14 + 17 + 22 = 53

		assert_eq!(tally(r), Tally { ayes: 54, nays: 53, turnout: 2100 });

		next_block();
		next_block();

		assert_eq!(Balances::free_balance(42), 2);
	});
}

#[test]
fn controversial_low_turnout_voting_should_work() {
	new_test_ext().execute_with(|| {
		let r = Democracy::inject_referendum(
			2,
			set_balance_proposal_hash_and_note(2),
			VoteThreshold::SuperMajorityApprove,
			VoteWeight::Quadratic,
			0
		);
		assert_ok!(Democracy::vote(Origin::signed(5), r, big_nay(5))); // 500 * 1 => 22
		assert_ok!(Democracy::vote(Origin::signed(6), r, big_aye(6))); // 600 * 1 => 24

		assert_eq!(tally(r), Tally { ayes: 24, nays: 22, turnout: 1100 });

		next_block();
		next_block();

		assert_eq!(Balances::free_balance(42), 0);
	});
}

#[test]
fn passing_low_turnout_voting_should_work() {
	new_test_ext().execute_with(|| {
		assert_eq!(Balances::free_balance(42), 0);
		assert_eq!(Balances::total_issuance(), 2100);

		let r = Democracy::inject_referendum(
			2,
			set_balance_proposal_hash_and_note(2),
			VoteThreshold::SuperMajorityApprove,
			VoteWeight::Quadratic,
			0
		);
		assert_ok!(Democracy::vote(Origin::signed(4), r, big_aye(4)));
		assert_ok!(Democracy::vote(Origin::signed(5), r, big_nay(5)));
		assert_ok!(Democracy::vote(Origin::signed(6), r, big_aye(6)));
		assert_eq!(tally(r), Tally { ayes: 44, nays: 22, turnout: 1500 });

		next_block();
		next_block();
		assert_eq!(Balances::free_balance(42), 2);
	});
}
