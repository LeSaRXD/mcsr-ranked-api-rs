mod time {
	use crate::types::Time;

	#[test]
	fn time() {
		let time = Time::new(3_923_124);
		assert_eq!(time.millis(), 124);
		assert_eq!(time.seconds(), 23);
		assert_eq!(time.minutes(), 5);
		assert_eq!(time.hours(), 1);
	}
}
