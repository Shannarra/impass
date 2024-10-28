use std::io::Write;

/// Checks the values of provided environment variables,
/// if they have been given by the user.
fn check_env_vars(env: super::Env) -> super::Env {
    if let Some(shr) = env.get("SHIFT") {
        let val = shr.parse::<u32>();
        if val.is_err() {
            crate::error!("Number provided for SHIFT must be a positive whole number!")
        }
    }

    if let Some(godnum) = env.get("GODNUM") {
        let val = godnum.parse::<u32>();
        if val.is_err() {
            crate::error!("Number provided for GODNUM must be a positive whole number!")
        }
    }

    if let Some(xor) = env.get("XOR") {
        let val = xor.parse::<u32>();
        if val.is_err() {
            crate::error!("Number provided for XOR must be a positive whole number!")
        }
    }

    env
}

/// Checks, collects and formats the environment variables
/// in a format that's easy to use by `impass`.
pub fn collect_env(env: super::Env) -> super::Env {
    let env = self::check_env_vars(env);
    super::Env::from([
        (
            "shift".to_string(),
            env.get("SHIFT").unwrap_or(&"11".to_string()).to_owned(),
        ),
        (
            "godnum".to_string(),
            env.get("GODNUM").unwrap_or(&"42".to_string()).to_owned(),
        ),
        (
            "xor".to_string(),
            env.get("XOR").unwrap_or(&"69".to_string()).to_owned(),
        ),
        (
            "answers".to_string(),
            env.get("ANSWERS")
                .unwrap_or(&"y,yes".to_string())
                .to_owned(),
        ),
    ])
}

fn check_autogen() {
    let env = std::env::vars().collect::<std::collections::HashMap<String, String>>();
    let env = collect_env(env);

    let secret = "this is my test secret";
    let encrypted = super::crypt::encrypt_secret(secret, &env);
    let decrypted = super::crypt::decrypt_secret(&encrypted, &env);

    if decrypted != secret {
        // crate::info!("Generated config file was incorrect; retrying...");
        _ = generate_env();
    }
}

/// Generates and populates a .env file with "random"
/// parameters for the environment.
pub fn generate_env() -> std::io::Result<()> {
    let mut f = std::fs::File::create(".env")?;

    let buf = format!(
        "SHIFT={sh}
GODNUM={gn}
XOR={xor}
ANSWERS=\"y,yes\"
",
        sh = super::within_range(super::rand(), 8, 16),
        gn = super::within_range(super::rand(), 8, 16),
        xor = super::within_range(super::rand(), 8, 31)
    );
    f.write_all(buf.as_bytes())?;
    f.flush()?;

    check_autogen();

    Ok(())
}

mod test {
    #[test]
    fn env_checks_correctly() {
        let normal_env = std::collections::HashMap::<String, String>::from([
            ("GODNUM".to_string(), "69".to_string()),
            ("shitenv".to_string(), "69".to_string()),
        ]);

        let res = super::check_env_vars(normal_env.clone());

        assert_eq!(normal_env, res)
    }

    #[test]
    #[should_panic]
    fn env_fails_check_correctly() {
        let normal_env = std::collections::HashMap::<String, String>::from([(
            "GODNUM".to_string(),
            "invalid value here".to_string(),
        )]);

        let _ = super::check_env_vars(normal_env.clone());
    }
}
