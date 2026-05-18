# Step 4: Polling IO Event Loop

Make the event loop wait efficiently for socket readiness using OS polling through `mio`.

## What To Implement

- A `mio::Poll` instance inside the event loop.
- Registration of sockets with tokens and interests such as readable or writable.
- A mapping from tokens to callbacks or handlers.
- Event dispatch from `poll.poll(&mut events, timeout)` back into event loop work.

## Requirements

- Multiple sockets can be registered at the same time.
- Callbacks fire only when the related socket is ready.
- The poll timeout should account for pending timers.
- The event loop should sleep efficiently when no immediate work is available.
- Existing task queue and timer behavior should continue working.

## Test-Oriented Examples

```rust
event_loop.watch_readable(socket, || {
    println!("socket is readable");
});

event_loop.run();
```

Use cases to test:

- Registering two sockets and receiving readiness for each independently.
- A readable callback is not called before data is available.
- A timer still fires while the event loop is also polling sockets.
- Event loop exits or can be stopped cleanly when no watchers remain.
- Re-registering or deregistering a socket does not leave stale callbacks behind.
