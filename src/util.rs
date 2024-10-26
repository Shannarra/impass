pub mod constants {
    pub static EOF_SIGNATURE: [u8; 12] = [0, 0, 0, 0, 73, 69, 78, 68, 174, 66, 96, 130];

    #[allow(dead_code)]
    pub const BCRYPT_COST: u32 = 15;
}

#[allow(dead_code, unused_variables)]
pub mod crypt {
    type Env = std::collections::HashMap<String, String>;

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

    fn within_range(x: u8, min: u8, max: u8) -> u8 {
        x % (max - min + 1) + min
    }

    #[allow(clippy::precedence)] // disable the false-positive ambiguous precedence
    fn rand_bytes(x: u32, sh: u32, pepper: u32, gn: u32, xor: u32) -> u32 {
        ((gn + (x << sh) ^ x) ^ xor) ^ pepper
    }

    fn derand_bytes(x: u32, sh: u32, pepper: u32, gn: u32, xor: u32) -> u32 {
        rand_bytes(x ^ pepper, sh, pepper, gn, xor) ^ pepper
    }

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

    pub fn decrypt_secret(encrypted: &[u8], len: usize, env: &Env) -> String {
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
}

pub fn index_vec<T>(haystack: &[T], needle: &[T]) -> Option<usize>
where
    T: Sized + Eq + PartialEq + std::fmt::Debug,
{
    for (position, window) in haystack.windows(needle.len()).enumerate() {
        if window == needle {
            return Some(position);
        }
    }
    None
}

pub fn prompt(message: &str) -> String {
    print!("{message}: ");
    use std::io;
    use std::io::*;
    let _ = std::io::stdout().flush();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

// Basically Jenkins Hash to work over bcrypt
#[allow(dead_code)]
pub fn impassible_hash<T>(item: &T) -> u128
where
    T: std::hash::Hash + std::string::ToString,
{
    let key = item.to_string();
    let mut hash = 0u128;
    for c in key.chars() {
        let tmp = c as u128;
        hash += tmp;
        hash += hash << 10;
        hash ^= hash >> 6;
    }
    hash += hash << 3;
    hash ^= hash >> 11;
    hash += hash << 15;
    hash
}
