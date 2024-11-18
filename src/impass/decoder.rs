pub fn decode(
    config: &crate::config::Config,
    content: &mut Vec<u8>,
    index: usize,
    output: bool,
) -> Option<String> {
    Decoder::new(config, content, index, output).decode()
}

#[derive(Debug)]
struct Decoder<'a> {
    config: &'a crate::config::Config,
    content: &'a mut Vec<u8>,
    index: usize,
    file: String,
    output: bool,
}

impl<'a> Decoder<'a> {
    pub fn new(
        config: &'a crate::config::Config,
        content: &'a mut Vec<u8>,
        index: usize,
        output: bool,
    ) -> Self {
        if let Some(file) = config.file_to_read().clone() {
            let index = index + crate::utils::constants::EOF_SIGNATURE.len();
            Self {
                config,
                content,
                index,
                file,
                output,
            }
        } else {
            crate::unreachable!("File to decode not provided");
        }
    }

    pub fn decode(&mut self) -> Option<String> {
        println!("Decoding... {}", self.file);
        let encoded_content = &self.content[self.index..];

        let mut iter_idex = 0;

        let has_pass = encoded_content[iter_idex];

        if has_pass == 1 {
            self.check_password(encoded_content, &mut iter_idex);
        }

        iter_idex += 1;

        let secret_size = encoded_content[iter_idex];
        iter_idex += 1;

        let secret_bytecode = &encoded_content[iter_idex..(secret_size as usize + iter_idex)];

        let result = crate::utils::crypt::decrypt_secret(secret_bytecode, &self.config.env);
        if self.output {
            println!("Secret: {}", result);
            None
        } else {
            Some(result)
        }
    }

    fn check_password(&self, encoded_content: &[u8], iter_idex: &mut usize) {
        crate::info!(format!("The file {} is password-protected.", self.file));

        let pass = if self.config.password.is_some() {
            crate::info!("Using password from config");
            self.config.password.clone().unwrap()
        } else {
            crate::utils::prompt("Please, enter your password")
        };

        *iter_idex += 1;
        let crypt_len = encoded_content[*iter_idex] as usize;
        *iter_idex += 1;
        let crypt = &encoded_content[*iter_idex..(crypt_len + *iter_idex)]
            .iter()
            .map(|c| *c as char)
            .collect::<String>();

        let hashed = crate::utils::impassible_hash(&pass);
        crate::info!("Verifying your password...");
        if let Ok(success) = bcrypt::verify(hashed.to_string(), crypt) {
            if success {
                crate::info!("Your password matches!");
                *iter_idex += crypt_len - 1;
            } else {
                crate::error!("Passwords do not match!");
            }
        } else {
            crate::unreachable!("BCrypt unable to verify hash.");
        }
    }
}

mod test {
    #[allow(unused_imports)]
    use crate::impass::test::{generate_config, get_content};

    #[test]
    fn decode_without_password() {
        let cfg = generate_config(
            None,
            crate::config::Mode::Read,
            "tests/nopass.png".to_string(),
        );
        let mut bytes = vec![];
        let index = get_content(&cfg, &mut bytes);

        let secret = super::decode(&cfg, &mut bytes, index, false).unwrap();

        assert_eq!(secret, "hello world!".to_string());
    }

    #[test]
    fn decode_with_password() {
        let cfg = generate_config(
            Some("asdasd".to_string()),
            crate::config::Mode::Read,
            "tests/asdasd_pass.png".to_string(),
        );
        let mut bytes = vec![];
        let index = get_content(&cfg, &mut bytes);

        let secret = super::decode(&cfg, &mut bytes, index, false).unwrap();

        assert_eq!(secret, "hello world!".to_string());
    }

    #[test]
    #[should_panic(
        expected = "Password provided contains invalid characters. Please, use ASCII-only characters!"
    )]
    fn disallows_decoding_with_invalid_pass() {
        let cfg = generate_config(
            Some("невалидна".to_string()), // "invalid"
            crate::config::Mode::Read,
            "tests/asdasd_pass.png".to_string(),
        );
        let mut bytes = vec![];
        let index = get_content(&cfg, &mut bytes);

        let secret = super::decode(&cfg, &mut bytes, index, false).unwrap();

        assert_eq!(secret, "hello world!".to_string());
    }

    #[test]
    #[should_panic(expected = "Passwords do not match!")]
    fn disallows_decoding_with_wrong_pass() {
        let cfg = generate_config(
            Some("wrongpass".to_string()),
            crate::config::Mode::Read,
            "tests/asdasd_pass.png".to_string(),
        );
        let mut bytes = vec![];
        let index = get_content(&cfg, &mut bytes);

        let secret = super::decode(&cfg, &mut bytes, index, false).unwrap();

        assert_eq!(secret, "hello world!".to_string());
    }
}
