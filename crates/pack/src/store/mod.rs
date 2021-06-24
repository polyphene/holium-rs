//! Provides all required material to store HoliumPacks as files.
//!
//! TODO talk about CIDs, distribution, IPFS

use std::collections::HashMap;
use std::io::Write;

use blake3::hash;
use flate2::Compression;
use flate2::write::ZlibEncoder;

use crate::HoliumPack;

/// Zlib compression level used in the process for deterministically computing objects' CIDs.
static ZLIB_COMPRESSION: Compression = Compression::new(6);

/// Type alias for Blake3-256 digests used to identify various resources.
type HoliumFragmentCID = [u8; 32];

/// Fragmented representation of an HoliumPack object.
pub struct FragmentDecomposition {
    /// The CID of the represented pack.
    entrypoint: HoliumFragmentCID,
    /// A map from all CIDs to individual fragments representing the holium pack.
    fragments: HashMap<HoliumFragmentCID, Vec<u8>>,
}

/// Computed the CID of any binarized fragment.
fn compute_cid(buf: &[u8]) -> HoliumFragmentCID {
    let mut zlib_encoder = ZlibEncoder::new(Vec::new(), ZLIB_COMPRESSION);
    zlib_encoder.write_all(buf).unwrap();
    let compressed_bytes = zlib_encoder.finish().unwrap();
    let cid = hash(&*compressed_bytes);
    return *cid.as_bytes();
}

impl FragmentDecomposition {
    /// Recursively turns a holium pack into its fragmented representation.
    pub fn from_holium_pack(p: HoliumPack) -> FragmentDecomposition {
        return match p {
            HoliumPack::Primitive(b) => {
                let cid = compute_cid(b.as_slice());
                let mut fragments = HashMap::new();
                fragments.insert(cid, b);
                FragmentDecomposition {
                    entrypoint: cid,
                    fragments,
                }
            }
            HoliumPack::Array(_) => {
                todo!();
            }
        };
    }

    /// Get the CID of an holium pack from its fragment decomposition.
    pub fn cid(&self) -> &HoliumFragmentCID {
        &self.entrypoint
    }

    /// Get the hashmap of fragments making an holium pack decomposition.
    pub fn fragments(&self) -> &HashMap<HoliumFragmentCID, Vec<u8>> {
        &self.fragments
    }
}

#[cfg(test)]
mod tests {
    use crate::store::compute_cid;

    #[test]
    fn cids_are_32_byte_long() {
        let v = vec![];
        let cid = compute_cid(&v);
        assert_eq!(32, cid.len());
    }

    #[test]
    fn cids_are_deterministic() {
        //
        let expected_cids: Vec<Vec<u8>> = vec![
            "7774fa189c73a445ff605c3d44afdc3a7799efe587de0ebf94d73b15bbb75aeb",
            "8d197d2ca2813d0d3c81b808e8472289c620784ff2fc206ee7bda585e412b461",
            "ffc54fd620c41cd1d07790d01f9c1c599d0321a4c7ba1d3cc2ac2ff4fdae2bc7",
            "85689442443219bd53eddd3e485233f20c861669eed1a133b97d6a7c11bee8c2",
            "1db44c0980bd6448f0f37a4327762c21db6332c6232e0d9edd083d55c4426661",
            "60777ba08c128ddaa85bbae839097db206f21ba8cfe15bdf2c67af70f4a25996",
            "57da2285ac73dfed2b1eb076fc0a70f7961d06e57e2e78e04dbfbe45526aad0f",
            "d5ff3e64052a15c6de266a96a6d26d5d5386e0dcc554d942e47ddebb00d63e43",
            "1e1aee2425d9a5513493784c1ba75a366ee7bf013a4fa40f68cfcbbdec4409e8",
            "bf061bdee6abbbb9f041942f9747b01485aac7391a9908aa5727649c623eb121",
        ]
            .iter()
            .map(|s| hex::decode(s).unwrap())
            .collect();
        //
        for (i, digest) in expected_cids.iter().enumerate() {
            let v = vec![0x00; i.into()];
            let cid = compute_cid(&v);
            assert_eq!(&cid.to_vec(), digest);
        }
    }
}