use serde::{de::DeserializeOwned, Serialize};

use crate::{types::DeResult, Result};

#[cfg(test)]
mod tests;

#[cfg(all(feature = "serialize", feature = "variations"))]
pub(crate) mod ser_str;

pub(crate) mod string_u64 {
	use serde::Deserialize;
	#[cfg(feature = "serialize")]
	use serde::{Serialize, Serializer};

	#[cfg(feature = "serialize")]
	pub(crate) fn serialize<S>(value: &u64, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		value.to_string().serialize(serializer)
	}
	pub(crate) fn deserialize<'de, D>(deserializer: D) -> Result<u64, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		let value = String::deserialize(deserializer)?;
		value.parse().map_err(|_| {
			serde::de::Error::invalid_type(serde::de::Unexpected::Str(&value), &"a u64")
		})
	}
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
) -> Result<T>
where
	T: DeserializeOwned,
	V: IntoIterator<Item = &'v S>,
	S: AsRef<str> + 'v,
{
	let url = construct_url(base_url, variables, params);
	reqwest::get(url.as_ref())
		.await?
		.json::<DeResult<T>>()
		.await?
		.into()
}

#[cfg(feature = "blocking")]
pub(crate) fn make_request_blocking<'v, T, V, S>(
	base_url: &str,
	variables: V,
	params: Option<&impl Serialize>,
) -> Result<T>
where
	T: DeserializeOwned,
	V: IntoIterator<Item = &'v S>,
	S: AsRef<str> + 'v,
{
	let url = construct_url(base_url, variables, params);
	reqwest::blocking::get(url.as_ref())?
		.json::<DeResult<T>>()?
		.into()
}
