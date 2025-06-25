use chrono::{serde::ts_seconds, DateTime, Utc};
use serde::{de, Deserialize, Deserializer};
#[cfg(feature = "serialize")]
use serde::{ser::SerializeMap, Serialize};

use crate::types::{Rank, Season};

#[cfg(test)]
mod tests;

pub type AchievementLevel = u64;
pub type AchievementGoal = u64;
pub type AchievementValue = u64;

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

#[cfg(feature = "serialize")]
impl AchievementData {
	fn id_data(&self) -> (&str, Box<[Box<str>]>) {
		let id = match self {
			Self::BestTime => "bestTime",
			Self::HighestWinStreak => "highestWinStreak",
			Self::PlayedMatches => "playedMatches",
			Self::Playtime => "playtime",
			Self::Wins => "wins",
			Self::SummonWither => "summonWither",
			Self::IronPickless => "ironPickless",
			Self::Oneshot => "oneshot",
			Self::Overtake => "overtake",
			Self::Foodless => "foodless",
			Self::ClassicRun => "classicRun",
			Self::Netherite => "netherite",
			Self::Armorless => "armorless",
			Self::HighLevel => "highLevel",
			Self::EgapHolder => "egapHolder",
			Self::IronHoe => "ironHoe",
			Self::SeasonOutcome { season, rank } => {
				return (
					"seasonResult",
					[season.to_string().into(), rank.to_string().into()].into(),
				)
			}
			Self::PlayoffsOutcome { season } => {
				return ("playoffsResult", [season.to_string().into()].into())
			}
			Self::WeeklyRace { count } => return ("weeklyRace", [count.to_string().into()].into()),
			Self::Secret { id, data } => return (id, data.to_owned()),
		};
		(id, [].into())
	}
}

#[derive(Deserialize)]
struct ApiAchievement {
	id: Box<str>,
	#[serde(with = "ts_seconds")]
	date: DateTime<Utc>,
	data: Box<[Box<str>]>,
	level: AchievementLevel,
	goal: Option<AchievementGoal>,
	value: Option<AchievementValue>,
}

/// User's achievement. `data` contains achievement type and associated data
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Achievement {
	/// The date the achievement was acquired
	date: DateTime<Utc>,
	/// The type and data of the achievement
	data: AchievementData,
	/// The level of the achievement
	level: AchievementLevel,
	/// The goal to level up the achievement
	goal: Option<AchievementGoal>,
	/// The value of the achievement
	value: Option<AchievementValue>,
}

impl Achievement {
	pub fn data(&self) -> &AchievementData {
		&self.data
	}
}

impl<'de> Deserialize<'de> for Achievement {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		use AchievementData as AD;

		let ApiAchievement {
			id,
			date,
			data,
			level,
			goal,
			value,
		} = ApiAchievement::deserialize(deserializer)?;

		let data = match id.as_ref() {
			"bestTime" => AD::BestTime,
			"highestWinStreak" => AD::HighestWinStreak,
			"playedMatches" => AD::PlayedMatches,
			"playtime" => AD::Playtime,
			"wins" => AD::Wins,
			"seasonResult" => match data.as_ref() {
				[season_str, rank_str] => match (season_str.parse(), rank_str.parse()) {
					(Ok(season), Ok(rank)) => AchievementData::SeasonOutcome { season, rank },
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
				other_data => return Err(de::Error::invalid_length(other_data.len(), &"2")),
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
				other_data => return Err(de::Error::invalid_length(other_data.len(), &"1")),
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
			level,
			date,
			data,
			goal,
			value,
		})
	}
}

#[cfg(feature = "serialize")]
impl Serialize for Achievement {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		let (id, data) = self.data.id_data();
		let mut map = serializer.serialize_map(Some(6))?;
		map.serialize_entry("id", id)?;
		map.serialize_entry("data", &data)?;
		map.serialize_entry("level", &self.level)?;
		map.serialize_entry("date", &self.date.timestamp())?;
		map.serialize_entry("goal", &self.goal)?;
		map.serialize_entry("value", &self.value)?;
		map.end()
	}
}
