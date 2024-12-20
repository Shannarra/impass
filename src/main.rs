mod config;
mod impass;
mod macros;
mod reading;
mod utils;

fn main() {
    dotenv::dotenv().ok();

    let env = std::env::vars().collect::<std::collections::HashMap<String, String>>();
    let argv: Vec<String> = std::env::args().collect();

    // Use the commented env for generating test files:
    let simplified_env = utils::env::collect_env(env); // utils::env::collect_env(utils::Env::new());
    let config = config::Config::from_args(&argv, simplified_env);

    let mut content = Vec::new();
    let index: usize = reading::gimme_bytecode(&config, &mut content);

    if config.mode == config::Mode::File || config.mode == config::Mode::Write {
        // We'll encode something, so get a secret
        let input = utils::prompt("Enter your secret");

        impass::encoder::encode(&config, &mut content, &input);
        println!(
            "Encoding into {}, using contents from {}... PASSWORD = \"{pass}\" and secret = {input}",
            config.output_file,
            config.file_to_read().clone().unwrap(),
            pass = if config.password.is_some() {
                config.password.unwrap()
            } else {
                "[none provided]".to_string()
            }
        );
    } else {
        // todo: add functionality to have number of tries against a password-protected secret
        impass::decoder::decode(&config, &mut content, index, true);
    }
}
