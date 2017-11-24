pub fn add_commas<T: ToString>(input: T) -> String {
	// TODO: replace with a loop that divides repeatedly
	// int digits = 0; while (number != 0) { number /= 10; digits++; }
	let num_digits = input.to_string().as_bytes().len();

	let power_of_1000 = num_digits % 3 == 0;
	let mut num_commas = if num_digits > 3 { num_digits / 3 } else { 0 };

	let s = input.to_string();
	let mut left_offset = if num_commas > 0 { s.len() % 3 as usize } else { 0 };
	let mut byte_string = s.as_bytes().to_vec();

	if num_commas > 0 {
		if power_of_1000 {
			left_offset = 3;
			num_commas -= 1;
		}

		for _ in 1..(num_commas+1) {
			byte_string.insert(left_offset, ",".as_bytes()[0]);
			left_offset += 3 + 1; // +1 to account for the byte we inserted	
		}
	}

	String::from_utf8(byte_string).unwrap()
}