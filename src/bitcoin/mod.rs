mod codec;
mod decode;
mod encode;
mod error;
mod protocol;

pub use codec::*;
pub use decode::Decode;
pub use encode::Encode;
pub use error::{Error, Result};
pub use protocol::*;

pub trait Checksum {
    fn sha256(&self) -> u32;
}

impl Checksum for Vec<u8> {
    fn sha256(&self) -> u32 {
        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        hasher.update(self);
        let result = hasher.finalize();
        let mut hasher = Sha256::new();
        hasher.update(result);
        let result = hasher.finalize();
        let mut bytes = [0; 4];
        bytes.copy_from_slice(&result[0..4]);
        u32::from_le_bytes(bytes)
    }
}
