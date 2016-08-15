// Reverse a String

pub fn reverse_string(word: &str) -> String {
	word.chars().rev().collect::<String>()
}

#[cfg(test)]
mod tests {
	use super::reverse_string;

	#[test]
	fn reverse_string_works() {
		assert_eq!(reverse_string("hello, world!"), "!dlrow ,olleh");
	}
}
