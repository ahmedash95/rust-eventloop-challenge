# Step 7: Tiny Executor

Replace manual generator-style tasks with Rust futures and a tiny executor.

## What To Implement

- `event_loop.spawn(async { ... })`
- Storage for spawned futures.
- Polling of futures with `Future::poll`.
- A minimal `Context` and `Waker` strategy.
- Safe pinning for futures, such as `Pin<Box<dyn Future<Output = ()>>>`.

## Requirements

- Ready futures complete when polled.
- Pending futures remain stored for later polling.
- Completed futures are removed.
- The executor should not require Tokio or async-std.
- Earlier callback-based APIs may remain available.

## Test-Oriented Examples

```rust
let mut event_loop = EventLoop::new();

event_loop.spawn(async {
    println!("hello from async");
});

event_loop.run();
```

Use cases to test:

- A simple async block runs to completion.
- Spawning multiple ready async blocks runs all of them.
- A future returning `Poll::Pending` is not dropped prematurely.
- A future that becomes ready after being woken completes.
- Panics or errors have a clear policy if you choose to support them.
