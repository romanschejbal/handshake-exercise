use crate::bitcoin::{Error, Result};
use bytes::BufMut;

pub trait Encode {
    fn encode(&self, buffer: &mut impl BufMut) -> Result<usize>;
}

macro_rules! make_encoder {
    ($t: ty, $fn: ident, $n: tt) => {
        impl Encode for $t {
            fn encode(&self, buffer: &mut impl BufMut) -> Result<usize> {
                if buffer.remaining_mut() < $n {
                    return Err(Error::NotEnoughSpace(stringify!($t)));
                }
                buffer.$fn(*self);
                Ok($n)
            }
        }
    };
}

make_encoder!(u8, put_u8, 1);
make_encoder!(u16, put_u16_le, 2);
make_encoder!(u32, put_u32_le, 4);
make_encoder!(u64, put_u64_le, 8);

make_encoder!(i16, put_i16_le, 2);
make_encoder!(i32, put_i32_le, 4);
make_encoder!(i64, put_i64_le, 8);

impl Encode for bool {
    fn encode(&self, buffer: &mut impl BufMut) -> Result<usize> {
        if buffer.remaining_mut() < 1 {
            return Err(Error::NotEnoughSpace("bool"));
        }
        buffer.put_u8(*self as u8);
        Ok(1)
    }
}

impl Encode for () {
    fn encode(&self, _buffer: &mut impl BufMut) -> Result<usize> {
        Ok(0)
    }
}

impl Encode for std::net::IpAddr {
    fn encode(&self, buffer: &mut impl BufMut) -> Result<usize> {
        if buffer.remaining_mut() < 16 {
            return Err(Error::NotEnoughSpace("IpAddr"));
        }
        use std::net::IpAddr::*;
        match self {
            V4(ip) => buffer.put_slice(&[[0; 4], [0; 4], [0; 4], ip.octets()].concat()),
            V6(ip) => buffer.put_slice(&ip.octets()),
        }
        Ok(16)
    }
}
