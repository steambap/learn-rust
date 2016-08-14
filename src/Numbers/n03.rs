// Fibonacci Sequence

#[allow(dead_code)]
pub fn get_fib(size: i32) -> i32 {
	match size {
		0 => 0,
		1 => 1,
		_ => get_fib(size - 1) + get_fib(size - 2)
	}
}

#[cfg(test)]
mod tests {
	use super::get_fib;

	#[test]
	fn get_fib_works() {
		assert_eq!(get_fib(10), 55);
		assert_eq!(get_fib(20), 6765);
	}
}