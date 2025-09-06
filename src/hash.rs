use crate::fs::File;
use core::fmt;
use sha3::digest::{ExtendableOutput, Update};
use std::io::Read;

const HASH_BYTES: usize = 6;
type Hasher = sha3::TurboShake128;

#[derive(Debug, Clone, Copy)]
pub struct ShortHash([u8; 6]);

impl ShortHash {
    pub fn from_file(mut file: &File) -> Self {
        let mut hasher = {
            let core = sha3::TurboShake128Core::new(HASH_BYTES as u8);
            Hasher::from_core(core)
        };

        {
            let mut buf = Vec::new();
            file.read_to_end(&mut buf).expect("Couldn't read from file");
            hasher.update(buf.as_ref());
        }

        let mut hash = [0_u8; HASH_BYTES];

        {
            let mut reader = hasher.finalize_xof();
            reader.read_exact(hash.as_mut()).unwrap();
        }

        Self(hash)
    }
}

impl fmt::Display for ShortHash {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let hex = const_hex::display(self.0);
        write!(f, "{}", hex)
    }
}

impl serde::Serialize for ShortHash {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        if serializer.is_human_readable() {
            use const_hex::ToHexExt;
            serializer.serialize_str(self.0.as_ref().encode_hex().as_ref())
        } else {
            serializer.serialize_bytes(self.0.as_ref())
        }
    }
}
