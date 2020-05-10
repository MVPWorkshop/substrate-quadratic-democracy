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
// use sp_runtime::traits::{Zero, IntegerSquareRoot};
// use sp_std::ops::{Add, Mul, Div, Rem};
// use crate::Tally;


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
