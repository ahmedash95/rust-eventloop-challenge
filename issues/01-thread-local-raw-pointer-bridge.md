# Thread-local + raw pointer bridge between `Sleep` and `EventLoop`

**Priority:** medium  
**Introduced in:** Step 5 — Sleep future  
**Blocks:** clean path to multi-threading, robust error handling

## Problem

`sleep()` registers timers by reading a thread-local pointer to the active `EventLoop`:

```rust
thread_local! { static CURRENT_LOOP: Cell<*mut EventLoop> ... }

// sleep.rs
with_current_loop(|ptr| unsafe { (*ptr).register_wakeup(...) });

// event_loop.rs — set only during run_futures()
set_current_loop(self as *mut Self);
```

This is hidden coupling: `Sleep::poll` has no typed connection to the executor, only a side channel valid during one phase of the loop.

## Why it matters

- **Fragile lifetime contract** — the pointer is only valid while `run_futures()` runs. Any code that polls a `Sleep` outside that window panics (or would be UB without the assert).
- **Panic safety** — if `run_futures()` panics before `set_current_loop(null)`, the thread-local may stay set and point at a stale `EventLoop`.
- **Unsafe** — correctness depends on manual discipline, not the type system.
- **Hard to extend** — TCP/HTTP futures will need the same bridge unless you replace this pattern first.

## Current behavior

Works correctly for single-threaded step 5: `run_futures` sets the pointer, polls all futures (including `sleep`), then clears it.

## Suggested solution

Pick one of these (ordered from simplest to most robust):

### Option A — RAII guard (minimal fix)

Keep the thread-local, but never clear manually:

```rust
struct CurrentLoopGuard(*mut EventLoop);

impl Drop for CurrentLoopGuard {
    fn drop(&mut self) {
        set_current_loop(std::ptr::null_mut());
    }
}

fn run_futures(&mut self) {
    let _guard = CurrentLoopGuard(self as *mut Self);
    // poll...
}
```

Fixes panic-leaving-pointer-set. Does not remove unsafe or hidden coupling.

### Option B — Registration queue (recommended for step 6+)

Remove the thread-local entirely:

1. Add `pending_timer_registrations: Vec<(Instant, Waker)>` on `EventLoop`.
2. Expose `pub(crate) fn push_timer_registration(deadline, waker)` backed by a thread-local **queue cell** or passed through a custom waker — or drain via a static `OnceLock<Mutex<Vec<...>>>` for single-thread (even simpler: store queue on `EventLoop`, pass `&mut EventLoop` only inside `run_futures` by having sleep register through a closure — still needs a bridge).

Practical single-thread version:

```rust
// On EventLoop
timer_registrations: Vec<(Instant, Waker)>,

fn drain_timer_registrations(&mut self) {
    for (deadline, waker) in self.timer_registrations.drain(..) {
        self.register_wakeup(deadline, waker);
    }
}
```

Have `Sleep::poll` push to a thread-local `Vec` that `run_futures` drains into the heap **after** each poll pass. No raw pointer to `EventLoop`.

### Option C — Custom waker carries executor context (Tokio-style)

Build a `RawWaker` whose `wake` pushes a task id onto a ready queue. Timer registration happens through data embedded in the waker or a handle stored when the task is spawned.

More work, but this is the production pattern.

## Acceptance criteria

- [ ] No `unsafe` dereference of `*mut EventLoop` from `sleep.rs` (Option B or C), **or** guard + documented invariants (Option A)
- [ ] Panic during `run_futures` does not leave a dangling `CURRENT_LOOP`
- [ ] `sleep()` still works alongside `set_timeout()`
- [ ] Pattern is documented so step 6 futures can reuse the same registration path

## Files involved

- `src/event_loop.rs`
- `src/sleep.rs`
