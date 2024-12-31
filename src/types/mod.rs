use core::fmt;
use std::marker::PhantomData;

use serde::{
	de::{self, MapAccess, Visitor},
	Deserialize, Deserializer,
};

#[cfg(test)]
mod tests;

pub type Elo = u16;
pub type EloChange = i16;
pub type Rank = u32;
pub type Season = u8;
pub type MatchId = u64;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub struct Time(pub u64);

impl Time {
	pub const fn new(value: u64) -> Self {
		Self(value)
	}
	pub const fn millis(&self) -> u64 {
		self.0 % 1000
	}
	pub const fn seconds(&self) -> u64 {
		(self.0 / 1000) % 60
	}
	pub const fn minutes(&self) -> u64 {
		(self.0 / 60000) % 60
	}
	pub const fn hours(&self) -> u64 {
		self.0 / 3600000
	}
}
