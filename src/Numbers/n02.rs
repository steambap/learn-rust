// Find e to the Nth Digit

use std::f64::consts::E;

#[allow(dead_code)]
pub fn get_euler(size: i32) -> String {
	let e = E.to_string();
	let (ret, _) = e.split_at(size as usize + 2);

	return ret.to_string();
}

#[cfg(test)]
mod tests {
	use super::get_euler;

	#[test]
	fn get_euler_works() {
		let my_e = get_euler(10);

		let mut count = 0;

		for _ in my_e.chars() {
			count += 1;
		}

		assert!(count == 12);
	}
}
