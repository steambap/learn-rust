// Fizz Buzz

#[allow(dead_code)]
pub fn fizz_buzz() {
	for x in 1..101 {
		if x % 15 == 0 {
			println!("FizzBuzz");
		} else if x % 3 == 0 {
			println!("Fizz");
		} else if x % 5 == 0 {
			println!("Buzz");
		} else {
			println!("{}", x);
		}
	}
}

#[cfg(test)]
mod tests {
	use super::fizz_buzz;

	#[test]
	fn fizz_buzz_works() {
		fizz_buzz();
	}
}
