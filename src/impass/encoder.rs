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

pub fn encode(config: &crate::config::Config, input: &String, index: usize) {
    println!("{:?}", Encoder::new(config, input, index));
}

#[derive(Debug)]
struct Encoder<'a> {
    config: &'a crate::config::Config,
    input: &'a String,
    index: usize,
}

impl<'a> Encoder<'a> {
    pub fn new(config: &'a crate::config::Config, input: &'a String, index: usize) -> Self {
        Self {
            config,
            input,
            index,
        }
    }
}
