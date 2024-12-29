use serde_repr::Deserialize_repr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Deserialize_repr)]
#[repr(u8)]
pub enum SupporterTier {
	None = 0,
	Stone = 1,
	Iron = 2,
	Diamond = 3,
}
