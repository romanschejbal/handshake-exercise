mod bitcoin;

use bitcoin::{Address, Command, Message, Payload, VersionMessage};
use futures::{SinkExt, StreamExt};
use tokio::net::TcpStream;
use tokio_util::codec::Framed;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Connecting");

    let stream = TcpStream::connect("seed.bitcoin.sipa.be:8333").await?;
    let framed_stream = Framed::new(stream, bitcoin::BitcoinCodec).fuse();
    let (mut sink, mut stream) = framed_stream.split();

    println!("Connected");

    let version_message = VersionMessage {
        version: 70015,
        timestamp: 0,
        user_agent: "/ramen/".into(),
        addr_recv: Address {
            time: (),
            services: 0,
            ip: "::".parse().unwrap(),
            port: 0.into(),
        },
        addr_from: Address {
            time: (),
            services: 0,
            ip: "::".parse().unwrap(),
            port: 0.into(),
        },
        nonce: 0,
        services: 0,
        start_height: 0,
        relay: false,
    };

    let message = Message::new(
        0xD9B4BEF9,
        Command::Version,
        Payload::Version(version_message),
    );

    println!("Sending version message");
    sink.send(message).await?;
    loop {
        if let Some(response) = stream.next().await {
            let message = match response {
                Ok(message) => message,
                Err(e) => {
                    println!("Error: {}", e);
                    continue;
                }
            };

            match message.payload() {
                Payload::Version(version) => {
                    println!("version message received: {:?}", version);
                    let message = Message::new(0xD9B4BEF9, Command::VerAck, Payload::VerAck);
                    println!("Sending verack: {:?}", message);
                    sink.send(message).await?;
                }
                Payload::VerAck => {
                    println!("verack message received");
                }
                Payload::SendHeaders => {
                    println!("sendheaders received. Closing connection.");
                    break;
                }
            }
        } else {
            println!("Closed");
            break;
        }
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }

    Ok(())
}
