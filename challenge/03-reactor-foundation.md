# Step 3: Reactor Foundation

**Builds on:** [Step 1 — Minimal task queue](01-minimal-task-queue.md), [Step 2 — Timers](02-timers.md)  
**Difficulty:** medium

## In this stage

You'll add the **reactor** — the I/O readiness layer that Tokio uses internally via `mio`. When nothing is ready, the loop blocks in the reactor instead of `thread::sleep`, but timers still fire on schedule.

This step is **plumbing only**. No public TCP API yet — that comes in step 6. Focus on unified wait: one place where the thread sleeps until a timer expires or a socket becomes ready.

## What you'll implement

- Add `mio` as a dependency
- A single unified wait path when the loop is idle
- Step 1–2 APIs and behavior unchanged

## Where to extend your code

- `Cargo.toml`
- `src/event_loop.rs`

## Concepts

| Concept | What it means here |
|---|---|
| Reactor | Block until something is ready or a timeout expires |
| Readiness | The OS notifies you when a socket can be read/written without blocking |

Docs: [mio guide](https://docs.rs/mio/latest/mio/)

## What to expect at the end

Same API as step 2 — no TCP yet:

```rust
let mut event_loop = EventLoop::new();

event_loop.spawn(|| println!("immediate"));
event_loop.set_timeout(Duration::from_millis(50), || println!("50ms"));
event_loop.set_timeout(Duration::from_millis(10), || println!("10ms"));

event_loop.run();
```

Output:

```text
immediate
10ms
50ms
```

The difference is internal: while idle, the loop waits through the reactor instead of `thread::sleep`. CPU stays low; no busy-spin.

## Hints

- Keep the reactor internal — step 6 adds the public async TCP API on top.
- Step 2's timer logic should carry over; only the wait mechanism changes.

## Things to verify

- [ ] Idle waiting no longer uses `thread::sleep`
- [ ] Timers fire in correct order (same behavior as step 2)
- [ ] Spawned tasks still run without waiting for unrelated future timers
- [ ] `run()` exits when queue and timers are empty
- [ ] No busy-spin while waiting for the next timer
