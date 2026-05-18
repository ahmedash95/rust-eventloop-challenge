# Step 1: Minimal Task Queue

Build the smallest possible event loop that can store callbacks and execute them later.

## What To Implement

- `EventLoop::new()`
- `event_loop.spawn(callback)`
- `event_loop.run()`
- A FIFO queue, likely backed by `VecDeque<Box<dyn FnMut()>>`

`run()` should repeatedly pop the next task and execute it until the queue is empty.

## Requirements

- Tasks execute in insertion order.
- `run()` exits when no tasks remain.
- Calling `run()` on an empty event loop is valid and should not panic.
- A task may enqueue more tasks if your API design supports sharing the event loop safely.

## Test-Oriented Examples

```rust
let mut output = Vec::new();
let mut event_loop = EventLoop::new();

event_loop.spawn(|| output.push("first"));
event_loop.spawn(|| output.push("second"));
event_loop.run();

assert_eq!(output, ["first", "second"]);
```

Use cases to test:

- Empty event loop exits immediately.
- Three tasks run in the same order they were spawned.
- A task that mutates captured state runs exactly once.
- Running the same event loop again after the queue is empty does nothing.
