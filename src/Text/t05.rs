// check if palindrome

use super::t02::reverse_string;

#[allow(dead_code)]
pub fn is_palindrome(word: &str) -> bool {
	let rev = reverse_string(word);

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
