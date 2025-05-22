use std::str::FromStr;

use mcsr_ranked_api::user::identifier::UserIdentifier;
use uuid::Uuid;

fn main() {
	let user_id = UserIdentifier::Uuid(Uuid::from_str("79635c3dbf634a228bf44544cc7c0d27").unwrap());
	let user_data = user_id.get_user_blocking(None).unwrap();

	let stats = user_data.statistics.season;
	let average = stats.completion_time.ranked.unwrap_or_default()
		/ stats.completions.ranked.unwrap_or(0).max(1)
		/ 1000;

	println!(
		"{}'s average ranked completion time is {} minutes {} seconds",
		user_data.profile.nickname,
		average / 60,
		average % 60
	);
}
