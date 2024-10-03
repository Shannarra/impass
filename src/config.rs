use crate::error;

#[derive(Debug, Default)]
pub enum Mode {
		Write,
		Read,
		File,
		#[default]
		Unknown
}


#[derive(Debug, Default)]
pub struct Config {
		read_file: String,
		write_file: String,
		pub in_file: String,
		pub out_file: String,
		pub file: String,
		pub password: String
}

impl Config {
		pub fn set(&mut self, mode: Mode, file: String) {
				match mode {
						Mode::Read => { self.in_file = file },
						Mode::Write => { self.out_file = file; }
						Mode::File => { self.file = file; }

						_ => panic!("Should never be reached!")
				}
		}

		pub fn set_password(&mut self, pass: String) {
				self.password = pass;
		}

		pub fn filtered(mut self) -> Self {
				//if we have in_file don't care about the -file flag
				if !self.in_file.is_empty() {
						self.file.clear();
						self.read_file.clone_from(&self.in_file); //.clone();
				} else if self.file.is_empty() { // no in_file, is there a file?
						error!("No file to use provided.");
				} else {
						self.read_file.clone_from(&self.file); //.clone();
				}
				
				self.write_file = if self.out_file.is_empty() {
						let path = std::path::Path::new(&self.read_file);
						if let Some(filename) = path.file_name() {
								format!("result/{}", filename.to_str().unwrap())
						} else {
								error!(format!("Input given for file is not a valid path: {path:?}"));
						}
				} else {
						self.out_file.clone()
				};

				self
		}
		
		pub fn print_help(&self) {
				println!("Usage: impass [OPTIONS]\n
Where available options are:
\t-i, --input \tSet an input file
\t-o, --output\tSet an output file
\t-f, --file  \tSet a file to read or write
\t-p, --pass  \tSet a password to protect your file
");
		}

		pub fn from_args(argv: &[String]) -> Config {
				if argv.len() < 2 {
						error!("An argument for image must be provided!");
				}

				let mut idx = 1; // skip program name
				let mut config = Config::default();

				while idx < argv.len() {
						match argv[idx].as_str() {
								"-o" => {
										if let Some(out_file) = argv.get(idx + 1) {
												idx += 1;
												config.set(Mode::Write, out_file.clone());
										} else {
												error!(format!("File name must be provided after the {} flag!", argv[idx]));
										}
								},
								"-i" => {
										if let Some(in_file) = argv.get(idx + 1) {
												idx += 1;
												config.set(Mode::Read, in_file.clone());
										} else { 
												error!(format!("File name must be provided after the {} flag!", argv[idx]));
										}
								},
								"-f" => {
										if let Some(write_file) = argv.get(idx + 1) {
												idx += 1;
												config.set(Mode::File, write_file.clone());
										} else { 
												error!(format!("File name must be provided after the {} flag!", argv[idx]));
										}
								},
								"-p" => {
										if let Some(pass) = argv.get(idx + 1) {
												idx += 1;
												config.set_password(pass.clone());
										}
								},
								"-h" => {
										config.print_help();
										std::process::exit(0);
								},
								_ => {
										config.print_help();
										error!(format!("Unrecognized option or flag {}", argv[idx]));
								}
						}
						idx += 1;
				}

				config.filtered()
		}

		pub fn file_to_read(&self) -> &String {
				&self.read_file
		}

		pub fn file_to_write(&self) -> &String {
				&self.write_file
		}
}
