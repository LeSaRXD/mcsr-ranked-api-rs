use super::{Pagination, PaginationError};

#[test]
fn new_ok() {
	assert_eq!(
		Pagination::new(32, 5).unwrap(),
		Pagination { page: 32, count: 5 }
	);
}

#[test]
fn new_page_error() {
	assert_eq!(
		Pagination::new(100, 20).unwrap_err(),
		PaginationError::Page(100)
	);
}

#[test]
fn new_count_error() {
	assert_eq!(
		Pagination::new(32, 51).unwrap_err(),
		PaginationError::Count(51)
	);

	assert_eq!(
		Pagination::new(32, 0).unwrap_err(),
		PaginationError::Count(0)
	);
}
