#![forbid(unsafe_code)]
#![warn(clippy::pedantic)]

use std::future::Future;

#[allow(async_fn_in_trait)]
pub trait AsyncOptionExt<T> {
	/// Maps an `Option<T>` to `Option<U>` by applying a function to a contained value (if `Some`) or returns `None` (if `None`).
	///
	/// # Example
	///
	/// ```rust
	/// use composable_utils::AsyncOptionExt;
	///
	/// async fn double(x: usize) -> usize {
	///     x * 2
	/// }
	///
	/// async_io::block_on(async {
	///     let value = Some(69);
	///     let value = value
	///         .async_map(|v| async move { double(v).await })
	///         .await
	///         .unwrap_or_else(|| panic!("value should always be Some"));
	///     assert_eq!(value, 138);
	/// });
	/// ```
	async fn async_map<U, Fut: Future<Output = U>, F: FnOnce(T) -> Fut>(self, f: F) -> Option<U>;
}

pub trait ResultOptionExt<T, E> {
	/// Maps either a `Result<Option<T>, E>` or `Option<Result<T, E>>` to a `Result<T, E2>` where `E2` is the type of the provided `err`.
	///
	/// If it's a Result:
	///    - Ok(Some(t)) -> Ok(t)
	///    - Ok(None) -> Err(err)
	///    - Err(_) -> Err(err)
	///
	/// If it's an Option:
	///    - Some(t) -> Ok(t)
	///    - None -> Err(err)
	///
	/// # Example
	///
	/// ```rust
	///	use composable_utils::ResultOptionExt;
	///
	/// enum ErrorOne {
	///	   One,
	/// }
	///
	/// enum ErrorTwo {
	///	   Two,
	/// }
	///
	/// fn result_ok_none() -> Result<Option<&'static str>, ErrorOne> {
	///    Ok(None)
	/// }
	///
	/// fn option_some_err() -> Option<Result<&'static str, ErrorOne>> {
	///    Some(Err(ErrorOne::One))
	/// }
	///
	/// assert!(matches!(result_ok_none().unwrap_or_err(ErrorTwo::Two), Err(ErrorTwo::Two)));
	///	assert!(matches!(option_some_err().unwrap_or_err(ErrorTwo::Two), Err(ErrorTwo::Two)));
	/// ```
	fn unwrap_or_err<E2>(self, err: E2) -> Result<T, E2>;

	/// Maps either a `Result<Option<T>, E>` or `Option<Result<T, E>>` to a `Result<T, E2>` where `E2` is the type of the result of the provided closure.
	///
	/// If it's a Result:
	///    - Ok(Some(t)) -> Ok(t)
	///    - Ok(None) -> Err(f())
	///    - Err(_) -> Err(f())
	///
	/// If it's an Option:
	///    - Some(t) -> Ok(t)
	///    - None -> Err(f())
	///
	/// # Example
	///
	/// ```rust
	///	use composable_utils::ResultOptionExt;
	///
	/// enum ErrorOne {
	///	   One,
	/// }
	///
	/// enum ErrorTwo {
	///	   Two,
	/// }
	///
	/// fn result_ok_none() -> Result<Option<&'static str>, ErrorOne> {
	///    Ok(None)
	/// }
	///
	/// fn option_some_err() -> Option<Result<&'static str, ErrorOne>> {
	///    Some(Err(ErrorOne::One))
	/// }
	///
	/// assert!(matches!(result_ok_none().unwrap_or_else_err(|| ErrorTwo::Two), Err(ErrorTwo::Two)));
	///	assert!(matches!(option_some_err().unwrap_or_else_err(|| ErrorTwo::Two), Err(ErrorTwo::Two)));
	/// ```
	fn unwrap_or_else_err<E2, F: FnOnce() -> E2>(self, f: F) -> Result<T, E2>;

	/// Maps either a `Result<Option<T>, E>` or `Option<Result<T, E>>` to a `Result<T, E2>` by applying a function to a contained Err value, leaving an Ok value untouched.
	/// Defaults to `Err(default)` if `None`.
	///
	/// # Example
	///
	/// ```rust
	///	use composable_utils::ResultOptionExt;
	///
	/// enum ErrorOne {
	///	   One,
	/// }
	///
	/// enum ErrorTwo {
	///	   Two,
	///    Three,
	/// }
	///
	/// fn result_ok_none() -> Result<Option<&'static str>, ErrorOne> {
	///    Ok(None)
	/// }
	///
	/// fn option_some_err() -> Option<Result<&'static str, ErrorOne>> {
	///    Some(Err(ErrorOne::One))
	/// }
	///
	/// assert!(matches!(result_ok_none().unwrap_or_map_err(ErrorTwo::Two, |err| ErrorTwo::Three), Err(ErrorTwo::Two)));
	///	assert!(matches!(option_some_err().unwrap_or_map_err(ErrorTwo::Two, |err| ErrorTwo::Three), Err(ErrorTwo::Three)));
	/// ```
	fn unwrap_or_map_err<E2, F: FnOnce(E) -> E2>(self, default: E2, f: F) -> Result<T, E2>;

	/// Maps either a `Result<Option<T>, E>` or `Option<Result<T, E>>` to a `Result<T, E2>` by applying a function to a contained Err value, leaving an Ok value untouched.
	/// Defaults to `Err(default)` if `None`.
	///
	/// # Example
	///
	/// ```rust
	///	use composable_utils::ResultOptionExt;
	///
	/// enum ErrorOne {
	///	   One,
	/// }
	///
	/// enum ErrorTwo {
	///	   Two,
	///    Three,
	/// }
	///
	/// fn result_ok_none() -> Result<Option<&'static str>, ErrorOne> {
	///    Ok(None)
	/// }
	///
	/// fn option_some_err() -> Option<Result<&'static str, ErrorOne>> {
	///    Some(Err(ErrorOne::One))
	/// }
	///
	/// assert!(matches!(result_ok_none().unwrap_or_else_map_err(|| ErrorTwo::Two, |err| ErrorTwo::Three), Err(ErrorTwo::Two)));
	///	assert!(matches!(option_some_err().unwrap_or_else_map_err(|| ErrorTwo::Two, |err| ErrorTwo::Three), Err(ErrorTwo::Three)));
	/// ```
	fn unwrap_or_else_map_err<E2, F: FnOnce(E) -> E2, F2: FnOnce() -> E2>(self, default: F2, f: F) -> Result<T, E2>;
}

impl<T> AsyncOptionExt<T> for Option<T> {
	async fn async_map<U, Fut: Future<Output = U>, F: FnOnce(T) -> Fut>(self, f: F) -> Option<U> {
		match self {
			Some(t) => Some(f(t).await),
			None => None,
		}
	}
}

impl<T, E> ResultOptionExt<T, E> for Option<Result<T, E>> {
	fn unwrap_or_err<E2>(self, err: E2) -> Result<T, E2> {
		match self {
			Some(Ok(t)) => Ok(t),
			Some(Err(_)) => Err(err),
			None => Err(err),
		}
	}

	fn unwrap_or_else_err<E2, F: FnOnce() -> E2>(self, f: F) -> Result<T, E2> {
		match self {
			Some(Ok(t)) => Ok(t),
			Some(Err(_)) => Err(f()),
			None => Err(f()),
		}
	}

	fn unwrap_or_map_err<E2, F: FnOnce(E) -> E2>(self, default: E2, f: F) -> Result<T, E2> {
		match self {
			Some(Ok(t)) => Ok(t),
			Some(Err(e)) => Err(f(e)),
			None => Err(default),
		}
	}

	fn unwrap_or_else_map_err<E2, F: FnOnce(E) -> E2, F2: FnOnce() -> E2>(self, default: F2, f: F) -> Result<T, E2> {
		match self {
			Some(Ok(t)) => Ok(t),
			Some(Err(e)) => Err(f(e)),
			None => Err(default()),
		}
	}
}

impl<T, E> ResultOptionExt<T, E> for Result<Option<T>, E> {
	fn unwrap_or_err<E2>(self, err: E2) -> Result<T, E2> {
		match self {
			Ok(t) => match t {
				Some(t) => Ok(t),
				None => Err(err),
			},
			Err(_) => Err(err),
		}
	}

	fn unwrap_or_else_err<E2, F: FnOnce() -> E2>(self, f: F) -> Result<T, E2> {
		match self {
			Ok(t) => match t {
				Some(t) => Ok(t),
				None => Err(f()),
			},
			Err(_) => Err(f()),
		}
	}

	fn unwrap_or_map_err<E2, F: FnOnce(E) -> E2>(self, default: E2, f: F) -> Result<T, E2> {
		match self {
			Ok(t) => match t {
				Some(t) => Ok(t),
				None => Err(default),
			},
			Err(e) => Err(f(e)),
		}
	}

	fn unwrap_or_else_map_err<E2, F: FnOnce(E) -> E2, F2: FnOnce() -> E2>(self, default: F2, f: F) -> Result<T, E2> {
		match self {
			Ok(t) => match t {
				Some(t) => Ok(t),
				None => Err(default()),
			},
			Err(e) => Err(f(e)),
		}
	}
}
