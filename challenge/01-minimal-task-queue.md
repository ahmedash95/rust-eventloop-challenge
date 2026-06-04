# Step 1: Minimal Task Queue

Build the smallest possible event loop that can store callbacks and execute them later.

## What To Implement

- `EventLoop::new()`
- `event_loop.spawn(callback)`
- `event_loop.run()`
- A FIFO queue, likely backed by `VecDeque<Box<dyn FnOnce() + 'static>>`

`spawn` should require `F: FnOnce() + 'static` so tasks can be stored and run later without borrowing stack locals.

`run()` should repeatedly pop the next task and execute it until the queue is empty (e.g. `push_back` on spawn, `pop_front` in `run`).

## Requirements

- Tasks execute in insertion order.
- `run()` exits when no tasks remain.
- Calling `run()` on an empty event loop is valid and should not panic.
- A task may enqueue more tasks if your API design supports sharing the event loop safely (optional; not required for the basic test).

## Test-Oriented Examples

```rust
let mut event_loop = EventLoop::new();

event_loop.spawn(|| println!("first"));
event_loop.spawn(|| println!("second"));
event_loop.run();
```

Verify FIFO order from the printed lines (or capture stdout in a test).

Closures that only call `println!` (or otherwise own their data) are `'static` and match how production runtimes store work: self-contained tasks, not shared `&mut` borrows of stack variables.

If you need to assert shared mutable state across spawns, use owned handles (e.g. `Rc<Cell<_>>` or `Arc<Mutex<_>>` with `move` closures)—not `RefCell` around a local `Vec` with multiple `&mut` captures.

## Use cases to test

- Empty event loop exits immediately.
- Three tasks run in the same order they were spawned.
- Each task runs exactly once when dequeued.
- Running the same event loop again after the queue is empty does nothing.
