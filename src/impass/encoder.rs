/// Encodes the given secret into an
/// image (provided via config)
/// @param config - the Config for the current run
/// @param secret - the data to be stored into the image
/// ```rust
/// encode(config, "Hello, World!", 23456);
/// ```
pub fn encode(config: &crate::config::Config, content: &mut Vec<u8>, secret: &String) {
    Encoder::new(config, content, secret).encode();
}

#[derive(Debug)]
struct Encoder<'a> {
    config: &'a crate::config::Config,
    content: &'a mut Vec<u8>,
    secret: &'a String,
}

impl<'a> Encoder<'a> {
    pub fn new(
        config: &'a crate::config::Config,
        content: &'a mut Vec<u8>,
        secret: &'a String,
    ) -> Self {
        Self {
            config,
            content,
            secret,
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
        crate::info!("Incorporating password...");
        self.content.push(1);

        use super::super::utils;

        let hashed = utils::impassible_hash(pass);
        crate::info!("Encrypting your passsword..");
        if let Ok(res) = bcrypt::hash(hashed.to_string(), utils::constants::BCRYPT_COST) {
            let len = res.len();
            self.content.push(len as u8);
            self.content.extend(res.chars().map(|x| x as u8));
        } else {
            crate::unreachable!("BCrypt failed for some reason");
        }
    }

    fn encode(&mut self) {
        if let Some(pass) = &self.config.password {
            self.use_pass(pass);
        } else {
            // NO_HAS_PASS
            self.content.push(0);
        }

        let secret: String = self.secret.chars().rev().collect();

        let crypt = crate::utils::crypt::encrypt_secret(&secret, &self.config.env);

        if &crate::utils::crypt::decrypt_secret(&crypt, &self.config.env) != self.secret {
            crate::error!("Your config seems to be incorrect. Please change its values or pass --generate-env once ro regenerate it and try running the program again");
        }

        self.content.push(crypt.len() as u8);

        // TODO: not a bad idea to be able to store the config env
        // within the content?

        self.content.extend(crypt);
        self.save_file()
    }

    fn save_file(&self) {
        if std::fs::write(&self.config.output_file, &self.content).is_err() {
            if std::fs::create_dir(&self.config.output_file).is_ok() {
                std::fs::write(&self.config.output_file, &self.content).unwrap();
            } else {
                crate::error!(format!("Could not create your output file \"{}\". Does the program have the right permissions?", &self.config.output_file));
            }
        }
        println!("Done :)");
    }
}

// TODO: tests
mod test {}
