#[cfg(feature = "serialize")]
use serde::{ser, Serialize};
#[cfg(test)]
mod tests;

use serde::{
	de::{self, IntoDeserializer},
	Deserialize,
};

use crate::helpers::ser_str::StrSerializer;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Variation {
	OverworldChestLoot(ChestLoot),
	BastionChestLoot(ChestLoot),
	Shipwreck(ShipwreckType),
	RuinedPortal(RuinedPortalType),
	OverworldStructureBiome(OverworldBiome),
	BastionBiome(NetherBiome),
	FortressBiome(NetherBiome),
	StablesGoodGaps(u8),
	BastionSmallRamParts(u8),
	BastionMediumRamParts(u8),
	BastionTripleChests(u8),
	BuriedEndSpawn(u8),
	CagedEndTower(ZeroTower),
	Other(Box<[Box<str>]>),
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChestLoot {
	Diamond,
	Obsidian,
	LootingSword,
	GoldenApple,
	#[serde(rename = "egap")]
	EnchantedGoldenApple,
	Carrot,
}

/// Shipwreck types
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ShipwreckType {
	/// Full shipwreck with masts
	Full,
	/// Right side up shipwreck without masts
	Normal,
	/// Upside down shipwreck
	UpsideDown,
	/// Sideways shipwreck
	Sideways,
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RuinedPortalType {
	/// Completable ruined portals have enough obsidian in the chest to finish the frame
	Completable,
	/// Lava ruined portals require using the lava around the ruined portal to build the frame
	Lava,
}

/// Overworld biomes taken from https://minecraft.wiki/w/Biome#Biome_IDs
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OverworldBiome {
	Badlands,
	BambooJungle,
	Beach,
	BirchForest,
	ColdOcean,
	DarkForest,
	DeepColdOcean,
	DeepFrozenOcean,
	DeepLukewarmOcean,
	DeepOcean,
	Desert,
	ErodedBadlands,
	FlowerForest,
	Forest,
	FrozenOcean,
	FrozenRiver,
	IceSpikes,
	Jungle,
	LukewarmOcean,
	MushroomFields,
	Ocean,
	Plains,
	River,
	Savanna,
	SavannaPlateau,
	SnowyBeach,
	SnowyTaiga,
	/// Snowy tundras were removed after version 1.16, which is why it is missing from the wiki page
	SnowyTundra,
	SunflowerPlains,
	Swamp,
	Taiga,
	WarmOcean,
}

/// Nether biomes taken from https://minecraft.wiki/w/Biome#Biome_IDs
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NetherBiome {
	BasaltDeltas,
	CrimsonForest,
	NetherWastes,
	SoulSandValley,
	WarpedForest,
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
#[repr(u8)]
pub enum ZeroTower {
	Front = 0,
	FrontCenter = 1,
	Back = 2,
	BackCenter = 3,
}

impl<'de> Deserialize<'de> for Variation {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		let id = String::deserialize(deserializer)?;
		let parts = id.split(':').collect::<Vec<_>>();

		macro_rules! de {
			($val:expr) => {
				Deserialize::deserialize($val.into_deserializer())?
			};
		}
		macro_rules! de_u8 {
			($val:expr) => {
				$val.parse::<u8>()
					.map_err(|_| de::Error::invalid_type(de::Unexpected::Str($val), &"u8"))?
			};
		}
		Ok(match *parts.as_slice() {
			["chest", "structure", item_name] => Self::OverworldChestLoot(de!(item_name)),
			["chest", "bastion", item_name] => Self::BastionChestLoot(de!(item_name)),
			["type", "structure", item_name] => {
				let de = item_name.into_deserializer();
				if let Ok(rp_type) = RuinedPortalType::deserialize(de) {
					Self::RuinedPortal(rp_type)
				} else {
					Self::Shipwreck(ShipwreckType::deserialize(de)?)
				}
			}
			["biome", "structure", biome_name] => Self::OverworldStructureBiome(de!(biome_name)),
			["biome", "bastion", biome_name] => Self::BastionBiome(de!(biome_name)),
			["biome", "fortress", biome_name] => Self::FortressBiome(de!(biome_name)),
			["bastion", "good_gap", count] => Self::StablesGoodGaps(de_u8!(count)),
			["bastion", "triple", count] => Self::BastionTripleChests(de_u8!(count)),
			["bastion", "single", count] => Self::BastionMediumRamParts(de_u8!(count)),
			["bastion", "small_single", count] => Self::BastionSmallRamParts(de_u8!(count)),
			["end_spawn", "buried", y_level] => Self::BuriedEndSpawn(de_u8!(y_level)),
			["end_tower", "caged", tower] => Self::CagedEndTower(de!(tower)),
			_ => Self::Other(parts.into_iter().map(Into::into).collect()),
		})
	}
}

#[cfg(feature = "serialize")]
impl Serialize for Variation {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		macro_rules! ser {
			($pat:literal, $val:expr) => {{
				let s = $val
					.serialize(StrSerializer)
					.map_err(|e| ser::Error::custom(e.to_string()))?;
				format!($pat, s)
			}};
		}

		let s = match self {
			Self::OverworldChestLoot(item_name) => ser!("chest:structure:{}", item_name),
			Self::BastionChestLoot(item_name) => ser!("chest:bastion:{}", item_name),
			Self::RuinedPortal(rp_type) => ser!("type:structure:{}", rp_type),
			Self::Shipwreck(ship_type) => ser!("type:structure:{}", ship_type),
			Self::OverworldStructureBiome(biome_name) => ser!("biome:structure:{}", biome_name),
			Self::BastionBiome(biome_name) => ser!("biome:bastion:{}", biome_name),
			Self::FortressBiome(biome_name) => ser!("biome:fortress:{}", biome_name),
			Self::StablesGoodGaps(count) => ser!("bastion:good_gap:{}", count),
			Self::BastionTripleChests(count) => ser!("bastion:triple:{}", count),
			Self::BastionMediumRamParts(count) => ser!("bastion:single:{}", count),
			Self::BastionSmallRamParts(count) => ser!("bastion:small_single:{}", count),
			Self::BuriedEndSpawn(y_level) => ser!("end_spawn:buried:{}", y_level),
			Self::CagedEndTower(tower) => ser!("end_tower:caged:{}", tower),
			Self::Other(parts) => parts.join(":"),
		};
		serializer.serialize_newtype_struct("Variation", &s)
	}
}
