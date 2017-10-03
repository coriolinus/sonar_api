
use argon2rs;
use rand::{Rng, OsRng};
use std::fmt;

/// How long should the salt be.
///
/// Good practice is apparently to use the same number of random bytes as the
/// hasher outputs. However, for simplicity's sake, we're limited to the characters
/// `[a-zA-Z0-9]`, which reduces the entropy per byte from 256 to 62; roughly a quarter.
/// Therefore, we quadruple the salt's length in order to retain entropy.
const SALT_LENGTH: usize = argon2rs::defaults::LENGTH * 4;
/// How long is the password hash.
///
/// We just take this from the hashing library.
const HASH_LENGTH: usize = argon2rs::defaults::LENGTH;

/// Salted Password representation.
///
/// Use this to manage automatically salting and validating user passwords.
///
/// Generally, passwords are stored in the DB as a string of the form
/// `${method}${salt}${hash}$`. Salts are generated independently for each
/// password.
///
/// Possibly in the future, multiple methods will be allowed; for now, the set
/// of allowed hash methods is `{argon2}`.
///
/// This struct doesn't manage actually storing or retrieving anything from
/// a database or other storage method; it simply provides methods for creating,
/// parsing, and validating passwords which have been stringified in the proper format.
pub struct SaltyPassword {
    salt: String,
    hash: [u8; HASH_LENGTH],
}

impl SaltyPassword {
    /// Generate a salt and hash the supplied password with it.
    pub fn new(password: &str) -> SaltyPassword {
        let salt: String = OsRng::new()
            .expect("Failed to access OS RNG; aborting")
            .gen_ascii_chars()
            .take(SALT_LENGTH)
            .collect();
        SaltyPassword {
            hash: argon2rs::argon2i_simple(password, &salt),
            salt: salt,
        }
    }

    pub fn parse(mut field: &str) -> Option<SaltyPassword> {
        // trim off constant bits of the field
        let prefix = "$argon2$";
        if !(field.starts_with(prefix) && field.ends_with("$")) {
            return None;
        }
        field = &field[prefix.len()..(field.len() - 1)];

        // find and split at the dollar, to isolate the salt and hash
        let split_index = field.find('$')?;
        let (salt, mut hash_chars) = field.split_at(split_index);
        // hash_chars always begins with '$' right now
        hash_chars = &hash_chars[1..];

        // parse the hash
        if hash_chars.len() != HASH_LENGTH * 2 {
            return None;
        }
        let mut hash = [0; HASH_LENGTH];
        for index in 0..HASH_LENGTH {
            let begin = index * 2;
            let end = begin + 2;
            if !(hash_chars.is_char_boundary(begin) && hash_chars.is_char_boundary(end)) {
                return None;
            }
            hash[index] = u8::from_str_radix(&hash_chars[begin..end], 16).ok()?
        }

        Some(SaltyPassword {
            hash: hash,
            salt: salt.to_string(),
        })
    }

    /// Check whether a given password matches this one.
    ///
    /// Generally speaking, you'll want to create a SaltyPassword from the
    /// password field in the database, and then use that to validate your
    /// maybe password.
    pub fn validate(&self, password: &str) -> bool {
        self.hash == argon2rs::argon2i_simple(password, &self.salt)
    }
}

impl fmt::Display for SaltyPassword {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f, "${method}${salt}$",
            method = "argon2",
            salt = self.salt,
        )?;
        for byte in self.hash.iter() {
            write!(f, "{:x}", byte)?;
        }
        write!(f, "$")
    }
}
