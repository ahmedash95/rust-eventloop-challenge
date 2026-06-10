# Noop waker + full-scan scheduling

**Priority:** medium  
**Introduced in:** Steps 4–5  
**Becomes painful in:** Step 6 (async TCP), many concurrent tasks

## Problem

The executor polls every in-flight future on every loop iteration using a noop waker:

```rust
let waker = Waker::noop();
let mut cx = Context::from_waker(&waker);
self.futures.retain_mut(|future| future.as_mut().poll(&mut cx).is_pending());
```

`sleep` stores a clone of that noop waker on the timer heap. When the timer fires:

```rust
TimerKind::Waker(waker) => waker.wake(), // does nothing
```

Progress happens only because `run_futures()` runs again and **polls all futures**, not because `wake()` re-queued a specific task.

## Why it matters

- **Misleading model** — you store wakers but scheduling is really “poll everything every tick.”
- **O(n) cost** — every loop pass polls every pending future, even if only one timer expired.
- **Step 6 gap** — I/O futures expect `wake()` to mean “this task is ready”; full-scan may work but hides bugs and wastes CPU.
- **Not how real runtimes work** — Tokio/async-std use a ready queue driven by wakers.

## Current behavior

Correct for step 5 with few tasks. Timer expiry → next `run_futures` pass → `Sleep` sees `now >= deadline` → `Ready`.

## Suggested solution

Introduce a **ready queue** and a **real waker per task**.

### 1. Give each spawned future a task id

```rust
struct Task {
    id: usize,
    future: Pin<Box<dyn Future<Output = ()> + 'static>>,
}

// On EventLoop
tasks: Slab<Task>,           // or Vec<Option<Task>>
ready: VecDeque<usize>,      // task ids to poll
```

### 2. Custom waker wakes by task id

```rust
fn waker_for(task_id: usize) -> Waker {
    // RawWaker vtable: wake clones task_id and pushes onto READY queue
    // Use thread-local or global RefCell for the ready queue in single-thread runtime
}
```

When `TimerKind::Waker(waker)` fires, `waker.wake()` pushes that task’s id onto `ready`.

### 3. Poll only ready tasks (plus newly spawned)

```rust
fn run_futures(&mut self) {
    while let Some(id) = self.ready.pop_front() {
        let task = &mut self.tasks[id];
        let waker = waker_for(id);
        let mut cx = Context::from_waker(&waker);
        if task.future.as_mut().poll(&mut cx).is_ready() {
            self.tasks.remove(id);
        }
    }
}
```

Newly spawned tasks are pushed onto `ready` immediately.

### 4. Timer wakers must use the task’s waker

When `sleep` registers, it should store the waker from `cx` **after** step 3 provides a real one — not `Waker::noop()`.

## Acceptance criteria

- [ ] `Waker::wake()` causes the associated task to be polled on a subsequent iteration
- [ ] Unrelated pending tasks are not polled when one timer fires
- [ ] `sleep().await` and `set_timeout()` still behave correctly
- [ ] Newly spawned async tasks run without requiring a timer/IO wakeup

## Files involved

- `src/event_loop.rs`
- `src/sleep.rs` (no logic change if waker comes from real `Context`)
- Optional new file: `src/waker.rs`

## References

- [Rust async book — Task wakeups](https://rust-lang.github.io/async-book/02_execution/03_wakeups.html)
- [Rust async book — Build an executor](https://rust-lang.github.io/async-book/02_execution/04_executor.html)
