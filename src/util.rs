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
