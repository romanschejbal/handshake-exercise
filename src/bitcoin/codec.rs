pub use super::{Decode, Encode, Error, Message, Result};
use bytes::{Buf, BytesMut};
use tokio_util::codec::{Decoder, Encoder};

pub struct BitcoinCodec;

impl Encoder<Message> for BitcoinCodec {
    type Error = Error;

    fn encode(&mut self, item: Message, dst: &mut BytesMut) -> Result<()> {
        item.encode(dst)?;
        Ok(())
    }
}

impl Decoder for BitcoinCodec {
    type Item = Message;
    type Error = Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>> {
        if src.is_empty() {
            return Ok(None);
        }

        let Ok(msg) = Message::decode(src) else {
            println!("Failed to decode bytes: {:?}", src);
            return Ok(None);
        };
        if !src.has_remaining() {
            src.clear();
        }

        Ok(Some(msg))
    }
}
