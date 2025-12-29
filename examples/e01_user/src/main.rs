use mcsr_ranked_api::user::identifier::UserIdentifier;
use uuid::uuid;

fn main() {
	let user_id = UserIdentifier::Uuid(uuid!("79635c3dbf634a228bf44544cc7c0d27"));
	let user_data = user_id.get_user_blocking(None).unwrap();

	let stats = user_data.statistics.season;
	let average = stats.completion_time.ranked / stats.completions.ranked.max(1) / 1000;

	println!(
		"{}'s average ranked completion time is {} minutes {} seconds",
		user_data.profile.nickname,
		average / 60,
		average % 60
	);
}
