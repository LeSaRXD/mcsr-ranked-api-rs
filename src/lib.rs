use serde::Deserialize;

#[cfg(feature = "achievements")]
pub mod achievement;
#[cfg(feature = "matches")]
pub mod game;
pub(crate) mod helpers;
pub mod user;

pub type Elo = u16;
pub type EloChange = i16;
pub type Rank = u32;
pub type Season = u8;
pub type MatchId = u64;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub struct Time(u64);

impl Time {
	pub const fn millis(&self) -> u64 {
		self.0
	}
	pub const fn seconds(&self) -> u64 {
		self.0 / 1000
	}
	pub const fn minutes(&self) -> u64 {
		self.0 / 60000
	}
	pub const fn hours(&self) -> u64 {
		self.0 / 3600000
	}
}
