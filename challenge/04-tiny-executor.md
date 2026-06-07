# Step 4: Tiny Executor

**Builds on:** [Steps 1–3](README.md)  
**Difficulty:** medium

## In this stage

You'll run async blocks on your event loop — real Rust futures, no Tokio. The loop polls tasks until they complete. Callback APIs from steps 1–2 keep working alongside async tasks.

## What you'll implement

- Spawn async blocks on the event loop
- Poll in-flight futures each loop iteration
- A waker stub (real wiring comes in steps 5–7)

## Where to extend your code

- `src/event_loop.rs`
- `src/waker.rs` (optional)

## Concepts

| Piece | Role |
|---|---|
| `Future` | Work that may complete later |
| `Waker` | Tells the executor to poll again when something is ready |
| `Pin` | Keeps futures stable while polling |

Docs: [Future trait](https://doc.rust-lang.org/std/future/trait.Future.html)

## What to expect at the end

```rust
let mut event_loop = EventLoop::new();

event_loop.spawn(async {
    println!("hello from async");
});

event_loop.run();
```

Output:

```text
hello from async
```

Then `run()` returns. Multiple async blocks can finish in one `run()` call.

## Hints

- Don't drop futures that aren't finished yet.
- If callback and async spawn share a name, pick one scheme and stay consistent.

## Things to verify

- [ ] A simple async block runs to completion
- [ ] Multiple async blocks all complete
- [ ] Callback `spawn` and `set_timeout` still work
- [ ] `run()` exits when all futures and other work are done
