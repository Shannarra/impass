use crate::config;
use crate::utils::{constants, index_vec};

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

mod test {
    #[test]
    fn verify_correct_input() {
        let mut content = Vec::new();
        super::read_and_verify_input(&mut content, &String::from("images/cat.png"));

        assert!(content.len() > 0) // no panics
    }

    #[test]
    #[should_panic(expected = "Image provided is not a PNG: images/kekw.jpg")]
    fn verify_incorrect_input() {
        let mut content = Vec::new();
        super::read_and_verify_input(&mut content, &String::from("images/kekw.jpg"));
        // should have paniced
        // thus content is not changed
        assert!(content.len() == 0)
    }

    #[test]
    #[should_panic(expected = "No such file or directory (os error 2)")]
    fn verify_nonexistent_input() {
        let mut content = Vec::new();
        super::read_and_verify_input(&mut content, &String::from("images/dont_exist.asdasd"));
        // should have paniced
        // thus content is not changed
        assert!(content.len() == 0)
    }

    #[test]
    fn correct_input_bytecode() {
        let cfg = super::config::Config::from_args(&[
            "--".to_string(), // needed to distinguish the initial arg (program name)
            "-f".to_string(),
            "images/cat.png".to_string(),
        ]);

        let mut content = Vec::new();
        let idx = super::gimme_bytecode(&cfg, &mut content);

        assert!(content.len() > 0);
        assert!(idx > 0) // no panics
    }

    #[test]
    #[should_panic(expected = "The given file to read \"nonexistent.shit\" does not exist!")]
    fn incorrect_input_bytecode() {
        let cfg = super::config::Config::from_args(&[
            "--".to_string(), // needed to distinguish the initial arg (program name)
            "-f".to_string(),
            "nonexistent.shit".to_string(),
        ]);

        let mut content = Vec::new();
        let _ = super::gimme_bytecode(&cfg, &mut content);
        // paniced
    }
}
