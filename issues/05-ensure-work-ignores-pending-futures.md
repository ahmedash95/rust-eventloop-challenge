# `ensure_work` ignores pending futures (busy-spin risk)

**Priority:** medium before step 6  
**Introduced in:** Steps 2–4, exposed in step 5  
**Critical for:** Step 6 async TCP (I/O pending without timer)

## Problem

`ensure_work` only blocks when the callback queue is empty **and** a future timer exists:

```rust
fn ensure_work(&mut self) {
    if !self.queue.is_empty() {
        return; // never wait if callbacks pending — correct
    }

    let timeout = self.timers.peek().map(|t| ...);

    if timeout.is_some_and(|d| d > Duration::ZERO) {
        self.poll.poll(&mut events, timeout).unwrap();
    }
    // If no timers: returns immediately — does NOT wait on I/O or pending futures
}
```

If futures are pending but **nothing is on the timer heap**, the main loop spins:

```text
ensure_work (instant return) → run_timers → run_tasks → run_futures → repeat
```

## Why it matters

- **Step 5 is safe** — `sleep` always registers a timer before returning `Pending`.
- **Step 6 breaks** — a TCP read may return `Pending` waiting for socket readiness via mio, with **no timer**. Without blocking in `ensure_work`, CPU spins at 100%.
- **Misleading comment** — doc says “checks for ready tasks, expired timers, and other pending events” but pending futures alone do not cause a wait.

## Current behavior

With only `sleep` futures and `set_timeout`, every pending async task has a corresponding timer entry, so `ensure_work` always has a deadline to sleep until.

## Suggested solution

### 1. Track “needs poll” / “blocking wait” state

Add flags or counters updated when futures return `Pending`:

```rust
struct EventLoop {
    // ...
    pending_futures: usize,  // or bool has_unpolled_ready
    next_io_deadline: Option<Instant>,  // from timers
}
```

### 2. Block on mio when futures are pending and no immediate work

```rust
fn ensure_work(&mut self) {
    if !self.queue.is_empty() {
        return;
    }

    let timeout = if self.futures.is_empty() {
        self.timers.peek().map(|t| t.duration.saturating_duration_since(Instant::now()))
    } else if let Some(t) = self.timers.peek() {
        Some(t.duration.saturating_duration_since(Instant::now()))
    } else {
        // futures pending, no timers — wait for I/O indefinitely (step 6)
        None  // mio: block until socket event
    };

    if self.futures.is_empty() && timeout.is_none() {
        return; // truly nothing to do
    }

    self.poll.poll(&mut events, timeout).unwrap();
}
```

For step 6, registered sockets wake via mio events; the loop then polls futures.

### 3. Avoid busy loop when only pending futures + expired timers

Also handle `timeout == Some(ZERO)`: don’t block, fall through to `run_timers` in the same iteration (you may already rely on this).

### 4. Optional: block with `Some(ZERO)` spin guard

If misconfigured future returns `Pending` with no timer and no I/O registration, detect and panic/log in debug builds:

```rust
if !self.futures.is_empty() && self.timers.is_empty() && no_mio_interests {
    debug_assert!(false, "pending futures but nothing to wait on");
}
```

## Acceptance criteria

- [ ] Loop does not busy-spin when the only work is a pending `sleep`
- [ ] Loop blocks on mio when futures wait for I/O (step 6 — may defer part of this issue until then)
- [ ] Immediate callback queue still runs without waiting for timers
- [ ] CPU usage stays low when idle with pending async tasks

## Files involved

- `src/event_loop.rs`

## Related issues

- [02-noop-waker-full-scan-scheduling.md](02-noop-waker-full-scan-scheduling.md) — complementary; ready queue reduces useless polls, `ensure_work` fixes blocking
