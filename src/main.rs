mod config;
mod impass;
mod macros;
mod reading;
mod util;

fn main() {
    let argv: Vec<String> = std::env::args().collect();
    let config = config::Config::from_args(&argv);

    let mut content = Vec::new();
    let index: usize = reading::gimme_bytecode(&config, &mut content);

    if config.mode == config::Mode::File {
        // We'll encode something, so get a secret
        let input = util::prompt("Enter your secret");

        impass::encoder::encode(&config, &input, index);
        println!(
            "Encoding into {}, using contents from {}... PASSWORD = {pass} and secret = {input}",
            config.output_file,
            config.file_to_read().clone().unwrap(),
            pass = if config.password.is_some() {
                config.password.unwrap()
            } else {
                "[none provided]".to_string()
            }
        );
    } else {
        println!("Decoding... {}", config.file_to_read().clone().unwrap());
    }
}
