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

#[macro_export]
macro_rules! unreachable {
    ( $( $x:expr), *) => {{
        $(error!("Should be unreachable"))*
    }};
}
