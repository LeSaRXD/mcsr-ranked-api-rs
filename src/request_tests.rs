#[test]
#[ignore]
fn user_redlime() {
	use std::str::FromStr;

	use uuid::Uuid;

	use crate::user::identifier::UserIdentifier;

	let redlime_id = UserIdentifier::Nickname("RED_LIME");
	let request = redlime_id.get_user_blocking(None);
	let redlime = request.unwrap();
	assert_eq!(
		redlime.profile().uuid(),
		Uuid::from_str("bbc886da1b024739b4b80f1542e9f61d").unwrap(),
		"Check your internet connection"
	);
}
