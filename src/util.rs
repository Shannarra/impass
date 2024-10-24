pub mod constants {
    pub static EOF_SIGNATURE: [u8; 12] = [0, 0, 0, 0, 73, 69, 78, 68, 174, 66, 96, 130];

    #[allow(dead_code)]
    pub const BCRYPT_COST: u32 = 15;
}

#[allow(dead_code, unused_variables)]
pub mod crypt {
    type Env = std::collections::HashMap<String, String>;

    fn check_env_vars(env: Env) -> Env {
        if let Some(shr) = env.get("RSHIFT") {
            let val = shr.parse::<u32>();
            if val.is_err() {
                crate::error!("Number provided for RSHIFT must be a positive whole number!")
            }
        }

        if let Some(shl) = env.get("LSHIFT") {
            let val = shl.parse::<u32>();
            if val.is_err() {
                crate::error!("Number provided for LSHIFT must be a positive whole number!")
            }
        }

        env
    }

    pub fn collect_env(env: Env) -> Env {
        let env = self::check_env_vars(env);
        Env::from([
            (
                "rshift".to_string(),
                env.get("RSHIFT").unwrap_or(&"15".to_string()).to_owned(),
            ),
            (
                "lshift".to_string(),
                env.get("LSHIFT").unwrap_or(&"11".to_string()).to_owned(),
            ),
        ])
    }

    pub fn encrypt_secret(secret: &str, env: &Env) -> Vec<u8> {
        /*let shr: i32 = env["rshift"].parse::<i32>().unwrap();
              let shl: i32 = env["lshift"].parse::<i32>().unwrap();
        */
        println!("{}", (secret.chars().next().unwrap() as u32) << 5);
        secret
            .chars()
            .enumerate()
            .map(|(idx, x)| (x as u32))
            .map(|x| x as u8)
            .collect::<Vec<u8>>()
    }

    pub fn decrypt_secret(encrypted: &[u8], len: usize, env: &Env) -> String {
        /*       let shr: usize = env["rshift"].parse::<usize>().unwrap();
               let shl: usize = env["lshift"].parse::<usize>().unwrap();
        */
        println!("{encrypted:?}");
        let text: String = encrypted
            .iter()
            //.enumerate()
            //.map(|(idx, x)| (*x as u8))
            .map(|x| *x as char)
            .rev()
            .collect();

        text
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
