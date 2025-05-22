use std::num::NonZeroU8;

use serde::Serialize;

use crate::types::MatchId;

const MAX_COUNT: u8 = 50;

#[cfg(test)]
mod tests;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Pagination {
	pub count: NonZeroU8,
	#[serde(flatten)]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub position: Option<RelativePos>,
}

impl Default for Pagination {
	fn default() -> Self {
		Self {
			count: const { NonZeroU8::new(20).unwrap() },
			position: None,
		}
	}
}

impl Pagination {
	/// Attempt to construct a pagination given a `count` and `position`
	pub fn new(count: u8, position: RelativePos) -> Option<Self> {
		NonZeroU8::new(count)
			.filter(|_| count <= MAX_COUNT)
			.map(|count| Self {
				count,
				position: Some(position),
			})
	}

	/// Create a new pagination given a `count` and `position`
	/// without checking for `count` bounds
	///
	/// # Safety
	/// this function calls `NonZeroU8::new_unchecked`. Passing a value outside the [1; 50] range for `count`
	/// will result in undefined behavior
	pub unsafe fn new_unchecked(count: u8, position: RelativePos) -> Self {
		Self {
			count: unsafe { NonZeroU8::new_unchecked(count) },
			position: Some(position),
		}
	}

	/// Attempt to construct a pagination given a `count` without relative position
	pub fn count(count: u8) -> Option<Self> {
		NonZeroU8::new(count).map(|count| Self {
			count,
			position: None,
		})
	}

	/// Create a new pagination given a `count` without a relative position
	/// without checking for `count` bounds
	///
	/// # Safety
	/// this function calls `NonZeroU8::new_unchecked`. Passing a value outside the [1; 50] range for `count`
	/// will result in undefined behavior
	pub unsafe fn count_unchecked(count: u8) -> Self {
		Self {
			count: unsafe { NonZeroU8::new_unchecked(count) },
			position: None,
		}
	}
}

impl From<RelativePos> for Pagination {
	fn from(position: RelativePos) -> Self {
		Self {
			position: Some(position),
			..Default::default()
		}
	}
}
impl From<Option<RelativePos>> for Pagination {
	fn from(position: Option<RelativePos>) -> Self {
		Self {
			position,
			..Default::default()
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum RelativePos {
	Before(MatchId),
	After(MatchId),
}
