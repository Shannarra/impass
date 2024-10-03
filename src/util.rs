pub mod macros {
		#[macro_export]
		macro_rules! error {
				( $( $x:expr ),* ) => {
						{
								$(
										println!("[ERROR]: {}", $x);

										#[cfg(not(test))]
										std::process::exit(1);

										#[cfg(test)]
										panic!("{}", $x);
								)*
						}
				};
		}
}

pub mod constants {
		pub static EOF_SIGNATURE: [u8; 12] = [0, 0, 0, 0, 73, 69, 78, 68, 174, 66, 96, 130];
}

pub fn index_vec<T>(haystack: &[T], needle: &[T]) -> Option<usize> where
		T: Sized + Eq + PartialEq + std::fmt::Debug {
		for (position, window) in haystack.windows(needle.len()).enumerate() {
				if window == needle {
						return Some(position);
				}
		}
		None
}

pub fn prompt(message: &str) -> String {
		print!("{message}: ");
		use std::io;
		use std::io::*;
		let _ = std::io::stdout().flush();

		let mut input = String::new();
		io::stdin().read_line(&mut input).unwrap();
		input
}
