# Step 5: `sleep().await`

**Builds on:** [Step 2 — Timers](02-timers.md), [Step 4 — Tiny executor](04-tiny-executor.md)  
**Difficulty:** medium

## In this stage

You'll add `sleep(duration).await` — a future backed by your timer heap. The task pauses without blocking the thread: store a waker, return pending, wake when the deadline passes.

## What you'll implement

- `sleep(duration).await` inside async tasks
- Timer expiry wakes sleeping tasks (same heap as step 2 — no second timer system)

## Where to extend your code

- `src/sleep.rs` (new file)
- `src/event_loop.rs`
- `src/timer.rs` (may need to handle waker-based timers)

## What to expect at the end

```rust
event_loop.spawn(async {
    println!("before");
    sleep(Duration::from_millis(10)).await;
    println!("after");
});

event_loop.run();
```

Output:

```text
before
after
```

There is a ~10 ms gap between the lines — other tasks can run during it. Two tasks with different sleeps finish in deadline order.

## Hints

- Never block the thread inside a future's poll.
- If code after `.await` never runs, the waker probably never fired.

## Things to verify

- [ ] Code after `sleep().await` runs after the delay
- [ ] Shorter sleep completes before longer sleep
- [ ] Other tasks run while one task is sleeping
- [ ] `set_timeout()` callbacks still work alongside sleeping tasks
