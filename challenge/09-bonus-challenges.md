# Step 9: Bonus Challenges

**Builds on:** [Step 8 — MVP runtime](08-mini-async-event-loop.md)

Optional extensions — pick what interests you. Define semantics **before** implementing each one.

Note: `shutdown`, `timeout`, and `join` are part of step 8 — don't redo them here unless extending.

## Easy

- **Recurring intervals** — repeat a callback every N ms until cancelled
- **Task IDs and cancellation** — cancel a spawned task or pending sleep

## Medium

- **Task priorities** — high-priority tasks run first under load
- **Async file reads** — same reactor pattern as TCP
- **`select(a, b).await`** — complete when the first future finishes; document what happens to the loser

## Hard

- Multithreaded executor, work stealing, lock-free queues
- io_uring, async DNS, reactor/proactor separation

## What to expect at the end

Each extension should be demonstrable in a small demo. Example shapes:

```rust
// Recurring interval
let id = event_loop.set_interval(Duration::from_secs(1), || println!("tick"));
// event_loop.clear_interval(id);

// select — first future wins
event_loop.spawn(async {
    let first = select(sleep(Duration::from_secs(5)), http_get("http://127.0.0.1:8080/")).await;
});

event_loop.run();
```

Define and document semantics for each extension before implementing.

## Hints

- Decide cancellation semantics upfront: drop cleanup, timer removal, open sockets.
- Intervals reschedule themselves; one-shot timers do not.

## Things to verify (pick per extension)

- [ ] Interval fires repeatedly until cancelled
- [ ] Cancelled task never runs again
- [ ] `select` completes when the first future completes
- [ ] Async file read doesn't block other tasks
