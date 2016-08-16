// Next Prime Number

use super::is_prime;

#[allow(dead_code)]
pub fn get_next_prime(number: i32) -> i32 {
	if is_prime(number) {
		return number;
	}
	let mut next_possible_prime = 
		if number % 2 == 0 {
			number + 1
		} else {
			number + 2
		};

	match is_prime(next_possible_prime) {
		true => next_possible_prime,
		false => get_next_prime(next_possible_prime)
	}
}

#[cfg(test)]
mod tests {
	use super::get_next_prime;

	#[test]
	fn get_next_prime() {
		assert_eq!(2, get_next_prime(2));
		assert_eq!(997, get_next_prime(992));
		assert_eq!(967, get_next_prime(954));
	}
}
