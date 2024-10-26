type Env = std::collections::HashMap<String, String>;

/// Checks the values of provided environment variables,
/// if they have been given by the user.
fn check_env_vars(env: Env) -> Env {
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

/// Constraints a given number to bounds of min < x < max
fn within_range(x: u8, min: u8, max: u8) -> u8 {
    x % (max - min + 1) + min
}

/// Reversible fast randomization of a sequence of bytes.
#[allow(clippy::precedence)] // disable the false-positive ambiguous precedence
fn rand_bytes(x: u32, sh: u32, pepper: u32, gn: u32, xor: u32) -> u32 {
    ((gn + (x << sh) ^ x) ^ xor) ^ pepper
}

/// Reversible fast de-randomization of a sequence of bytes.
fn derand_bytes(x: u32, sh: u32, pepper: u32, gn: u32, xor: u32) -> u32 {
    rand_bytes(x ^ pepper, sh, pepper, gn, xor) ^ pepper
}

/// Checks, collects and formats the environment variables
/// in a format that's easy to use by `impass`.
pub fn collect_env(env: Env) -> Env {
    let env = self::check_env_vars(env);
    Env::from([
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
    ])
}

use base64::prelude::*;

/// Encrypts the given secret using the given env
/// parameters as controls.
/// `secret` - The string to encrypt. Supports only ASCII characters.
/// `env` - The environment variables used as a control.
/// # Examples:
/// ```rust
/// let env = collect_env(Env::new());
/// let crypt = encrypt_secret("Hello, world!", env);
/// assert_ne!(crypt, "Hello, world!");
/// ```
pub fn encrypt_secret(secret: &str, env: &Env) -> Vec<u8> {
    let shift: u32 = env[&"shift".to_string()].clone().parse().unwrap();
    let godnum: u32 = env[&"godnum".to_string()].clone().parse().unwrap();
    let xor: u32 = env[&"xor".to_string()].clone().parse().unwrap();

    let st = secret
        .as_bytes()
        .iter()
        .enumerate()
        .map(|(idx, ch)| {
            (self::rand_bytes(
                *ch as u32,
                self::within_range(shift as u8, 1, 31) as u32,
                self::within_range(idx as u8, 1, 8) as u32,
                godnum,
                xor,
            ) as u8
                ^ within_range(shift as u8, 7, 30)) as char
        })
        .collect::<String>();

    BASE64_STANDARD.encode(st).into()
}

/// Decrypts the given encrypted secret using the given env
/// parameters as controls.
/// `encrypted` - The encrypted secret to decrypt.  
/// `env` - The environment variables used as a control.  
/// # Examples:
/// ```rust
/// let env = collect_env(Env::new());
/// let crypt = encrypt_secret("Hello, world!", env);
/// let decrypt = dencrypt_secret(crypt, env);
/// assert_ne!(decrypt, "Hello, world!");
/// ```
pub fn decrypt_secret(encrypted: &[u8], env: &Env) -> String {
    let shift: u32 = env[&"shift".to_string()].clone().parse().unwrap();
    let godnum: u32 = env[&"godnum".to_string()].clone().parse().unwrap();
    let xor: u32 = env[&"xor".to_string()].clone().parse().unwrap();

    BASE64_STANDARD
        .decode(encrypted)
        .unwrap()
        .iter()
        .enumerate()
        .map(|(idx, ch)| {
            (self::derand_bytes(
                *ch as u32,
                self::within_range(shift as u8, 1, 31) as u32,
                self::within_range(idx as u8, 1, 8) as u32,
                godnum,
                xor,
            ) as u8
                ^ within_range(shift as u8, 7, 30)) as char
        })
        .rev()
        .collect()
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

    #[test]
    fn within_range_works() {
        let nums = [123, 53, 97, 45, 65];

        for n in nums {
            let res = super::within_range(n, 1, 10);

            assert!(res > 1 && res < 10);
        }
    }

    #[test]
    fn encrypt_decrypt_works() {
        let texts = [
            "hello world!",
            "how are youuuuu!?",
            "this is some text I'm boutta encode",
        ];

        let env = super::collect_env(super::Env::new());

        for txt in texts {
            let crypted = super::encrypt_secret(txt, &env)
                .iter()
                .map(|x| *x as char)
                .collect::<String>();

            assert_ne!(crypted, txt);

            let decrypted = super::decrypt_secret(crypted.as_bytes(), &env);
            assert_eq!(decrypted.chars().rev().collect::<String>(), txt);
        }
    }
}
