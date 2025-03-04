use core::fmt;

use chrono::{DateTime, Utc};
use serde::de::{self, MapAccess, Visitor};
use serde::{Deserialize, Deserializer};

use crate::helpers::NextUnixTimestamp;
use crate::types::{Rank, Season};

#[cfg(test)]
mod tests;

/// Achievement type and the type's associated data
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AchievementData {
	BestTime,
	HighestWinStreak,
	PlayedMatches,
	Playtime,
	Wins,
	SeasonOutcome { season: Season, rank: Rank },
	PlayoffsOutcome { season: Season },
	WeeklyRace { count: u16 },
	SummonWither,
	IronPickless,
	Oneshot,
	Overtake,
	Foodless,
	ClassicRun,
	Netherite,
	Armorless,
	HighLevel,
	EgapHolder,
	IronHoe,
	Secret { id: Box<str>, data: Box<[Box<str>]> },
}

pub type AchievementLevel = u64;
pub type AchievementGoal = u64;

/// User's achievement. `data` contains achievement type and associated data
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Achievement {
	date: DateTime<Utc>,
	data: AchievementData,
	level: AchievementLevel,
	goal: Option<AchievementGoal>,
}

impl Achievement {
	/// The date the achievement was acquired
	pub fn date(&self) -> DateTime<Utc> {
		self.date
	}
	/// The type and data of the achievement
	pub fn data(&self) -> &AchievementData {
		&self.data
	}
	/// The level of the achievement
	pub fn level(&self) -> AchievementLevel {
		self.level
	}
	/// The goal to level up the achievement
	pub fn goal(&self) -> Option<AchievementGoal> {
		self.goal
	}
}

impl<'de> Deserialize<'de> for Achievement {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		enum Field {
			Id,
			Date,
			Data,
			Level,
			Goal,
		}

		impl<'de> Deserialize<'de> for Field {
			fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
			where
				D: Deserializer<'de>,
			{
				struct FieldVisitor;

				impl Visitor<'_> for FieldVisitor {
					type Value = Field;

					fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
						formatter.write_str("`id`, `date`, `data`, `level` or `goal`")
					}

					fn visit_str<E>(self, value: &str) -> Result<Field, E>
					where
						E: de::Error,
					{
						match value {
							"id" => Ok(Field::Id),
							"date" => Ok(Field::Date),
							"data" => Ok(Field::Data),
							"level" => Ok(Field::Level),
							"goal" => Ok(Field::Goal),
							_ => Err(de::Error::unknown_field(value, FIELDS)),
						}
					}
				}

				deserializer.deserialize_identifier(FieldVisitor)
			}
		}

		struct AchievementVisitor;

		impl<'de> Visitor<'de> for AchievementVisitor {
			type Value = Achievement;

			fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
				formatter.write_str("struct Achievement")
			}

			fn visit_map<V>(self, mut map: V) -> Result<Achievement, V::Error>
			where
				V: MapAccess<'de>,
			{
				use AchievementData as AD;

				let mut id: Option<Box<str>> = None;
				let mut date: Option<DateTime<Utc>> = None;
				let mut data: Option<Box<[Box<str>]>> = None;
				let mut level: Option<u64> = None;
				let mut goal: Option<u64> = None;

				while let Some(key) = map.next_key()? {
					match key {
						Field::Id => {
							if id.is_some() {
								return Err(de::Error::duplicate_field("id"));
							}
							id = Some(map.next_value()?);
						}
						Field::Date => {
							if date.is_some() {
								return Err(de::Error::duplicate_field("date"));
							}
							date = Some(map.next_unix_timestamp()?);
						}
						Field::Data => {
							if data.is_some() {
								return Err(de::Error::duplicate_field("data"));
							}
							data = Some(map.next_value()?);
						}
						Field::Level => {
							if level.is_some() {
								return Err(de::Error::duplicate_field("level"));
							}
							level = Some(map.next_value()?);
						}
						Field::Goal => {
							if goal.is_some() {
								return Err(de::Error::duplicate_field("goal"));
							}
							goal = Some(map.next_value()?);
						}
					}
				}

				let id = id.ok_or_else(|| de::Error::missing_field("id"))?;
				let date = date.ok_or_else(|| de::Error::missing_field("date"))?;
				let data = data.ok_or_else(|| de::Error::missing_field("data"))?;
				let level = level.ok_or_else(|| de::Error::missing_field("level"))?;

				let achievment_data = match id.as_ref() {
					"bestTime" => AD::BestTime,
					"highestWinStreak" => AD::HighestWinStreak,
					"playedMatches" => AD::PlayedMatches,
					"playtime" => AD::Playtime,
					"wins" => AD::Wins,
					"seasonResult" => match data.as_ref() {
						[season_str, rank_str] => match (season_str.parse(), rank_str.parse()) {
							(Ok(season), Ok(rank)) => {
								AchievementData::SeasonOutcome { season, rank }
							}
							(Err(_), _) => {
								return Err(de::Error::invalid_type(
									de::Unexpected::Str(season_str),
									&"season number",
								))
							}
							(_, Err(_)) => {
								return Err(de::Error::invalid_type(
									de::Unexpected::Str(rank_str),
									&"rank number",
								))
							}
						},
						other_data => {
							return Err(de::Error::invalid_length(other_data.len(), &"2"))
						}
					},
					"playoffsResult" => match data.as_ref() {
						[season_str] => match season_str.parse() {
							Ok(season) => AD::PlayoffsOutcome { season },
							Err(_) => {
								return Err(de::Error::invalid_type(
									de::Unexpected::Str(season_str),
									&"season number",
								))
							}
						},
						other_data => {
							return Err(de::Error::invalid_length(other_data.len(), &"1"))
						}
					},
					"summonWither" => AD::SummonWither,
					"ironPickless" => AD::IronPickless,
					"oneshot" => AD::Oneshot,
					"overtake" => AD::Overtake,
					"foodless" => AD::Foodless,
					"classicRun" => AD::ClassicRun,
					"netherite" => AD::Netherite,
					"armorless" => AD::Armorless,
					"highLevel" => AD::HighLevel,
					"egapHolder" => AD::EgapHolder,
					"ironHoe" => AD::IronHoe,
					other_id => AD::Secret {
						id: other_id.into(),
						data,
					},
				};

				Ok(Achievement {
					date,
					data: achievment_data,
					level,
					goal,
				})
			}
		}

		const FIELDS: &[&str] = &["id", "date", "data", "level", "goal"];
		deserializer.deserialize_struct("Achievement", FIELDS, AchievementVisitor)
	}
}
