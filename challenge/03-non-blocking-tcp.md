# Step 3: Non-Blocking TCP

Introduce real IO without letting socket operations freeze the event loop.

## What To Implement

- A small TCP echo client or echo server using `std::net`.
- Sockets configured with `TcpStream::set_nonblocking(true)` or `TcpListener::set_nonblocking(true)`.
- Read and write loops that handle `WouldBlock`.
- Basic handling for partial reads and partial writes.

This step can be a standalone experiment before integrating sockets deeply into the event loop.

## Requirements

- Waiting for network data must not block all event loop progress.
- `WouldBlock` should mean "try again later", not "fail the operation".
- Partial reads and writes should preserve buffered data correctly.
- Socket errors should be surfaced or handled predictably.

## Test-Oriented Examples

```rust
let stream = TcpStream::connect(addr)?;
stream.set_nonblocking(true)?;

match stream.read(&mut buffer) {
    Ok(0) => println!("connection closed"),
    Ok(n) => println!("read {n} bytes"),
    Err(err) if err.kind() == ErrorKind::WouldBlock => println!("not ready yet"),
    Err(err) => return Err(err),
}
```

Use cases to test:

- Connecting to a local echo server and receiving the same bytes sent.
- Reading before data is available returns `WouldBlock` instead of blocking.
- Writing a large buffer handles partial writes.
- Closing the peer is detected as EOF or a connection error.
- A timer or queued task can still run while a socket is not ready.
