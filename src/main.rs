mod util;
mod config;

use config::{Config, Mode};

fn index<T>(haystack: &[T], needle: &[T]) -> Option<usize> where
T: Sized + Eq + PartialEq + std::fmt::Debug {
		for (position, window) in haystack.windows(needle.len()).enumerate() {
				println!("{window:?} ==? {needle:?}");
				if window == needle {
						return Some(position);
				}
		}
		None
}
//TODO: remove mode and use file name

fn read_and_verify_input(content: &mut Vec<u8>, filename: &String) {
		if let Err(err) = std::fs::read(filename) {
				error!(err);
		}
		let text = std::fs::read(filename).unwrap();
		content.extend(text);
		
		let png_signature: Vec<u8> = vec![137, 80, 78, 71, 13, 10, 26, 10];
		if content[0..8] != png_signature {
				error!("Image provided is not a PNG.");
		}		 
}

fn write_image(content: &mut Vec<u8>, output: &String) {
		if let Some(idx) = index(content, &util::constants::EOF_SIGNATURE) {
				let write_idx = idx + util::constants::EOF_SIGNATURE.len() - 1;

				content.extend((69000_i32).to_be_bytes());
		} else {
				error!("Could not find PNG EOF signature!");
		}
		
		let output: &std::path::Path = std::path::Path::new(output);
		
		let _ = std::fs::write(output, &content);
		
		println!("{content:?}");

		println!("Done!");		
}

mod reading;

fn main() {
		let argv: Vec<String> = std::env::args().collect();
		let config = config::Config::from_args(&argv);

		//let mut content = Vec::new(); 
		//read_and_verify_input(&mut content, &argv[1]);

		error!(format!("Config: {config:?}"));
		
		// match mode.mode {
		// 		Mode::Read(file) => { todo!("read") }
		// 		Mode::Write(file) => {
		// 				todo!("write")
		// 				// write_image(&mut content, file)
		// 		}
		// }

}
