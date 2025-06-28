use std::{error::Error, fmt::Display};

use serde::{
	ser::{
		SerializeMap, SerializeSeq, SerializeStruct, SerializeStructVariant, SerializeTuple,
		SerializeTupleStruct, SerializeTupleVariant,
	},
	Serializer,
};

#[derive(Debug, Clone)]
pub(crate) struct UnsupportedType(String);
impl Display for UnsupportedType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "Unsupported type: {}", self.0)
	}
}
impl Error for UnsupportedType {}
impl serde::ser::Error for UnsupportedType {
	fn custom<T>(msg: T) -> Self
	where
		T: Display,
	{
		Self(msg.to_string())
	}
}

pub(crate) struct StrSerialize(String);
impl SerializeStructVariant for StrSerialize {
	type Ok = String;
	type Error = UnsupportedType;

	fn serialize_field<T>(&mut self, _key: &'static str, _value: &T) -> Result<(), Self::Error>
	where
		T: ?Sized + serde::Serialize,
	{
		Ok(())
	}
	fn skip_field(&mut self, _key: &'static str) -> Result<(), Self::Error> {
		Ok(())
	}
	fn end(self) -> Result<Self::Ok, Self::Error> {
		Ok(self.0)
	}
}
impl SerializeTupleStruct for StrSerialize {
	type Ok = String;
	type Error = UnsupportedType;

	fn serialize_field<T>(&mut self, _value: &T) -> Result<(), Self::Error>
	where
		T: ?Sized + serde::Serialize,
	{
		Ok(())
	}
	fn end(self) -> Result<Self::Ok, Self::Error> {
		Ok(self.0)
	}
}
impl SerializeSeq for StrSerialize {
	type Ok = String;
	type Error = UnsupportedType;

	fn serialize_element<T>(&mut self, _value: &T) -> Result<(), Self::Error>
	where
		T: ?Sized + serde::Serialize,
	{
		Ok(())
	}
	fn end(self) -> Result<Self::Ok, Self::Error> {
		Ok(self.0)
	}
}
impl SerializeTuple for StrSerialize {
	type Ok = String;
	type Error = UnsupportedType;
	fn serialize_element<T>(&mut self, _value: &T) -> Result<(), Self::Error>
	where
		T: ?Sized + serde::Serialize,
	{
		Ok(())
	}
	fn end(self) -> Result<Self::Ok, Self::Error> {
		Ok(self.0)
	}
}
impl SerializeStruct for StrSerialize {
	type Ok = String;
	type Error = UnsupportedType;
	fn skip_field(&mut self, _key: &'static str) -> Result<(), Self::Error> {
		Ok(())
	}
	fn serialize_field<T>(&mut self, _key: &'static str, _value: &T) -> Result<(), Self::Error>
	where
		T: ?Sized + serde::Serialize,
	{
		Ok(())
	}
	fn end(self) -> Result<Self::Ok, Self::Error> {
		Ok(self.0)
	}
}
impl SerializeMap for StrSerialize {
	type Ok = String;
	type Error = UnsupportedType;
	fn serialize_entry<K, V>(&mut self, _key: &K, _value: &V) -> Result<(), Self::Error>
	where
		K: ?Sized + serde::Serialize,
		V: ?Sized + serde::Serialize,
	{
		Ok(())
	}
	fn serialize_value<T>(&mut self, _value: &T) -> Result<(), Self::Error>
	where
		T: ?Sized + serde::Serialize,
	{
		Ok(())
	}
	fn serialize_key<T>(&mut self, _key: &T) -> Result<(), Self::Error>
	where
		T: ?Sized + serde::Serialize,
	{
		Ok(())
	}
	fn end(self) -> Result<Self::Ok, Self::Error> {
		Ok(self.0)
	}
}
impl SerializeTupleVariant for StrSerialize {
	type Ok = String;
	type Error = UnsupportedType;
	fn serialize_field<T>(&mut self, _value: &T) -> Result<(), Self::Error>
	where
		T: ?Sized + serde::Serialize,
	{
		Ok(())
	}
	fn end(self) -> Result<Self::Ok, Self::Error> {
		Ok(self.0)
	}
}

pub(crate) struct StrSerializer;
impl Serializer for StrSerializer {
	type Ok = String;
	type Error = UnsupportedType;
	type SerializeStructVariant = StrSerialize;
	type SerializeTupleStruct = StrSerialize;
	type SerializeSeq = StrSerialize;
	type SerializeTuple = StrSerialize;
	type SerializeStruct = StrSerialize;
	type SerializeMap = StrSerialize;
	type SerializeTupleVariant = StrSerialize;

	fn is_human_readable(&self) -> bool {
		true
	}
	fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
		Ok(v.to_string())
	}
	fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
		Ok(v.to_string())
	}
	fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
		Ok(v.to_string())
	}
	fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
		Ok(v.to_string())
	}
	fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
		Ok(v.to_string())
	}
	fn serialize_i128(self, v: i128) -> Result<Self::Ok, Self::Error> {
		Ok(v.to_string())
	}
	fn collect_str<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
	where
		T: ?Sized + Display,
	{
		Ok(value.to_string())
	}
	fn collect_map<K, V, I>(self, _iter: I) -> Result<Self::Ok, Self::Error>
	where
		K: serde::Serialize,
		V: serde::Serialize,
		I: IntoIterator<Item = (K, V)>,
	{
		Err(UnsupportedType("map".to_string()))
	}
	fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
		Ok(v.to_string())
	}
	fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
		Ok(v.to_string())
	}
	fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
		Ok(v.to_string())
	}
	fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
		Ok(v.to_string())
	}
	fn serialize_u128(self, v: u128) -> Result<Self::Ok, Self::Error> {
		Ok(v.to_string())
	}
	fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
		Ok(v.to_string())
	}
	fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
		Ok(v.to_string())
	}
	fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
		Ok(v.to_string())
	}
	fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
		Ok(v.to_string())
	}
	fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
		String::from_utf8(v.to_vec()).map_err(|_| UnsupportedType("bytes".to_string()))
	}
	fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
		Ok("".to_string())
	}
	fn serialize_some<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
	where
		T: ?Sized + serde::Serialize,
	{
		value.serialize(Self)
	}
	fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
		Ok("".to_string())
	}
	fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
		Ok(name.to_string())
	}
	fn serialize_unit_variant(
		self,
		_name: &'static str,
		_variant_index: u32,
		variant: &'static str,
	) -> Result<Self::Ok, Self::Error> {
		Ok(variant.to_string())
	}
	fn serialize_newtype_struct<T>(
		self,
		name: &'static str,
		_value: &T,
	) -> Result<Self::Ok, Self::Error>
	where
		T: ?Sized + serde::Serialize,
	{
		Ok(name.to_string())
	}
	fn serialize_newtype_variant<T>(
		self,
		_name: &'static str,
		_variant_index: u32,
		variant: &'static str,
		_value: &T,
	) -> Result<Self::Ok, Self::Error>
	where
		T: ?Sized + serde::Serialize,
	{
		Ok(variant.to_string())
	}
	fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
		Err(UnsupportedType("sequence".to_string()))
	}
	fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
		Err(UnsupportedType("tuple".to_string()))
	}
	fn serialize_tuple_struct(
		self,
		name: &'static str,
		_len: usize,
	) -> Result<Self::SerializeTupleStruct, Self::Error> {
		Ok(StrSerialize(name.to_string()))
	}
	fn serialize_tuple_variant(
		self,
		_name: &'static str,
		_variant_index: u32,
		variant: &'static str,
		_len: usize,
	) -> Result<Self::SerializeTupleVariant, Self::Error> {
		Ok(StrSerialize(variant.to_string()))
	}
	fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
		Err(UnsupportedType("map".to_string()))
	}
	fn serialize_struct(
		self,
		name: &'static str,
		_len: usize,
	) -> Result<Self::SerializeStruct, Self::Error> {
		Ok(StrSerialize(name.to_string()))
	}
	fn serialize_struct_variant(
		self,
		_name: &'static str,
		_variant_index: u32,
		variant: &'static str,
		_len: usize,
	) -> Result<Self::SerializeStructVariant, Self::Error> {
		Ok(StrSerialize(variant.to_string()))
	}
	fn collect_seq<I>(self, _iter: I) -> Result<Self::Ok, Self::Error>
	where
		I: IntoIterator,
		<I as IntoIterator>::Item: serde::Serialize,
	{
		Err(UnsupportedType("sequence".to_string()))
	}
}
