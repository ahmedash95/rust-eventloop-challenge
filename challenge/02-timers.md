# Step 2: Timers

Teach the event loop to delay work until a specific time.

## What To Implement

- `event_loop.set_timeout(duration, callback)`
- A timer storage structure containing `execute_at: Instant` and a callback.
- Event loop behavior that executes ready timers and sleeps or waits when only future timers remain.

Keep `spawn()` and `run()` from Step 1 working.

## Requirements

- Multiple timers can be scheduled at once.
- Timers fire after their delay has elapsed.
- Earlier timers fire before later timers.
- The event loop should avoid a busy loop while waiting for the next timer.
- Optional: add `set_interval(duration, callback)` for recurring callbacks.

## Test-Oriented Examples

```rust
let mut event_loop = EventLoop::new();

event_loop.set_timeout(Duration::from_millis(20), || {
    println!("later");
});

event_loop.run();
```

Use cases to test:

- A zero-duration timer runs during the next `run()`.
- A 10 ms timer fires before a 50 ms timer.
- Normal queued tasks run without waiting for future timers.
- `run()` exits after all timers have fired.
- The observed firing time is greater than or equal to the requested delay, allowing a small tolerance for scheduling.
