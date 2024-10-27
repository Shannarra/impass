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

        let crypt = crate::utils::crypt::encrypt_secret(&secret, &self.config.env);
        println!("Overencryption: {crypt:?}");

        println!(
            "Decrypted: {}",
            crate::utils::crypt::decrypt_secret(&crypt, &self.config.env)
        );

        if &crate::utils::crypt::decrypt_secret(&crypt, &self.config.env) != self.secret {
            crate::error!("Your config seems to be incorrect. Please change its values or pass --generate-env once ro regenerate it and try running the program again");
        }

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

// TODO: tests
mod test {}
