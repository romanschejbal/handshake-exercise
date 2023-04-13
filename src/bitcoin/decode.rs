use crate::bitcoin::Result;
use bytes::Buf;

pub trait Decode
where
    Self: Sized,
{
    fn decode(buffer: &mut impl Buf) -> Result<Self>;
}

impl Decode for () {
    fn decode(_buffer: &mut impl Buf) -> Result<Self> {
        Ok(())
    }
}

impl Decode for bool {
    fn decode(buffer: &mut impl Buf) -> Result<Self> {
        Ok(buffer.get_u8() != 0)
    }
}

macro_rules! make_decoder {
    ($t: ty, $fn: ident, $n: tt) => {
        impl Decode for $t {
            fn decode(buffer: &mut impl Buf) -> Result<Self> {
                if buffer.remaining() < $n {
                    return Err(crate::bitcoin::Error::NotEnoughBytes(stringify!($t)));
                }
                Ok(buffer.$fn())
            }
        }
    };
}

make_decoder!(u8, get_u8, 1);
make_decoder!(u16, get_u16_le, 2);
make_decoder!(u32, get_u32_le, 4);
make_decoder!(u64, get_u64_le, 8);
make_decoder!(i8, get_i8, 1);
make_decoder!(i16, get_i16_le, 2);
make_decoder!(i32, get_i32_le, 4);
make_decoder!(i64, get_i64_le, 8);

impl Decode for std::net::IpAddr {
    fn decode(buffer: &mut impl Buf) -> Result<Self> {
        if buffer.remaining() < 16 {
            return Err(crate::bitcoin::Error::NotEnoughBytes("IpAddr"));
        }
        let ip = TryInto::<[u8; 16]>::try_into(&buffer.copy_to_bytes(16)[..])?.into();
        Ok(ip)
    }
}
