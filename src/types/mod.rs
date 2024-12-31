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

#[doc(hidden)]
/// Result with this crate's own `Error` type
pub type ReqResult<T> = Result<T, ReqError>;

#[doc(hidden)]
/// Error returned by a request to the API
#[derive(Debug)]
pub enum ReqError {
	/// Ranked API error
	Api(Option<Box<str>>),
	/// Reqwest library error
	Reqwest(reqwest::Error),
}

impl PartialEq for ReqError {
	fn eq(&self, other: &Self) -> bool {
		use ReqError::*;

		match (self, other) {
			(Api(lhs), Api(rhs)) => lhs == rhs,
			(Reqwest(lhs), Reqwest(rhs)) => lhs.to_string() == rhs.to_string(),
			_ => false,
		}
	}
}
impl Eq for ReqError {}

impl From<reqwest::Error> for ReqError {
	fn from(value: reqwest::Error) -> Self {
		Self::Reqwest(value)
	}
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum DeReqResult<T> {
	Ok(T),
	Err(ReqError),
}

impl<'de, T> Deserialize<'de> for DeReqResult<T>
where
	T: Deserialize<'de>,
{
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		enum Field {
			Status,
			Data,
		}

		impl<'de> Deserialize<'de> for Field {
			fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
			where
				D: Deserializer<'de>,
			{
				struct FieldVisitor;

				impl<'de> Visitor<'de> for FieldVisitor {
					type Value = Field;

					fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
						formatter.write_str("`status` or `data`")
					}

					fn visit_str<E>(self, value: &str) -> Result<Field, E>
					where
						E: de::Error,
					{
						match value {
							"status" => Ok(Field::Status),
							"data" => Ok(Field::Data),
							_ => Err(de::Error::unknown_field(value, FIELDS)),
						}
					}
				}

				deserializer.deserialize_identifier(FieldVisitor)
			}
		}

		struct ApiResultVisitor<T> {
			_type: PhantomData<T>,
		}

		impl<'de, T> Visitor<'de> for ApiResultVisitor<T>
		where
			T: Deserialize<'de>,
		{
			type Value = DeReqResult<T>;

			fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
				formatter.write_str("struct ApiResult")
			}

			fn visit_map<V>(self, mut map: V) -> Result<Self::Value, V::Error>
			where
				V: MapAccess<'de>,
			{
				use Field::*;

				#[derive(Deserialize)]
				#[serde(rename_all = "lowercase")]
				enum ApiStatus {
					Success,
					Error,
				}

				let mut status: Option<ApiStatus> = None;
				let mut result: Option<DeReqResult<T>> = None;

				while let Some(key) = map.next_key()? {
					match key {
						Status => {
							if status.is_some() {
								return Err(de::Error::duplicate_field("status"));
							}
							status = Some(map.next_value()?);
						}
						Data => {
							if result.is_some() {
								return Err(de::Error::duplicate_field("data"));
							}
							match status {
								None => return Err(de::Error::missing_field("status")),
								Some(ApiStatus::Error) => {
									let error_message: Option<Box<str>> = map.next_value()?;
									result = Some(DeReqResult::Err(ReqError::Api(error_message)));
								}
								Some(ApiStatus::Success) => {
									let data: T = map.next_value()?;
									result = Some(DeReqResult::Ok(data));
								}
							}
						}
					}
				}

				result.ok_or_else(|| de::Error::missing_field("data"))
			}
		}

		const FIELDS: &[&str] = &["status", "data"];
		deserializer.deserialize_struct(
			"ApiResult",
			FIELDS,
			ApiResultVisitor::<T> { _type: PhantomData },
		)
	}
}

impl<T> From<DeReqResult<T>> for ReqResult<T> {
	fn from(value: DeReqResult<T>) -> Self {
		match value {
			DeReqResult::Ok(t) => Ok(t),
			DeReqResult::Err(e) => Err(e),
		}
	}
}
