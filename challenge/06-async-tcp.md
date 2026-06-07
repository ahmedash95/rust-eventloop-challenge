# Step 6: Async TCP I/O

**Builds on:** [Step 3 — Reactor foundation](03-reactor-foundation.md), [Steps 4–5](README.md)  
**Difficulty:** medium

## In this stage

You'll add async TCP — connect, read, and write as futures on top of the step 3 reactor. Sockets that aren't ready pause the task until the reactor wakes them.

## What you'll implement

- Async connect, write, and read over TCP
- Handle partial writes, EOF, and connection errors
- Register and deregister sockets with the reactor

## Where to extend your code

- `src/tcp.rs` (new file)
- `src/event_loop.rs`

## Concepts

| Concept | What it means here |
|---|---|
| Partial I/O | A write may send only part of a buffer; retry the rest |
| Non-blocking connect | Connecting may not finish immediately |

## What to expect at the end

Run a local echo server: `nc -l 8080`

```rust
let mut event_loop = EventLoop::new();

event_loop.spawn(async {
    let mut stream = TcpStream::connect("127.0.0.1:8080").await.unwrap();
    stream.write_all(b"hello").await.unwrap();
    let data = stream.read_to_end().await.unwrap();
    assert_eq!(data, b"hello");
});

event_loop.set_timeout(Duration::from_millis(5), || println!("timer still works"));
event_loop.run();
```

Echo round-trip succeeds. `"timer still works"` may print while waiting. `run()` exits when done.

## Hints

- Don't block inside poll — save progress and try again when woken.
- Non-blocking connect is a common source of "hangs forever" bugs.

## Things to verify

- [ ] Echo round-trip completes through `event_loop.run()`
- [ ] A timer fires while a socket is waiting
- [ ] Two concurrent connections both complete
- [ ] Partial writes retry until the full buffer is sent
- [ ] Peer close is detected (EOF or error)
