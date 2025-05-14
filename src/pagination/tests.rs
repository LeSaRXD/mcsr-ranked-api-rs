use super::*;

#[test]
fn before() {
	let before = RelativePos::Before(999);
	let before_str = serde_qs::to_string(&before).unwrap();
	assert_eq!(before_str, "before=999");

	let pagination = Pagination {
		count: NonZeroU8::new(10).unwrap(),
		position: Some(before),
	};
	let pagination_str = serde_qs::to_string(&pagination).unwrap();
	assert_eq!(pagination_str, "count=10&before=999");
}

#[test]
fn after() {
	let after = RelativePos::After(1999);
	let after_str = serde_qs::to_string(&after).unwrap();
	assert_eq!(after_str, "after=1999");

	let pagination = Pagination {
		count: NonZeroU8::new(10).unwrap(),
		position: Some(after),
	};
	let pagination_str = serde_qs::to_string(&pagination).unwrap();
	assert_eq!(pagination_str, "count=10&after=1999");
}
