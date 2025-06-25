use std::{error::Error, fmt::Display, num::NonZeroU8};

use serde::Serialize;

use crate::types::MatchId;

const MAX_COUNT: u8 = 100;

#[cfg(test)]
mod tests;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Pagination {
	pub count: NonZeroU8,
	#[serde(flatten)]
	pub position: RelativePos,
}

impl Default for Pagination {
	fn default() -> Self {
		Self {
			count: const { NonZeroU8::new(20).unwrap() },
			position: Default::default(),
		}
	}
}

impl Pagination {
	/// Attempt to construct a pagination given a `count` and `position`
	pub fn new(count: u8, position: RelativePos) -> Option<Self> {
		NonZeroU8::new(count)
			.filter(|_| count <= MAX_COUNT)
			.map(|count| Self { count, position })
	}

	/// Create a new pagination given a `count` and `position`
	/// without checking for `count` bounds
	///
	/// # Safety
	/// this function calls `NonZeroU8::new_unchecked`.
	///
	/// Passing `count = 0` will result in undefined behavior.
	///
	/// Passing `count` > [`MAX_COUNT`] will result in an API error
	pub unsafe fn new_unchecked(count: u8, position: RelativePos) -> Self {
		Self {
			count: unsafe { NonZeroU8::new_unchecked(count) },
			position,
		}
	}

	/// Attempt to construct a pagination given a `count` without relative position
	pub fn count(count: u8) -> Option<Self> {
		NonZeroU8::new(count).map(|count| Self {
			count,
			..Default::default()
		})
	}

	/// Create a new pagination given a `count` without a relative position
	/// without checking for `count` bounds
	///
	/// # Safety
	/// This function calls `NonZeroU8::new_unchecked`.
	///
	/// Passing `count = 0` will result in undefined behavior.
	///
	/// Passing `count` > [`MAX_COUNT`] will result in an API error
	pub unsafe fn count_unchecked(count: u8) -> Self {
		Self {
			count: unsafe { NonZeroU8::new_unchecked(count) },
			..Default::default()
		}
	}
}

impl From<RelativePos> for Pagination {
	fn from(position: RelativePos) -> Self {
		Self {
			position,
			..Default::default()
		}
	}
}

#[derive(Debug, Clone)]
pub struct RelativePosError {
	/// The `before` match id
	pub before: MatchId,
	/// The `after` match id
	pub after: MatchId,
}
impl Display for RelativePosError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"Cannot create RelativePos, before = {} > after = {}",
			self.before, self.after
		)
	}
}
impl Error for RelativePosError {}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub struct RelativePos {
	#[serde(skip_serializing_if = "Option::is_none")]
	before: Option<MatchId>,
	#[serde(skip_serializing_if = "Option::is_none")]
	after: Option<MatchId>,
}
impl RelativePos {
	/// Creates a new [`RelativePos`] after checking whether [`before`, `after`] is a valid range.
	pub fn new_checked(before: MatchId, after: MatchId) -> Result<Self, RelativePosError> {
		if after > before {
			Err(RelativePosError { before, after })
		} else {
			Ok(Self {
				before: Some(before),
				after: Some(after),
			})
		}
	}
	/// Creates a new [`RelativePos`] without checking `before` and `after`.
	///
	/// If you are taking input from a user, it is recommended to use [`RelativePos::new_checked`] instead.
	pub fn new(before: MatchId, after: MatchId) -> Self {
		Self {
			before: Some(before),
			after: Some(after),
		}
	}
	/// Creates a [`RelativePos`] with only the `before` field set
	pub fn before(before: MatchId) -> Self {
		Self {
			before: Some(before),
			after: None,
		}
	}
	/// Creates a [`RelativePos`] with only the `after` field set
	pub fn after(after: MatchId) -> Self {
		Self {
			before: None,
			after: Some(after),
		}
	}
}
