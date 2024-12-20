pub mod constants;
pub mod crypt;
pub mod env;

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

/// Modified Jenkins Hash to work in conjunction
/// with bcrypt. Allows for 11-characters max ASCII
/// only string.
pub fn impassible_hash(item: &String) -> u128 {
    // Allow ASCII-only characters, so that even passwords
    // like "Pa$_swOrd" work :)
    if !item.chars().all(|c| char::is_ascii(&c)) {
        crate::error!(
            "Password provided contains invalid characters. Please, use ASCII-only characters!"
        );
    }

    if item.len() > 11 {
        // 11 so it can bother you hehe
        crate::error!("Maximum password length is 11.");
    }

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

/// Constraints a given number to bounds of min < x < max
fn within_range(x: u8, min: u8, max: u8) -> u8 {
    x % (max - min + 1) + min
}

/// A very simple "random" number generation constraint to
/// the bounds of u8 (0-255)
fn rand() -> u8 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .subsec_nanos() as u8
}

pub type Env = std::collections::HashMap<String, String>;

mod test {
    #[test]
    fn within_range_works() {
        let nums = [123, 53, 97, 45, 65];

        for n in nums {
            let res = super::within_range(n, 1, 10);

            assert!(res > 1 && res < 10);
        }
    }

    #[test]
    fn index_vec_works() {
        let nums = [1, 2, 3, 4, 5, 6];
        let target = [3, 4, 5];

        let idx = super::index_vec(&nums, &target);

        assert_eq!(idx, Some(2));

        let idx = super::index_vec(&nums, &[4, 3]);

        assert_eq!(idx, None);
    }

    #[test]
    fn impassible_works_as_expected() {
        let items = vec!["helloworld!", "Pa$_swOrd"];

        for it in items {
            assert!(super::impassible_hash(&it.to_string()) > 100000);
        }
    }

    #[test]
    #[should_panic(expected = "Maximum password length is 11.")]
    fn impassible_has_strict_length() {
        let item = "my password is tooo long";

        super::impassible_hash(&item.to_string());
    }

    #[test]
    #[should_panic(
        expected = "Password provided contains invalid characters. Please, use ASCII-only characters!"
    )]
    fn impassible_has_strict_contents() {
        let item = "невалиден"; // "invalid" in bulgarian

        super::impassible_hash(&item.to_string());
    }
}
