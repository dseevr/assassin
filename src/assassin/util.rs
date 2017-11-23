pub fn format_money(f: f64) -> String {
	if f >= 1_000_000_000.0 {
		panic!("number is too big");
	}

	if f <= -1_000_000_000.0 {
		panic!("number is too small");
	}

	let positive = f >= 0.0;
	let abs_f = f.abs();
	let i = f.abs() as i64;

	let mantissa = abs_f - (abs_f as i64) as f64;
	let decimal = (mantissa * 100.0).round() as i64;

	let weird_edge_case = ! positive && decimal == 0 && f >= -0.005;

	let final_string = add_commas(i);

	let sign = if positive || weird_edge_case { "" } else { "-" }.to_string();

	format!("{}${}.{:0>2}", sign, final_string, decimal)
}

pub fn add_commas<T: ToString>(input: T) -> String {
	let num_digits = input.to_string().as_bytes().len();

	let power_of_1000 = num_digits % 3 == 0;

	let num_commas = if num_digits > 3 {
		let num = num_digits / 3;

		// if the integer is a multiple of 3, drop one comma
		if power_of_1000 { num - 1 } else { num }
	} else {
		0
	};

	let s = input.to_string();
	let mut left_offset = if num_commas > 0 { s.len() % 3 as usize } else { 0 };
	let mut byte_string = s.as_bytes().to_vec();

	if num_commas > 0 {

		if power_of_1000 {
			left_offset = 3;
			// don't need to delete a comma because that was done above
		}

		for _ in 1..(num_commas+1) {
			byte_string.insert(left_offset, ",".as_bytes()[0]);
			left_offset += 3 + 1; // +1 to account for the byte we inserted	
		}
	}

	String::from_utf8(byte_string).unwrap()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn add_commas_works() {
		assert!(add_commas(1)             == "1");
		assert!(add_commas(10)            == "10");
		assert!(add_commas(100)           == "100");
		assert!(add_commas(1_000)         == "1,000");
		assert!(add_commas(10_000)        == "10,000");
		assert!(add_commas(100_000)       == "100,000");
		assert!(add_commas(1_000_000)     == "1,000,000");
		assert!(add_commas(10_000_000)    == "10,000,000");
		assert!(add_commas(100_000_000)   == "100,000,000");
		assert!(add_commas(1_000_000_000) == "1,000,000,000")
	}

	fn test(f: f64, s: &str) {
		let result = format_money(f);

		println!("Checking format_money({})", f);
		println!("Expected: {}", s);
		println!("Got: {}", result);
		println!("");

		assert!(result == s);
	}

	#[test]
	fn format_money_works() {
		println!("");

		// zero
		test(0.0,   "$0.00");
		test(0.005, "$0.01");

		// negative zero edge case
		test(-0.001, "$0.00");
		test(-0.005, "-$0.01");

		// rounding
		test(1.234, "$1.23");
		test(1.235, "$1.24");
		test(-1.234, "-$1.23");
		test(-1.235, "-$1.24");

		// positive
		test(12.30,          "$12.30");
		test(123.0,          "$123.00");
		test(123.45,         "$123.45");
		test(1_000.00,       "$1,000.00");
		test(12_345.67,      "$12,345.67");
		test(100_000.00,     "$100,000.00");
		test(12_345_678.90,  "$12,345,678.90");
		test(112_345_678.90, "$112,345,678.90");

		// negative
		test(-12.30,          "-$12.30");
		test(-123.0,          "-$123.00");
		test(-123.45,         "-$123.45");
		test(-1_000.00,       "-$1,000.00");
		test(-12_345.67,      "-$12,345.67");
		test(-100_000.00,     "-$100,000.00");
		test(-12_345_678.90,  "-$12,345,678.90");
		test(-112_345_678.90, "-$112,345,678.90");

		// edge cases
		test(343.49999999999994, "$343.50");
	}

	#[test]
	#[should_panic]
	fn format_money_rejects_huge_numbers() {
		format_money(999_999_999_999.0);
	}

	#[test]
	#[should_panic]
	fn format_money_rejects_huge_negative_numbers() {
		format_money(-999_999_999_999.0);
	}
}
