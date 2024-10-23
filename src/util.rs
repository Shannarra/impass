pub mod constants {
    pub static EOF_SIGNATURE: [u8; 12] = [0, 0, 0, 0, 73, 69, 78, 68, 174, 66, 96, 130];

    #[allow(dead_code)]
    pub const BCRYPT_COST: u32 = 15;
}

#[allow(dead_code, unused_variables)]
pub mod crypt {
    pub fn encrypt_secret(secret: &str) -> Vec<u8> {
        vec![]
    }

    pub fn decrypt_secret(encrypted: &[u8]) -> String {
        encrypted.iter().map(|x| *x as char).collect()
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
