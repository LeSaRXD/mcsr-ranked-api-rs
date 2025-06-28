use crate::variations::{NetherBiome, OverworldBiome, RuinedPortalType, ShipwreckType, ZeroTower};

use super::Variation;

macro_rules! test_variation {
	($string:literal, $repr:expr) => {{
		const JSON: &str = concat!('"', $string, '"');
		let variation: Variation = serde_json::from_str(JSON).unwrap();
		assert_eq!($repr, variation);
		#[cfg(feature = "serialize")]
		assert_eq!(serde_json::to_string(&variation).unwrap(), JSON);
	}};
}
#[test]
fn variations() {
	test_variation!("bastion:triple:1", Variation::BastionTripleChests(1));
	test_variation!("bastion:single:1", Variation::BastionMediumRamParts(1));
	test_variation!("bastion:small_single:2", Variation::BastionSmallRamParts(2));
	test_variation!("bastion:good_gap:0", Variation::StablesGoodGaps(0));
	test_variation!("end_spawn:buried:57", Variation::BuriedEndSpawn(57));
	test_variation!(
		"biome:structure:forest",
		Variation::OverworldStructureBiome(OverworldBiome::Forest)
	);
	test_variation!(
		"biome:bastion:soul_sand_valley",
		Variation::BastionBiome(NetherBiome::SoulSandValley)
	);
	test_variation!(
		"biome:fortress:nether_wastes",
		Variation::FortressBiome(NetherBiome::NetherWastes)
	);
	test_variation!(
		"end_tower:caged:front",
		Variation::CagedEndTower(ZeroTower::Front)
	);
	test_variation!(
		"type:structure:full",
		Variation::Shipwreck(ShipwreckType::Full)
	);
	test_variation!(
		"type:structure:lava",
		Variation::RuinedPortal(RuinedPortalType::Lava)
	);
}
