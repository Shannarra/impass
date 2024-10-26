pub mod constants;
pub mod crypt;

/// Gives the index of a `needle` within a
/// given `haystack` if such exists.
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

/// Prompts the user for a string.
pub fn prompt(message: &str) -> String {
    print!("{message}: ");
    use std::io;
    use std::io::*;
    let _ = std::io::stdout().flush();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

/// Modified Jenkins Hash to work over bcrypt
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
