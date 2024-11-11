pub fn decode(config: &crate::config::Config, content: &mut Vec<u8>, index: usize) {
    Decoder::new(config, content, index).decode();
}

#[derive(Debug)]
struct Decoder<'a> {
    content: &'a mut Vec<u8>,
    index: usize,
    file: String,
}

impl<'a> Decoder<'a> {
    pub fn new(config: &'a crate::config::Config, content: &'a mut Vec<u8>, index: usize) -> Self {
        if let Some(file) = config.file_to_read().clone() {
            let index = index + crate::utils::constants::EOF_SIGNATURE.len();
            Self {
                content,
                index,
                file,
            }
        } else {
            crate::unreachable!("File to decode not provided");
        }
    }

    pub fn decode(&self) {
        println!("Decoding... {}", self.file);
        let encoded_content = &self.content[self.index..];

        let mut iter_idex = 0;

        let has_pass = encoded_content[iter_idex];

        if has_pass == 1 {
            self.check_password(encoded_content, &mut iter_idex);
        }
    }

    fn check_password(&self, encoded_content: &[u8], iter_idex: &mut usize) {
        crate::info!(format!("The file {} is password-protected.", self.file));

        let pass = crate::utils::prompt("Please, enter your password");

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
            } else {
                crate::error!("Passwords do not match!");
            }
        } else {
            crate::unreachable!("BCrypt unable to verify hash.");
        }
    }
}

mod test {}
