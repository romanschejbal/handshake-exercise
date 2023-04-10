use crate::bitcoin::{Checksum, Decode, Encode, Error, Result};
use std::io::Write;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Message {
    magic: u32,
    command: Command,
    length: u32,
    checksum: u32,
    payload: Payload,
}

impl Message {
    pub fn new(magic: u32, command: Command, payload: Payload) -> Self {
        let mut encoded = Vec::new();
        let length = payload.encode(&mut encoded).unwrap() as u32;
        let checksum = encoded.sha256();
        Self {
            magic,
            command,
            length,
            checksum,
            payload,
        }
    }

    pub fn payload(&self) -> &Payload {
        &self.payload
    }

    pub fn len(&self) -> usize {
        4 + 12 + 4 + 4 + self.length as usize
    }
}

impl Encode for Message {
    fn encode(&self, buffer: &mut impl Write) -> Result<usize> {
        let mut written = self.magic.encode(buffer)?;
        written += self.command.encode(buffer)?;
        written += self.length.encode(buffer)?;
        written += self.checksum.encode(buffer)?;
        written += self.payload.encode(buffer)?;
        Ok(written)
    }
}

impl Decode for Message {
    fn decode(bytes: &[u8]) -> Result<(Self, &[u8])> {
        let (magic, bytes) = u32::decode(bytes)?;
        let (command, bytes) = Command::decode(bytes)?;
        let (length, bytes) = u32::decode(bytes)?;
        let (checksum, bytes) = u32::decode(bytes)?;
        let (payload, bytes) = Payload::decode_command(command.clone(), bytes)?;
        Ok((
            Message {
                magic,
                command,
                length,
                checksum,
                payload,
            },
            bytes,
        ))
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Command {
    Version,
    VerAck,
    SendHeaders,
    #[allow(dead_code)]
    SendCmpct,
}

impl Encode for Command {
    fn encode(&self, buffer: &mut impl Write) -> Result<usize> {
        match self {
            Self::Version => buffer.write_all(b"version\0\0\0\0\0")?,
            Self::VerAck => buffer.write_all(b"verack\0\0\0\0\0\0")?,
            Self::SendHeaders => buffer.write_all(b"sendheaders\0")?,
            Self::SendCmpct => return Err(Error::Command("unimplemented".to_string())),
        };
        Ok(12)
    }
}

impl Decode for Command {
    fn decode(bytes: &[u8]) -> Result<(Self, &[u8])> {
        match &bytes[..12] {
            b"version\0\0\0\0\0" => Ok((Command::Version, &bytes[12..])),
            b"verack\0\0\0\0\0\0" => Ok((Command::VerAck, &bytes[12..])),
            b"sendheaders\0" => Ok((Command::SendHeaders, &bytes[12..])),
            x => Err(Error::Command(format!(
                "unhandled command: {:?}",
                String::from_utf8_lossy(x)
            ))),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Payload {
    Version(VersionMessage),
    VerAck,
    SendHeaders,
}

impl Payload {
    fn decode_command(command: Command, bytes: &[u8]) -> Result<(Self, &[u8])> {
        match command {
            Command::Version => {
                let (version, bytes) = VersionMessage::decode(bytes)?;
                Ok((Payload::Version(version), bytes))
            }
            Command::VerAck => Ok((Payload::VerAck, bytes)),
            Command::SendHeaders => Ok((Payload::SendHeaders, bytes)),
            Command::SendCmpct => Ok((Payload::SendHeaders, bytes)),
        }
    }
}

impl Encode for Payload {
    fn encode(&self, buffer: &mut impl Write) -> Result<usize> {
        match self {
            Self::Version(version) => version.encode(buffer),
            Self::VerAck => ().encode(buffer),
            Self::SendHeaders => ().encode(buffer),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct VersionMessage {
    pub version: i32,
    pub services: u64,
    pub timestamp: i64,
    pub addr_recv: Address<()>,
    pub addr_from: Address<()>,
    pub nonce: u64,
    pub user_agent: VariableLengthString,
    pub start_height: i32,
    pub relay: bool,
}

impl Encode for VersionMessage {
    fn encode(&self, buffer: &mut impl Write) -> Result<usize> {
        let mut written = self.version.encode(buffer)?;
        written += self.services.encode(buffer)?;
        written += self.timestamp.encode(buffer)?;
        written += self.addr_recv.encode(buffer)?;
        written += self.addr_from.encode(buffer)?;
        written += self.nonce.encode(buffer)?;
        written += self.user_agent.encode(buffer)?;
        written += self.start_height.encode(buffer)?;
        written += self.relay.encode(buffer)?;
        Ok(written)
    }
}

impl Decode for VersionMessage {
    fn decode(bytes: &[u8]) -> Result<(Self, &[u8])> {
        let (version, bytes) = i32::decode(bytes)?;
        let (services, bytes) = u64::decode(bytes)?;
        let (timestamp, bytes) = i64::decode(bytes)?;
        let (addr_recv, bytes) = Address::decode(bytes)?;
        let (addr_from, bytes) = Address::decode(bytes)?;
        let (nonce, bytes) = u64::decode(bytes)?;
        let (user_agent, bytes) = VariableLengthString::decode(bytes)?;
        let (start_height, bytes) = i32::decode(bytes)?;
        let (relay, bytes) = bool::decode(bytes)?;
        Ok((
            VersionMessage {
                version,
                services,
                timestamp,
                addr_recv,
                addr_from,
                nonce,
                user_agent,
                start_height,
                relay,
            },
            bytes,
        ))
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Address<T> {
    pub time: T,
    pub services: u64,
    pub ip: std::net::IpAddr,
    pub port: Port,
}

impl<T> Encode for Address<T>
where
    T: Encode,
{
    fn encode(&self, buffer: &mut impl Write) -> Result<usize> {
        let mut bytes = 0;
        bytes += self.time.encode(buffer)?;
        bytes += self.services.encode(buffer)?;
        bytes += self.ip.encode(buffer)?;
        bytes += self.port.encode(buffer)?;
        Ok(bytes)
    }
}

impl<T> Decode for Address<T>
where
    T: Decode,
{
    fn decode(bytes: &[u8]) -> Result<(Self, &[u8])> {
        let (time, bytes) = T::decode(bytes)?;
        let (services, bytes) = u64::decode(bytes)?;
        let (ip, bytes) = std::net::IpAddr::decode(bytes)?;
        let (port, bytes) = Port::decode(bytes)?;
        Ok((
            Address {
                time,
                services,
                ip,
                port,
            },
            bytes,
        ))
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Port(u16);

impl From<u16> for Port {
    fn from(port: u16) -> Self {
        Self(port)
    }
}

impl Encode for Port {
    fn encode(&self, buffer: &mut impl Write) -> Result<usize> {
        buffer.write_all(&self.0.to_be_bytes())?;
        Ok(2)
    }
}

impl Decode for Port {
    fn decode(bytes: &[u8]) -> Result<(Self, &[u8])> {
        Ok((
            Self(u16::from_be_bytes(bytes[..2].try_into()?)),
            &bytes[2..],
        ))
    }
}

#[derive(Debug, Default, Clone, Eq, PartialEq)]
struct VariableInt(u64);

impl Encode for VariableInt {
    fn encode(&self, buffer: &mut impl Write) -> Result<usize> {
        match self.0 {
            0..=0xFC => {
                buffer.write_all(&[self.0 as u8])?;
                Ok(1)
            }
            0xFD..=0xFFFF => {
                buffer.write_all(&[0xFD])?;
                Ok(1 + (self.0 as u16).encode(buffer)?)
            }
            0x10000..=0xFFFFFFFF => {
                buffer.write_all(&[0xFE])?;
                Ok(1 + (self.0 as u32).encode(buffer)?)
            }
            _ => {
                buffer.write_all(&[0xFE])?;
                self.0.encode(buffer)?;
                Ok(1 + self.0.encode(buffer)?)
            }
        }
    }
}

impl Decode for VariableInt {
    fn decode(bytes: &[u8]) -> Result<(Self, &[u8])> {
        match bytes[0] {
            0xFD => {
                let (number, bytes) = u16::decode(&bytes[1..])?;
                Ok((VariableInt(number as u64), bytes))
            }
            0xFE => {
                let (number, bytes) = u32::decode(&bytes[1..])?;
                Ok((VariableInt(number as u64), bytes))
            }
            0xFF => {
                let (number, bytes) = u64::decode(&bytes[1..])?;
                Ok((VariableInt(number), bytes))
            }
            _ => Ok((VariableInt(bytes[0] as u64), &bytes[1..])),
        }
    }
}

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct VariableLengthString(VariableInt, String);

impl From<&str> for VariableLengthString {
    fn from(s: &str) -> Self {
        VariableLengthString(VariableInt(s.len() as u64), s.to_string())
    }
}

impl Encode for VariableLengthString {
    fn encode(&self, buffer: &mut impl Write) -> Result<usize> {
        let written = self.0.encode(buffer)?;
        buffer.write_all(self.1.as_bytes())?;
        Ok(written + self.1.len())
    }
}

impl Decode for VariableLengthString {
    fn decode(bytes: &[u8]) -> Result<(Self, &[u8])> {
        let (length, bytes) = VariableInt::decode(bytes)?;
        let str = String::from_utf8_lossy(&bytes[..(length.0 as usize)]).into_owned();
        let bytes = &bytes[(length.0 as usize)..];
        Ok((Self(length, str), bytes))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn decode() {
        let message_bin = b"\xf9\xbe\xb4\xd9version\0\0\0\0\0f\0\0\0@e\xe2A\x80\x11\x01\0\t\x04\0\0\0\0\0\0\x0e\xb1$d\0\0\0\0\0\0\0\0\0\0\0\0*\x02\x83\x08\x90\x0cY\0\xb5\x9b\xb5Q\x1c&\x02\xa8\xdb~\t\x04\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0SH\x1f\xe5\xdc6S`\x10/Satoshi:23.0.0/\xe8\xf2\x0b\0\x01\xf9\xbe\xb4\xd9verack\0\0\0\0\0\0\0\0\0\0]\xf6\xe0\xe2";
        let (message, _) = Message::decode(&mut &message_bin[..]).unwrap();
        assert_eq!(
            message,
            Message {
                magic: 3652501241,
                command: Command::Version,
                length: 102,
                checksum: 1105356096,
                payload: Payload::Version(VersionMessage {
                    version: 70016,
                    services: 1033,
                    timestamp: 1680126222,
                    addr_recv: Address {
                        time: (),
                        services: 0,
                        ip: "2a02:8308:900c:5900:b59b:b551:1c26:2a8".parse().unwrap(),
                        port: Port(56190),
                    },
                    addr_from: Address {
                        time: (),
                        services: 1033,
                        ip: "::".parse().unwrap(),
                        port: Port(0),
                    },
                    nonce: 6940951773072803923,
                    user_agent: "/Satoshi:23.0.0/".into(),
                    start_height: 783080,
                    relay: true,
                })
            }
        );
    }

    #[test]
    fn encode_decode() {
        let msg = Message {
            magic: 3652501241,
            command: Command::Version,
            length: 102,
            checksum: 1105356096,
            payload: Payload::Version(VersionMessage {
                version: 70016,
                services: 1033,
                timestamp: 1680126222,
                addr_recv: Address {
                    time: (),
                    services: 0,
                    ip: "2a02:8308:900c:5900:b59b:b551:1c26:2a8".parse().unwrap(),
                    port: Port(56190),
                },
                addr_from: Address {
                    time: (),
                    services: 1033,
                    ip: "::192.168.0.1".parse().unwrap(),
                    port: Port(0),
                },
                nonce: 6920951773072803923,
                user_agent: "/Satoshi:23.0.0:Nakamoto//Satoshi:23.0.0:Nakamoto//Satoshi:23.0.0:Nakamoto//Satoshi:23.0.0:Nakamoto//Satoshi:23.0.0:Nakamoto//Satoshi:23.0.0:Nakamoto//Satoshi:23.0.0:Nakamoto//Satoshi:23.0.0:Nakamoto//Satoshi:23.0.0:Nakamoto//Satoshi:23.0.0:Nakamoto//Satoshi:23.0.0:Nakamoto//Satoshi:23.0.0:Nakamoto//Satoshi:23.0.0:Nakamoto//Satoshi:23.0.0:Nakamoto//Satoshi:23.0.0:Nakamoto//Satoshi:23.0.0:Nakamoto//Satoshi:23.0.0:Nakamoto//Satoshi:23.0.0:Nakamoto//Satoshi:23.0.0:Nakamoto//Satoshi:23.0.0:Nakamoto//Satoshi:23.0.0:Nakamoto//Satoshi:23.0.0:Nakamoto//Satoshi:23.0.0:Nakamoto//Satoshi:23.0.0:Nakamoto//Satoshi:23.0.0:Nakamoto//Satoshi:23.0.0:Nakamoto//Satoshi:23.0.0:Nakamoto//Satoshi:23.0.0:Nakamoto//Satoshi:23.0.0:Nakamoto/".into(),
                start_height: 1932515342,
                relay: true,
            }),
        };

        let mut buf = vec![];

        let encoded = msg.encode(&mut buf).unwrap();

        let decoded = Message::decode(&buf[..encoded]).unwrap().0;

        assert_eq!(decoded, msg);
    }
}
