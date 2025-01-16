use chrono::{naive::serde::ts_seconds, DateTime, Utc};
use serde::{
	de::{DeserializeOwned, Error, MapAccess},
	Deserialize, Deserializer, Serialize,
};

use crate::types::DeReqResult;

#[cfg(test)]
mod tests;

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

pub(crate) fn string_u64<'de, T, D>(de: D) -> Result<T, D::Error>
where
	D: serde::Deserializer<'de>,
	T: std::str::FromStr,
	<T as std::str::FromStr>::Err: std::fmt::Display,
{
	let value = String::deserialize(de)?;
	value
		.parse()
		.map_err(|_| serde::de::Error::invalid_type(serde::de::Unexpected::Str(&value), &"a u64"))
}

fn construct_url<'v, V, S>(
	base: impl ToString,
	variables: V,
	params: Option<&impl Serialize>,
) -> Box<str>
where
	V: IntoIterator<Item = &'v S>,
	S: AsRef<str> + 'v,
{
	let mut url = base.to_string();
	for elem in variables {
		url = url.replacen("{}", elem.as_ref(), 1);
	}

	if let Some(params) = params {
		let params_str = serde_qs::to_string(params).expect("Expected valid params");
		url.reserve_exact(params_str.len() + 1);
		url.push('?');
		url.push_str(&params_str);
	}
	url.into_boxed_str()
}

pub(crate) async fn make_request<'v, T, V, S>(
	base_url: &str,
	variables: V,
	params: Option<&impl Serialize>,
) -> crate::Result<T>
where
	T: DeserializeOwned,
	V: IntoIterator<Item = &'v S>,
	S: AsRef<str> + 'v,
{
	let url = construct_url(base_url, variables, params);
	reqwest::get(url.as_ref())
		.await?
		.json::<DeReqResult<T>>()
		.await?
		.into()
}

#[cfg(feature = "blocking")]
pub(crate) fn make_request_blocking<'v, T, V, S>(
	base_url: &str,
	variables: V,
	params: Option<&impl Serialize>,
) -> crate::Result<T>
where
	T: DeserializeOwned,
	V: IntoIterator<Item = &'v S>,
	S: AsRef<str> + 'v,
{
	let url = construct_url(base_url, variables, params);
	reqwest::blocking::get(url.as_ref())?
		.json::<DeReqResult<T>>()?
		.into()
}
