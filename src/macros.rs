/// A generic error facilitation macro.
/// Errors and exits with status 1.
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

/// Unreachable code error, hopefully is never to be displayed.
#[macro_export]
macro_rules! unreachable {
    ( $( $x:expr), *) => {{
        $(error!("Should be unreachable"))*
    }};
}

/// A generic info facilitation macro.
/// Just a fancy-er printf.
#[macro_export]
macro_rules! info {
		( $( $x:expr ),* ) => {{
						$(println!("[INFO]: {}", $x))*
		}};
}
