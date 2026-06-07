# Step 2: Timers

**Builds on:** [Step 1 — Minimal task queue](01-minimal-task-queue.md)  
**Difficulty:** easy

## In this stage

You'll teach your event loop about time. Callbacks registered with `set_timeout` should fire after a delay — but the loop must not busy-spin while waiting. When no tasks are ready, wait until the nearest timer deadline.

Step 1's `spawn()` and `run()` must still work unchanged.

## What you'll implement

- One-shot delayed callbacks via `set_timeout`
- Multiple timers with different deadlines
- Efficient waiting when only future timers remain

## Where to extend your code

- `src/timer.rs` (new file)
- `src/event_loop.rs`

## What to expect at the end

You can mix immediate tasks and delayed callbacks. Output (rough ordering):

```text
immediate
10ms
50ms
```

The loop waits efficiently between work — no busy-spin. Step 3 swaps the wait mechanism for a reactor; timer behavior stays the same.

## Hints

- Ready tasks must not block on unrelated future timers — only wait when the task queue is empty.
- If timers never fire, check that the soonest deadline is picked first.

## Things to verify

- [ ] Zero-duration timer runs during the next `run()` iteration
- [ ] A 10 ms timer fires before a 50 ms timer
- [ ] Spawned tasks run without waiting for unrelated future timers
- [ ] `run()` exits after all timers have fired
- [ ] Observed delay is ≥ requested delay (allow small OS tolerance)
