// fn write_image(content: &mut Vec<u8>, output: &String) {
// 		if let Some(idx) = util::index_vec(content, &util::constants::EOF_SIGNATURE) {
// 				let write_idx = idx + util::constants::EOF_SIGNATURE.len() - 1;

// 				content.extend((69000_i64).to_be_bytes());
// 		} else {
// 				error!("Could not find PNG EOF signature!");
// 		}

// 		let output: &std::path::Path = std::path::Path::new(output);

// 		let _ = std::fs::write(output, &content);

// 		println!("{content:?}");

// 		println!("Done!");
// }

/// Encodes the given secret into an
/// image (provided via config)
/// @param config - the Config for the current run
/// @param secret - the data to be stored into the image
/// @param index - the index of the EOF pattern
/// ```rust
/// encode(config, "Hello, World!", 23456);
/// ```
pub fn encode(
    config: &crate::config::Config,
    content: &mut Vec<u8>,
    secret: &String,
    index: usize,
) {
    Encoder::new(config, content, secret, index).encode();
}

#[allow(dead_code)]
#[derive(Debug)]
struct Encoder<'a> {
    config: &'a crate::config::Config,
    content: &'a mut Vec<u8>,
    secret: &'a String,
    index: usize,
}

impl<'a> Encoder<'a> {
    pub fn new(
        config: &'a crate::config::Config,
        content: &'a mut Vec<u8>,
        secret: &'a String,
        index: usize,
    ) -> Self {
        Self {
            config,
            content,
            secret,
            index,
        }
    }

    /*
    The password that has been set can be checked by something like:

    if self.impassible_hash(&self.config.password.clone().unwrap())
        == self.impassible_hash(&"assword")
    {
        println!("Password ok!");
    } else {
        println!("pass: {impassible}");
        println!("ass: {}", self.impassible_hash(&"assword"));
    }

     */
    fn use_pass(&mut self, pass: &String) {
        println!("Incorporating password {pass}....");
    }

    fn encode(&mut self) {
        if let Some(pass) = &self.config.password {
            self.use_pass(pass);
        }

        let secret: String = self.secret.chars().rev().collect();

        let crypt = crate::util::crypt::encrypt_secret(&secret); // bcrypt::hash(secret, crate::util::constants::BCRYPT_COST).unwrap();
        println!("Overencryption: {crypt:?}");

        println!("Decrypted: {}", crate::util::crypt::decrypt_secret(&crypt));

        self.content.push(crypt.len() as u8);
        self.content.extend(crypt);
        if std::fs::write(&self.config.output_file, &self.content).is_err() {
            std::fs::create_dir(&self.config.output_file).unwrap();

            std::fs::write(&self.config.output_file, &self.content).unwrap();
        } else {
            println!("Done :)");
        }
    }
}
