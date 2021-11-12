// TODO Delete this file.

//! Module implementing some improvements to the more generic but less specialised official
//! multiformat crates.

use anyhow;
use cid::multihash::Multihash;

/// Blake3 multicodec code.
const BLAKE3_HASH_FUNC_TYPE: u8 = 0x1e;

/// Create a Multihash form a 32-byte blake3 digest.
/// [hash] should should be the output of the Blake3 algorithm although obviously no verification
/// can be performed.
///
/// # Rationale
///
/// This function facilitates the creation of a proper Multihash with custom Blake3 implementations.
pub fn blake3_hash_to_multihash(hash: [u8; 32]) -> anyhow::Result<Multihash> {
    let mut multihash_bytes = vec![BLAKE3_HASH_FUNC_TYPE, hash.len() as u8];
    multihash_bytes.extend_from_slice(hash.as_ref());
    let multihash = Multihash::from_bytes(multihash_bytes.as_slice());
    match multihash {
        Ok(mh) => Ok(mh),
        Err(_) => Err(anyhow::anyhow!(
            "failed to create multihash from blake3 hash"
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_get_multihash_from_blake3_hash() {
        let hash = [0x42 as u8; 32];
        let multihash = blake3_hash_to_multihash(hash.clone()).unwrap();
        assert_eq!(multihash.code(), BLAKE3_HASH_FUNC_TYPE as u64);
        assert_eq!(multihash.size(), 0x20);
        assert_eq!(*multihash.digest(), hash[..]);
    }
}
