use chrono::{naive::serde::ts_seconds, DateTime, Utc};
use serde::{
	de::{Error, MapAccess},
	Deserialize, Deserializer,
};

struct DeserializeUnixTimestamp(pub(crate) DateTime<Utc>);
impl<'de> Deserialize<'de> for DeserializeUnixTimestamp {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		Ok(DeserializeUnixTimestamp(
			ts_seconds::deserialize(deserializer)?.and_utc(),
		))
	}
}

pub(crate) trait NextUnixTimestamp<'de> {
	type Error: Error;
	fn next_unix_timestamp(&mut self) -> Result<DateTime<Utc>, Self::Error>;
}

impl<'de, E: Error, M: MapAccess<'de, Error = E>> NextUnixTimestamp<'de> for M {
	type Error = E;
	fn next_unix_timestamp(&mut self) -> Result<DateTime<Utc>, E> {
		Ok(self.next_value::<DeserializeUnixTimestamp>()?.0)
	}
}