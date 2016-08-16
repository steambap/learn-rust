// Prime Factorization

use super::is_prime;

#[allow(dead_code)]
pub fn get_prime_fac(number: i32) -> Vec<i32> {
	let mut factors = Vec::new();
	let sqrt_n = (number as f64).sqrt() as i32;
	let upper_range = sqrt_n + 1;

	for fac in 2..upper_range {
		if (number % fac == 0) && is_prime(fac) {
			factors.push(fac);
		}
	}

	factors
}

#[cfg(test)]
mod tests {
	use super::get_prime_fac;

	#[test]
	fn get_prime_fac_works() {
		assert_eq!(get_prime_fac(4), vec![2_i32]);
		assert_eq!(get_prime_fac(1001), vec![7_i32, 11, 13]);
	}
}
