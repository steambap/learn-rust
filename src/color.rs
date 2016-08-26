// hex to rgba

#[allow(dead_code)]
pub fn hex_rgb(hex: &str) -> (u32, u32, u32) {
	let hex = hex.trim().trim_left_matches('#');

	let count = hex.chars().count();

	let mut six_digit_hex = String::with_capacity(6);
	let hex = 
		if count == 3 {
			for c in hex.chars() {
				six_digit_hex.push(c);
				six_digit_hex.push(c);
			}

			six_digit_hex.as_str()
		} else {
			hex
		};

	let num = u32::from_str_radix(hex, 16)
		.expect("Expect a valid hex string");

	(num >> 16, num >> 8 & 255, num & 255)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn hex_rgb_works() {
		assert_eq!(hex_rgb("4183c4"), (65, 131, 196));
		assert_eq!(hex_rgb("#4183c4"), (65, 131, 196));
		assert_eq!(hex_rgb("#3264c8"), (50, 100, 200));
		assert_eq!(hex_rgb("#39c"), (51, 153, 204));
		assert_eq!(hex_rgb("#fff"), (255, 255, 255));
	}
}