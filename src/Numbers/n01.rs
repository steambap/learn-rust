// Find PI to the Nth Digit
/*
Enter a number and have the program generate PI up to that many decimal places. 
Keep a limit to how far the program will go.
*/
use std::f64::consts::PI;

#[allow(dead_code)]
pub fn get_pi(size: i32) -> String {
	let pi = PI.to_string();
	let (result, _) = pi.split_at(size as usize + 2);

	return result.to_string();
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn get_pi_works() {
		let my_pi = get_pi(10);
		let mut count = 0;

		for _ in my_pi.chars() {
			count += 1;
		}
		assert!(count == 12);
	}
}
