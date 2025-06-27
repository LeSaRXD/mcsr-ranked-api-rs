use crate::pagination::Pagination;

#[test]
#[ignore]
fn user_redlime() {
	use uuid::uuid;

	use crate::user::identifier::UserIdentifier;

	let redlime_id = UserIdentifier::Nickname("RED_LIME");
	let request = redlime_id.get_user_blocking(None);
	let redlime = request.unwrap();
	assert_eq!(
		redlime.profile.uuid,
		uuid!("bbc886da1b024739b4b80f1542e9f61d"),
		"Check your internet connection"
	);
}

#[test]
#[ignore]
fn user_matches() {
	use uuid::uuid;

	use crate::user::identifier::UserIdentifier;

	let doogile_id = UserIdentifier::Uuid(uuid!("3c8757790ab0400b8b9e3936e0dd535b"));
	let params = Pagination::count(100).unwrap().into();
	let request = doogile_id.get_matches_blocking(Some(&params));
	println!("{request:?}");
	assert!(request.is_ok());
}
