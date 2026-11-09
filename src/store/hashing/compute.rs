use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

use blake3::Hasher;

use super::digest::Digest;
use super::error::HashingError;
use crate::store::models::Node;

pub fn hash_bytes(bytes : &[u8]) -> Digest { blake3::hash(bytes).into() }

pub fn hash_reader<R : Read>(mut reader : R) -> Result<Digest, HashingError> {
    let mut hasher : Hasher = blake3::Hasher::new();
    let mut buffer : [u8; 8192] = [0; 8192];

    loop {
        let count : usize = reader.read(&mut buffer).map_err(HashingError::Io)?;
        if count == 0 {
            break;
        }
        hasher.update(&buffer[..count]);
    }

    Ok(hasher.finalize().into())
}

pub fn hash_file<P : AsRef<Path>>(path : P) -> Result<Digest, HashingError> {
    hash_reader(File::open(path).map_err(HashingError::Io)?)
}

pub fn hash_node(node : &Node, lean_sha : &Digest, doc_sha : &Digest) -> Digest {
    hash_bytes(&node.canonical_bytes(lean_sha.as_str(), doc_sha.as_str()))
}
