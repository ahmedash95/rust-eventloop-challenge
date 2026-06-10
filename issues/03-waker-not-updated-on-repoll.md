# Waker not updated when `Sleep` is re-polled

**Priority:** low (until real wakers land)  
**Depends on:** [02-noop-waker-full-scan-scheduling.md](02-noop-waker-full-scan-scheduling.md)  
**Introduced in:** Step 5

## Problem

`Sleep` registers its timer and waker only once:

```rust
if !self.registered {
    let waker = cx.waker().clone();
    register_wakeup(self.deadline, waker);
    self.get_mut().registered = true;
}
Poll::Pending
```

The Rust `Future` contract says: if you return `Poll::Pending`, you must treat the waker in `Context` as the latest way to notify the task. Executors may call `poll` multiple times before the operation completes, potentially with different wakers.

## Why it matters

- With **noop wakers + full scan**, this bug is masked — you poll everyone anyway.
- With **real wakers** (issue 02), a stale waker on the timer heap may wake the wrong notifier or a dropped waker, and the task may never run again.
- Production runtimes always update the waker while still pending.

## Current behavior

First poll registers timer + waker. Subsequent polls return `Pending` without touching the heap entry. Works today because scheduling does not rely on the stored waker firing correctly.

## Suggested solution

### Option A — Update waker on every pending poll (simple)

Remove the `registered` flag’s “only once” semantics for the waker. Still register the timer once, but update the waker:

```rust
if !self.registered {
    register_timer(deadline); // timer without waker, or combined
    self.registered = true;
}
update_timer_waker(deadline, cx.waker().clone());
Poll::Pending
```

This requires either:
- storing waker separately keyed by deadline/task id, or
- cancelling and re-pushing the timer each poll (wasteful but simple for learning).

### Option B — Store waker on the future, not the timer (recommended)

Keep one timer per sleep keyed by task id. On each pending poll:

```rust
self.waker = Some(cx.waker().clone());
Poll::Pending
```

When the timer fires, look up the task by id and call **the waker stored on the task/future**, not the waker copied into the heap at registration time.

Separates “when to wake” (timer heap) from “who to wake” (latest waker on the task).

### Option C — Re-register timer each poll

Push a new `TimerKind::Waker` every poll (and accept duplicate heap entries until expiry). Easiest to implement, messy heap.

## Acceptance criteria

- [ ] After switching to real wakers (issue 02), a task that is polled twice before expiry still completes
- [ ] No duplicate timer firings cause panics or lost wakeups
- [ ] Document which approach you chose in a code comment

## Files involved

- `src/sleep.rs`
- `src/event_loop.rs`
- `src/timer.rs` (if timer entries change shape)
