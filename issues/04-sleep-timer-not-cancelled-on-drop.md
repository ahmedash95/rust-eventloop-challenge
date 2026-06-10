# Sleep timer not cancelled when future is dropped

**Priority:** low (step 5) → medium (step 8 join/cancel)  
**Introduced in:** Step 5

## Problem

When a `Sleep` future is dropped before its deadline (cancelled task, timeout, abandoned `.await`), the timer entry remains on the heap until it fires:

```rust
// sleep registers once, never removes itself
register_wakeup(self.deadline, waker);
```

When the deadline passes, `run_timers` still runs:

```rust
TimerKind::Waker(waker) => waker.wake(),
```

## Why it matters

- **Spurious wakeups** — wakes a task that no longer exists (may be harmless or may poll a dropped future if ids are reused carelessly).
- **Heap pollution** — cancelled sleeps accumulate until expiry.
- **Step 8** — `join`, timeouts, and shutdown need clean cancellation semantics.

## Current behavior

No task cancellation exists, so every spawned async task runs to completion. The leak is latent, not user-visible in step 5 tests.

## Suggested solution

### 1. Track timer identity

Give each sleep a unique id (monotonic `u64`) or store an index into a side table:

```rust
struct Sleep {
    deadline: Instant,
    timer_id: Option<u64>,
}
```

### 2. Cancel on drop

```rust
impl Drop for Sleep {
    fn drop(&mut self) {
        if let Some(id) = self.timer_id {
            with_current_loop(|ptr| unsafe {
                (*ptr).cancel_timer(id);
            });
        }
    }
}
```

`cancel_timer` removes the entry from the heap (or marks it cancelled in a side map).

### 3. Heap removal is awkward with `BinaryHeap`

`BinaryHeap` has no efficient arbitrary delete. Options:

| Approach | Tradeoff |
|---|---|
| **Lazy deletion** — `HashSet<u64>` of cancelled ids; skip on pop in `run_timers` | Simple, some wasted pops |
| **Separate map** — `BTreeMap<Instant, Vec<TimerId>>` instead of heap | Easier cancellation, refactor timers |
| **Generational ids** — ignore wake if generation mismatch | Good with task slab |

For a learning runtime, **lazy deletion** is usually enough:

```rust
cancelled_timers: HashSet<u64>,

fn run_timers(&mut self) {
    while let Some(timer) = self.timers.peek() {
        if timer.id.is_some_and(|id| self.cancelled_timers.contains(&id)) {
            self.timers.pop();
            continue;
        }
        // ...
    }
}
```

### 4. Ensure waker wake on cancelled timer is safe

If the future is dropped, the waker should either be dropped too or `wake()` should be a no-op that does not push a stale task id onto the ready queue.

## Acceptance criteria

- [ ] Dropping a `Sleep` before expiry removes or invalidates its timer
- [ ] No panic when the old deadline passes after drop
- [ ] Completed sleeps (returned `Ready`) do not double-cancel or double-fire
- [ ] `set_timeout` callbacks unaffected

## Files involved

- `src/sleep.rs`
- `src/event_loop.rs`
- `src/timer.rs`

## Notes

Consider implementing together with issue 02 (task ids) so timer cancellation and ready-queue wakeups share the same identity scheme.
