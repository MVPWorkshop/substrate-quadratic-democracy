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

//! Voting weight.

#[cfg(feature = "std")]
use serde::{Serialize, Deserialize};
use codec::{Encode, Decode};
use sp_runtime::traits::{Zero, IntegerSquareRoot};
use crate::{AccountVote, Delegations};
use crate::vote::AccountVoteWeight;


/// A means of determining what is weight of the vote.
#[derive(Clone, Copy, PartialEq, Eq, Encode, Decode, sp_runtime::RuntimeDebug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum VoteWeight {
	/// A standard way of weight e.g. one coin, one vote
	Standard,
	/// Quadratic way of calculating vote weight
	Quadratic,

	// add more as needed
}

pub trait Calculate<Balance> {
	fn calculate(&self, vote: AccountVote<Balance>) -> AccountVoteWeight<Balance>;
	// @TODO possible rename to votes
	fn delegation(&self, vote: Delegations<Balance>) -> Delegations<Balance>;
}

impl<
	Balance: From<u8> + Zero + Copy + IntegerSquareRoot
> Calculate<Balance> for VoteWeight {
	fn calculate(&self, vote: AccountVote<Balance>) -> AccountVoteWeight<Balance> {
		match *self {
			// VoteWeight::Standard => vote,
			VoteWeight::Quadratic => {
				match vote {
					AccountVote::Standard { vote, balance } => {
						AccountVoteWeight::Standard {
							vote,
							balance,
							weighted_balance: balance.integer_sqrt(),
						}
					}
					AccountVote::Split { aye, nay } => {
						AccountVoteWeight::Split {
							aye,
							nay,
							aye_weight: aye.integer_sqrt(),
							nay_weight: nay.integer_sqrt(),
						}
					}
				}
			}
			VoteWeight::Standard => {
				match vote {
					AccountVote::Standard { vote, balance } => {
						AccountVoteWeight::Standard {
							vote,
							balance,
							weighted_balance: balance,
						}
					}
					AccountVote::Split { aye, nay } => {
						AccountVoteWeight::Split {
							aye,
							nay,
							aye_weight: aye,
							nay_weight: nay,
						}
					}
				}
			}
		}
	}

	fn delegation(&self, delegation: Delegations<Balance>) -> Delegations<Balance> {
		let Delegations {capital, votes} = delegation;
		match *self {
			// VoteWeight::Standard => vote,
			VoteWeight::Quadratic => {
				Delegations {
					capital,
					votes: votes.integer_sqrt()
				}
			}
			VoteWeight::Standard => {
				Delegations {
					capital,
					votes
				}
			}
		}
	}
}
