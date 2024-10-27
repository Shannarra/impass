/// Reversible fast randomization of a sequence of bytes.
#[allow(clippy::precedence)] // disable the false-positive ambiguous precedence
fn rand_bytes(x: u32, sh: u32, pepper: u32, gn: u32, xor: u32) -> u32 {
    ((gn + (x << sh) ^ x) ^ xor) ^ pepper
}

/// Reversible fast de-randomization of a sequence of bytes.
fn derand_bytes(x: u32, sh: u32, pepper: u32, gn: u32, xor: u32) -> u32 {
    rand_bytes(x ^ pepper, sh, pepper, gn, xor) ^ pepper
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
pub fn encrypt_secret(secret: &str, env: &super::Env) -> Vec<u8> {
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
                super::within_range(shift as u8, 1, 31) as u32,
                super::within_range(idx as u8, 1, 8) as u32,
                godnum,
                xor,
            ) as u8
                ^ super::within_range(shift as u8, 7, 30)) as char
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
pub fn decrypt_secret(encrypted: &[u8], env: &super::Env) -> String {
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
                super::within_range(shift as u8, 1, 31) as u32,
                super::within_range(idx as u8, 1, 8) as u32,
                godnum,
                xor,
            ) as u8
                ^ super::within_range(shift as u8, 7, 30)) as char
        })
        .rev()
        .collect()
}

mod test {
    #[test]
    fn within_range_works() {
        let nums = [123, 53, 97, 45, 65];

        for n in nums {
            let res = super::super::within_range(n, 1, 10);

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

        let env = super::super::env::collect_env(super::super::Env::new());

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
