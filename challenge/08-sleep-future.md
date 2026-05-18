# Step 8: `sleep().await`

Connect timers to the future executor so async tasks can pause without blocking.

## What To Implement

- A `sleep(duration) -> Sleep` future.
- `Sleep` stores its deadline and the current task's `Waker` when pending.
- Timer expiration wakes the sleeping task.
- The event loop polls the task again after it is woken.

## Requirements

- `sleep().await` does not block the whole event loop thread.
- Multiple sleeping tasks can wait for different durations.
- A sleeping task resumes only after its deadline.
- Timers should reuse or integrate with the timer system from Step 2.

## Test-Oriented Examples

```rust
event_loop.spawn(async {
    println!("before");
    sleep(Duration::from_millis(10)).await;
    println!("after");
});

event_loop.run();
```

Use cases to test:

- Code after `sleep().await` runs after the delay.
- A task with a shorter sleep completes before a task with a longer sleep.
- Other ready tasks run while one task is sleeping.
- A zero-duration sleep yields once or completes immediately, depending on your chosen semantics.
- Dropping or completing a sleeping future does not leave a timer that wakes a dead task.
