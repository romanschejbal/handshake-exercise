mod codec;
mod decode;
mod encode;
mod error;

pub use codec::*;
pub use decode::*;
pub use encode::*;
pub use error::*;

#[derive(Debug, Clone)]
pub enum Message {
    Auth { recipient_pubk: [u8; 128] },
}

impl Encode for Message {
    fn encode(&self, buf: &mut impl std::io::Write) -> Result<usize> {
        use secp256k1::rand::rngs::OsRng;
        use secp256k1::{Message as SecpMessage, Secp256k1};
        let secp = Secp256k1::new();
        let (secret_key, public_key) = secp.generate_keypair(&mut OsRng);

        pub fn write_auth(&mut self, buf: &mut BytesMut) {
            let unencrypted = self.create_auth_unencrypted();

            let mut out = buf.split_off(buf.len());
            out.put_u16(0);

            let mut encrypted = out.split_off(out.len());
            self.encrypt_message(&unencrypted, &mut encrypted);

            let len_bytes = u16::try_from(encrypted.len()).unwrap().to_be_bytes();
            out[..len_bytes.len()].copy_from_slice(&len_bytes);

            out.unsplit(encrypted);

            self.init_msg = Some(Bytes::copy_from_slice(&out));

            buf.unsplit(out);
        }
        // auth = auth-size || enc-auth-body;
        // auth-size = size of enc-auth-body, encoded as a big-endian 16-bit integer
        // auth-vsn = 4
        // auth-body = [sig, initiator-pubk, initiator-nonce, auth-vsn, ...]
        // enc-auth-body = ecies.encrypt(recipient-pubk, auth-body || auth-padding, auth-size)
        // auth-padding = arbitrary data
        Ok(0)
    }
}
