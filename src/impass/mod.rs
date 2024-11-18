pub mod decoder;
pub mod encoder;

pub(crate) mod test {
    // Stop Clippy from complaining
    #[allow(dead_code)]
    pub fn generate_config(
        pass: Option<String>,
        mode: crate::config::Mode,
        image: String,
    ) -> crate::config::Config {
        crate::config::Config::new(
            mode,
            image,
            pass,
            "output.png".to_string(),
            crate::utils::env::collect_env(crate::utils::Env::new()),
        )
    }

    #[allow(dead_code)]
    pub fn get_content(config: &crate::config::Config, content: &mut Vec<u8>) -> usize {
        crate::reading::gimme_bytecode(config, content)
    }
}
