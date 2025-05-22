use chrono::{serde::ts_seconds, DateTime, Utc};
use serde::{de, Deserialize, Deserializer};

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
pub type AchievementValue = u64;

#[derive(Deserialize)]
pub struct ApiAchievement {
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
	date: DateTime<Utc>,
	data: AchievementData,
	level: AchievementLevel,
	goal: Option<AchievementGoal>,
	value: Option<AchievementValue>,
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
