use composable_utils::ResultOptionExt;

enum ErrorOne {
	One,
}

enum ErrorTwo {
	Two,
	Three,
}

fn result_ok_some() -> Result<Option<&'static str>, ErrorOne> {
	Ok(Some("trans rights"))
}

fn result_ok_none() -> Result<Option<&'static str>, ErrorOne> {
	Ok(None)
}

fn result_err() -> Result<Option<&'static str>, ErrorOne> {
	Err(ErrorOne::One)
}

fn option_some_ok() -> Option<Result<&'static str, ErrorOne>> {
	Some(Ok("trans rights"))
}

fn option_some_err() -> Option<Result<&'static str, ErrorOne>> {
	Some(Err(ErrorOne::One))
}

fn option_none() -> Option<Result<&'static str, ErrorOne>> {
	None
}

#[test]
fn unwrap_or_err() {
	assert!(result_ok_some().unwrap_or_err(ErrorTwo::Two).is_ok());
	assert!(result_ok_none().unwrap_or_err(ErrorTwo::Two).is_err());
	assert!(result_err().unwrap_or_err(ErrorTwo::Two).is_err());

	assert!(option_some_ok().unwrap_or_err(ErrorTwo::Two).is_ok());
	assert!(option_some_err().unwrap_or_err(ErrorTwo::Two).is_err());
	assert!(option_none().unwrap_or_err(ErrorTwo::Two).is_err());
}

#[test]
fn unwrap_or_else_err() {
	assert!(result_ok_some().unwrap_or_else_err(|| ErrorTwo::Two).is_ok());
	assert!(result_ok_none().unwrap_or_else_err(|| ErrorTwo::Two).is_err());
	assert!(result_err().unwrap_or_else_err(|| ErrorTwo::Two).is_err());

	assert!(option_some_ok().unwrap_or_else_err(|| ErrorTwo::Two).is_ok());
	assert!(option_some_err().unwrap_or_else_err(|| ErrorTwo::Two).is_err());
	assert!(option_none().unwrap_or_else_err(|| ErrorTwo::Two).is_err());
}

#[test]
fn unwrap_or_map_err() {
	assert!(result_ok_some()
		.unwrap_or_map_err(ErrorTwo::Two, |_| ErrorTwo::Three)
		.is_ok());
	assert!(matches!(
		result_ok_none().unwrap_or_map_err(ErrorTwo::Two, |_| ErrorTwo::Three),
		Err(ErrorTwo::Two)
	));
	assert!(matches!(
		result_err().unwrap_or_map_err(ErrorTwo::Two, |_| ErrorTwo::Three),
		Err(ErrorTwo::Three)
	));

	assert!(option_some_ok()
		.unwrap_or_map_err(ErrorTwo::Two, |_| ErrorTwo::Three)
		.is_ok());
	assert!(matches!(
		option_some_err().unwrap_or_map_err(ErrorTwo::Two, |_| ErrorTwo::Three),
		Err(ErrorTwo::Three)
	));
	assert!(matches!(
		option_none().unwrap_or_map_err(ErrorTwo::Two, |_| ErrorTwo::Three),
		Err(ErrorTwo::Two)
	));
}

#[test]
fn unwrap_or_else_map_err() {
	assert!(result_ok_some()
		.unwrap_or_else_map_err(|| ErrorTwo::Two, |_| ErrorTwo::Three)
		.is_ok());
	assert!(matches!(
		result_ok_none().unwrap_or_else_map_err(|| ErrorTwo::Two, |_| ErrorTwo::Three),
		Err(ErrorTwo::Two)
	));
	assert!(matches!(
		result_err().unwrap_or_else_map_err(|| ErrorTwo::Two, |_| ErrorTwo::Three),
		Err(ErrorTwo::Three)
	));

	assert!(option_some_ok()
		.unwrap_or_else_map_err(|| ErrorTwo::Two, |_| ErrorTwo::Three)
		.is_ok());
	assert!(matches!(
		option_some_err().unwrap_or_else_map_err(|| ErrorTwo::Two, |_| ErrorTwo::Three),
		Err(ErrorTwo::Three)
	));
	assert!(matches!(
		option_none().unwrap_or_else_map_err(|| ErrorTwo::Two, |_| ErrorTwo::Three),
		Err(ErrorTwo::Two)
	));
}
