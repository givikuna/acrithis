use std::fmt;

use super::error::HashingError;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Digest(String);

impl Digest {
    pub fn from_hex(hex : String) -> Result<Self, HashingError> {
        if hex.len() != 64 {
            return Err(HashingError::InvalidLength(hex.len()));
        }

        if !hex.bytes()
               .all(|b : u8| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
        {
            return Err(HashingError::InvalidHex);
        }

        Ok(Self(hex))
    }

    pub fn as_str(&self) -> &str { &self.0 }
}

impl fmt::Display for Digest {
    fn fmt(&self, f : &mut fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{}", self.0)
    }
}

impl From<blake3::Hash> for Digest {
    fn from(hash : blake3::Hash) -> Self { Self(hash.to_hex().to_string()) }
}
