use crate::error;

/// The mode in which the config will work for
/// the current run.
#[derive(Debug, Default, PartialEq)]
pub enum Mode {
    Write,
    Read,
    File,
    #[default]
    Unknown,
}

/// A simple configuration setup for the
/// run. Exposes a password, mode, output
/// file and environment to be propagated and used
/// during runtime for both encryption/decription.
#[derive(Debug, Default)]
pub struct Config {
    read_file: Option<String>,
    write_file: Option<String>,
    file: Option<String>,
    pub password: Option<String>,
    pub mode: Mode,
    pub output_file: String,
    pub env: std::collections::HashMap<String, String>,
}

impl Config {
    /// Sets a mode by modifying self. Defaults to Write.
    fn set_mode(mut self) -> Self {
        if self.file.is_some() {
            self.mode = Mode::File;
        }

        if self.read_file.is_some() {
            self.mode = Mode::Read;
        }

        if self.mode == Mode::Unknown {
            self.mode = Mode::Write;
        }

        self
    }

    /// Sets the output file by modifying self.
    /// Useful as an accessor down the work process.
    fn set_output(mut self) -> Self {
        if let Some(out) = &self.write_file {
            self.output_file.clone_from(out); // = out.clone();
        } else {
            let file = if let Some(f) = &self.file {
                f
            } else if let Some(f) = &self.read_file {
                f
            } else {
                unreachable!()
            }
            .clone();

            let path = std::path::Path::new(&file);
            self.output_file = format!("result/{}", path.file_name().unwrap().to_str().unwrap());
        }

        self
    }

    /// Sets an env by modifying self.
    pub fn with_env(mut self, env: std::collections::HashMap<String, String>) -> Self {
        self.env = env;

        self
    }

    /// Checks if configuration is valid before
    /// allowing it to propagate in runtime.
    fn checked(self) -> Self {
        if self.mode == Mode::Write && self.file_to_read().is_none() {
            error!("A file name to write (output) was provided, but no file to use was given");
        } // we good with mode

        if let Some(file) = self.file_to_read() {
            if !std::path::Path::new(&file).exists() {
                error!(format!("The given file to read \"{file}\" does not exist!"))
            }
        }

        self
    }

    /// Sets a password for self.
    pub fn set_password(&mut self, pass: String) {
        self.password = Some(pass);
    }

    /// A help message.
    pub fn print_help(&self) {
        println!(
            "Usage: impass [OPTIONS]\n
Where available options are:
\t-i, --input \tSet an input file
\t-o, --output\tSet an output file
\t-f, --file  \tSet a file to read or write
\t-p, --pass  \tSet a password to protect your file
"
        );
    }

    /// Constructs a Self from a list of args.
    /// Expected to be command-line provided, but
    /// could also be just in cli-style.
    pub fn from_args(argv: &[String]) -> Config {
        if argv.len() < 2 {
            error!("An argument for image must be provided!");
        }

        let mut idx = 1; // skip program name
        let mut config = Config::default();

        while idx < argv.len() {
            match argv[idx].as_str() {
                "-o" | "--output" => {
                    if let Some(out_file) = argv.get(idx + 1) {
                        idx += 1;
                        config.write_file = Some(out_file.to_string());
                    } else {
                        error!(format!(
                            "File name must be provided after the {} flag!",
                            argv[idx]
                        ));
                    }
                }
                "-i" | "--input" => {
                    if let Some(in_file) = argv.get(idx + 1) {
                        idx += 1;
                        config.read_file = Some(in_file.to_string());
                    } else {
                        error!(format!(
                            "File name must be provided after the {} flag!",
                            argv[idx]
                        ));
                    }
                }
                "-f" | "--file" => {
                    if let Some(write_file) = argv.get(idx + 1) {
                        idx += 1;
                        config.file = Some(write_file.to_string());
                    } else {
                        error!(format!(
                            "File name must be provided after the {} flag!",
                            argv[idx]
                        ));
                    }
                }
                "-p" | "--pass" => {
                    if let Some(pass) = argv.get(idx + 1) {
                        idx += 1;
                        config.set_password(pass.clone());
                    }
                }
                "-h" | "--help" => {
                    config.print_help();
                    std::process::exit(0);
                }
                "--create-env" | "--generate-env" => {
                    idx += 1;
                    if crate::utils::env::generate_env().is_err() {
                        error!("Could not generate a config properly!");
                    } else {
                        crate::info!("Config was generated successfully!");
                    }
                }
                _ => {
                    config.print_help();
                    error!(format!("Unrecognized option or flag {}", argv[idx]));
                }
            }
            idx += 1;
        }

        config.set_mode().set_output().checked()
    }

    /// An accessor to the input file the runtime will use.
    pub fn file_to_read(&self) -> &Option<String> {
        match self.mode {
            Mode::File => &self.file,
            Mode::Read => &self.read_file,
            _ => &None,
        }
    }
}

mod test {
    mod config {
        #[test]
        fn can_create_config_from_valid_args() {
            let cfg = super::super::Config::from_args(&[
                "--".to_string(), // needed to distinguish the initial arg (program name)
                "-f".to_string(),
                "images/cat.png".to_string(),
                "-o".to_string(),
                "out/output.png".to_string(),
                "-i".to_string(),
                "images/harold.png".to_string(),
            ]);

            // File mode got overwritten by last -i
            assert_eq!(cfg.mode, super::super::Mode::Read);
            // The in_file was also overwritten
            assert_eq!(cfg.file_to_read(), &Some("images/harold.png".to_owned()));
            assert_eq!(cfg.output_file, "out/output.png".to_owned());
            assert_eq!(cfg.password, None);
        }

        #[test]
        fn can_create_config_from_valid_args_with_pass() {
            let cfg = super::super::Config::from_args(&[
                "--".to_string(), // needed to distinguish the initial arg (program name)
                "-f".to_string(),
                "images/cat.png".to_string(),
                "-o".to_string(),
                "out/output.png".to_string(),
                "-i".to_string(),
                "images/harold.png".to_string(),
                "--pass".to_string(),
                "password123!".to_string(),
            ]);

            // File mode got overwritten by last -i
            assert_eq!(cfg.mode, super::super::Mode::Read);
            // The in_file was also overwritten
            assert_eq!(cfg.file_to_read(), &Some("images/harold.png".to_owned()));
            assert_eq!(cfg.output_file, "out/output.png".to_owned());
            assert_eq!(cfg.password, Some("password123!".to_string()));
        }

        #[test]
        #[should_panic(expected = "An argument for image must be provided!")]
        fn cant_create_config_from_invalid_args() {
            let _ = super::super::Config::from_args(&[
                "--".to_string(), // needed to distinguish the initial arg (program name),
            ]);
            // none
        }

        #[test]
        #[should_panic(
            expected = "A file name to write (output) was provided, but no file to use was given"
        )]
        fn cant_create_config_with_unfinished_args() {
            let _ = super::super::Config::from_args(&[
                "--".to_string(), // needed to distinguish the initial arg (program name),
                "-o".to_string(),
                "out/output.png".to_string(),
            ]);
            // none
        }

        // flags
        #[test]
        #[should_panic(expected = "File name must be provided after the -o flag!")]
        fn test_invalid_o_flag() {
            let _ = super::super::Config::from_args(&[
                "--".to_string(), // needed to distinguish the initial arg (program name),
                "-o".to_string(),
            ]);
        }

        #[test]
        #[should_panic(expected = "File name must be provided after the -f flag!")]
        fn test_invalid_f_flag() {
            let _ = super::super::Config::from_args(&[
                "--".to_string(), // needed to distinguish the initial arg (program name),
                "-f".to_string(),
            ]);
        }

        #[test]
        #[should_panic(expected = "File name must be provided after the -i flag!")]
        fn test_invalid_i_flag() {
            let _ = super::super::Config::from_args(&[
                "--".to_string(), // needed to distinguish the initial arg (program name),
                "-i".to_string(),
            ]);
        }

        #[test]
        #[should_panic(expected = "Unrecognized option or flag -asdkashdkajsdhkhk")]
        fn test_invalid_x_flag() {
            let _ = super::super::Config::from_args(&[
                "--".to_string(), // needed to distinguish the initial arg (program name),
                "-asdkashdkajsdhkhk".to_string(),
            ]);
        }
    }
}
