use arr_macro::arr;

#[test]
fn int() {
	let result = crate::arr!(0, 1, 2;5, 3, 0;2, -1;6);
	assert_eq!(
		result,
		[0, 1, 2, 2, 2, 2, 2, 3, 0, 0, -1, -1, -1, -1, -1, -1]
	);
}

#[test]
fn string() {
	let result = crate::arr!("test", "double";2, "");
	assert_eq!(result, ["test", "double", "double", ""]);
}

#[derive(PartialEq, Debug)]
enum EnumTest {
	A,
	B,
	C,
}

#[test]
fn enum_test() {
	let result = crate::arr!(EnumTest::A, EnumTest::B, EnumTest::C;2);
	assert_eq!(result, [EnumTest::A, EnumTest::B, EnumTest::C, EnumTest::C])
}

#[test]
fn const_test() {
	let result = crate::arr!(i64::MIN, i64::MAX, i64::MIN;4);
	assert_eq!(
		result,
		[i64::MIN, i64::MAX, i64::MIN, i64::MIN, i64::MIN, i64::MIN]
	)
}
