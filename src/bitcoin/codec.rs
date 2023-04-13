pub use super::{Decode, Encode, Error, Message, Result};
use bytes::{Buf, BufMut, BytesMut};
use tokio_util::codec::{Decoder, Encoder};

pub struct BitcoinCodec;

impl Encoder<Message> for BitcoinCodec {
    type Error = Error;

    fn encode(&mut self, item: Message, dst: &mut BytesMut) -> Result<()> {
        item.encode(&mut dst.writer()).unwrap();
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

        let msg = Message::decode(src)?;

        if msg.len() == src.len() {
            src.clear();
        } else if src.has_remaining() {
            src.advance(msg.len());
        }

        Ok(Some(msg))
    }
}
