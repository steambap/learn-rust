mod n01;
mod n02;
mod n03;
mod n04;
// mod n05;

pub fn is_prime(n: i32) -> bool {
	let sqrt_n = (n as f64).sqrt();
	let upper_range =
		if sqrt_n >= 2f64 {
			sqrt_n as i32 + 1
		} else {
			2i32
		};

	for fac in 2..upper_range {
		if n % fac == 0 {
			return false;
		}
	}

	true
}

#[cfg(test)]
mod tests {
	use super::is_prime;

	#[test]
	fn is_prime_works() {
		assert!(is_prime(7));
		assert!(is_prime(97));
		assert!(is_prime(2));
		assert!(!is_prime(4));
		assert!(!is_prime(91));
		assert!(!is_prime(99));
	}
}
