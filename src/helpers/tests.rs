use serde::Serialize;

use crate::helpers::construct_url;

#[test]
fn construct() {
	#[derive(Serialize)]
	struct TestQueryParams {
		age: u8,
		name: String,
	}

	const BASE_URL: &str = "http://example.com/user/{}";
	let url = construct_url(
		BASE_URL,
		[&"test_user"],
		&Some(TestQueryParams {
			age: 19,
			name: "laysar".to_owned(),
		}),
	);
	assert_eq!(
		url.as_ref(),
		r#"http://example.com/user/test_user?age=19&name=laysar"#
	);
}
