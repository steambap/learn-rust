// check if palindrome

#[allow(dead_code)]
pub fn is_palindrome(word: &str) -> bool {
	let rev = word.chars().rev().collect::<String>();

	rev == word
}

#[cfg(test)]
mod tests {
	use super::is_palindrome;

	#[test]
	fn is_palindrome_works() {
		assert!(is_palindrome("racecar"));
		assert!(!is_palindrome("傻逼"));
	}
}
