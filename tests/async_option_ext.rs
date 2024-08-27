use composable_utils::AsyncOptionExt;

async fn double(value: usize) -> usize {
	value * 2
}

#[test]
fn async_map() {
	async_io::block_on(async {
		let value = Some(69);
		let value = value
			.async_map(|v| async move { double(v).await })
			.await
			.unwrap_or_else(|| panic!("value should always be Some"));
		assert_eq!(value, 138);
	});
}
