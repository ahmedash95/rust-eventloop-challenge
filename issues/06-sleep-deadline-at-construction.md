# `Sleep` deadline computed at construction, not first poll

**Priority:** low  
**Introduced in:** Step 5

## Problem

The sleep duration starts counting when `sleep()` is called, not when the future is first polled:

```rust
pub fn sleep(duration: Duration) -> Sleep {
    Sleep {
        deadline: Instant::now() + duration,  // clock starts here
        registered: false,
    }
}
```

In typical `sleep(d).await` usage, creation and first poll happen back-to-back, so behavior matches intuition. If the future is constructed early and awaited later, the effective sleep is **shorter** than requested.

## Example

```rust
let fut = sleep(Duration::from_secs(1));
expensive_sync_work(); // takes 200ms
fut.await;             // only waits ~800ms more
```

Most runtimes (including Tokio’s `tokio::time::sleep`) start the timer at **first poll**, not at builder construction.

## Why it matters

- Subtle semantic difference from production APIs.
- Unlikely to bite in simple spawn-and-run code, but surprising in composed futures or manual future building.
- Low urgency for the learning project unless you expose `sleep()` as a public API others will hold onto.

## Current behavior

Deadline is fixed at `Sleep` struct creation. Works as expected when polled immediately inside `spawn_async` blocks.

## Suggested solution

### Option A — Start at first poll (match Tokio)

```rust
pub struct Sleep {
    duration: Duration,
    deadline: Option<Instant>,
    registered: bool,
}

pub fn sleep(duration: Duration) -> Sleep {
    Sleep { duration, deadline: None, registered: false }
}

fn poll(...) -> Poll<()> {
    if self.deadline.is_none() {
        self.deadline = Some(Instant::now() + self.duration);
    }
    let deadline = self.deadline.unwrap();
    // ...
}
```

### Option B — Document current behavior

If you prefer construction-time start (valid choice for a minimal runtime), document it on `sleep()`:

```rust
/// The duration begins when this future is created, not when first polled.
pub fn sleep(duration: Duration) -> Sleep
```

## Acceptance criteria

- [ ] Behavior is either aligned with Tokio (first poll) **or** explicitly documented
- [ ] Existing step 5 tests still pass
- [ ] Zero-duration sleep still completes immediately on first poll

## Files involved

- `src/sleep.rs`
