use std::{error::Error, fmt::Display, num::NonZeroU8};

use serde::Serialize;

#[cfg(test)]
mod tests;

/// Pagination error type
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PaginationError {
	/// Incorrect page count
	Page(u8),
	/// Incorrect entry count
	Count(u8),
}

impl Display for PaginationError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		use PaginationError as Err;

		match self {
			Err::Page(p) => write!(f, "Page number {p} is not within bounds [0; 99]"),
			Err::Count(c) => write!(f, "Item count {c} is not within bounds [1; 50]"),
		}
	}
}
impl Error for PaginationError {}

/// Pagination struct for GET requests
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Pagination {
	/// Page number, 0-indexed
	///
	/// Default is 0
	page: u8,
	/// Item count
	///
	/// Default is 10
	count: NonZeroU8,
}

impl Default for Pagination {
	fn default() -> Self {
		Self {
			page: 0,
			count: const { NonZeroU8::new(10).unwrap() },
		}
	}
}

impl Pagination {
	/// Try to create a new pagination with `page` number and item `count`
	///
	/// # Limitations
	/// The `page` must be within the range \[0; 99\] inclusive
	///
	/// The `count` must be within the range \[1; 50\] inclusive
	pub fn new(page: u8, count: u8) -> Result<Self, PaginationError> {
		let opt_count = NonZeroU8::new(count);
		match (page, opt_count.map(|c| (c, c.get()))) {
			(0..=99, Some((count, 1..=50))) => Ok(Self { page, count }),
			(0..=99, _) => Err(PaginationError::Count(count)),
			(_, _) => Err(PaginationError::Page(page)),
		}
	}

	/// Try to crerate a new pagination with just the `page` number
	pub fn page(page: u8) -> Result<Self, PaginationError> {
		match page {
			0..=99 => Ok(Self {
				page,
				..Default::default()
			}),
			_ => Err(PaginationError::Page(page)),
		}
	}

	/// Try to crerate a new pagination with just the item `count`
	pub fn count(count: u8) -> Result<Self, PaginationError> {
		let opt_count = NonZeroU8::new(count);
		match opt_count.map(|c| (c, c.get())) {
			Some((count, 1..=50)) => Ok(Self {
				count,
				..Default::default()
			}),
			_ => Err(PaginationError::Count(count)),
		}
	}

	/// Create a new pagination, not checking for bounds
	///
	/// # Safety
	/// The `page` must be within the range \[0; 99\] inclusive
	///
	/// The `count` must be within the range \[1; 50\] inclusive
	///
	/// Breaking these bounds will result in a
	pub unsafe fn new_unchecked(page: u8, count: u8) -> Self {
		Self {
			page,
			count: unsafe { NonZeroU8::new_unchecked(count) },
		}
	}
}
