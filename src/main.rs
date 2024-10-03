mod config;
mod util;

// fn write_image(content: &mut Vec<u8>, output: &String) {
// 		if let Some(idx) = util::index_vec(content, &util::constants::EOF_SIGNATURE) {
// 				let write_idx = idx + util::constants::EOF_SIGNATURE.len() - 1;

// 				content.extend((69000_i32).to_be_bytes());
// 		} else {
// 				error!("Could not find PNG EOF signature!");
// 		}

// 		let output: &std::path::Path = std::path::Path::new(output);

// 		let _ = std::fs::write(output, &content);

// 		println!("{content:?}");

// 		println!("Done!");
// }

mod reading;

fn main() {
    let argv: Vec<String> = std::env::args().collect();
    let config = config::Config::from_args(&argv);

    let mut content = Vec::new();
    let index: usize = reading::gimme_bytecode(&config, &mut content);

    if config.in_file.is_empty() {
        let input = util::prompt("Enter your secret");
        println!("Input: {input}");
    } else {
        println!("Decoding... {}", config.in_file);
    }
}
