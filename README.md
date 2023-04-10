# Testing

Running `cargo r` should output the following:

```
Connecting
Connected
Sending version message
version message received: VersionMessage { version: 70016, services: 1033, timestamp: 1681128367, addr_recv: Address { time: (), services: 0, ip: 2a02:8308:900c:5900:8d5:e301:b26a:7b60, port: Port(61854) }, addr_from: Address { time: (), services: 1033, ip: ::, port: Port(0) }, nonce: 17096882618434121085, user_agent: VariableLengthString(VariableInt(16), "/Satoshi:23.0.0/"), start_height: 784777, relay: true }
Sending verack: Message { magic: 3652501241, command: VerAck, length: 0, checksum: 3806393949, payload: VerAck }
verack message received
sendheaders received. Closing connection.
```

When `sendheaders` is received, the handshake can be considered done and connection established.
