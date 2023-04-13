use super::{Encode, Message};
use bytes::BufMut;
use tokio_util::codec::{Decoder, Encoder};

pub struct EthereumCodec;

impl Encoder<Message> for EthereumCodec {
    type Error = super::Error;

    fn encode(&mut self, item: Message, dst: &mut bytes::BytesMut) -> Result<(), Self::Error> {
        let mut writer = dst.writer();
        item.encode(&mut writer)?;
        Ok(())
    }
}

impl Decoder for EthereumCodec {
    type Item = Message;

    type Error = super::Error;

    fn decode(&mut self, src: &mut bytes::BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        println!("Decoding Ethereum message: {src:?}");
        Ok(None)
    }
}
