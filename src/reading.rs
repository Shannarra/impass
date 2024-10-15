use crate::config;
use crate::util::{constants, index_vec};

fn read_and_verify_input(content: &mut Vec<u8>, filename: &String) {
    let file = std::fs::read(filename);
    if let Err(err) = file {
        crate::error!(err);
    }
    let text = file.unwrap();

    let png_signature: Vec<u8> = vec![137, 80, 78, 71, 13, 10, 26, 10];
    if text[0..8] != png_signature {
        crate::error!(format!("Image provided is not a PNG: {filename}"));
    }

    content.extend(text);
}

pub fn gimme_bytecode(config: &config::Config, content: &mut Vec<u8>) -> usize {
    read_and_verify_input(content, &config.file_to_read().clone().unwrap());

    if let Some(idx) = index_vec(content, &constants::EOF_SIGNATURE) {
        idx
    } else {
        crate::error!(format!(
            "File is not a valid PNG: {}",
            config.file_to_read().clone().unwrap()
        ));
    }
}
