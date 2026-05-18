# Step 10: Mini Async Event Loop

Put the pieces together into the final educational event loop API.

## What To Implement

- A cohesive `EventLoop` that owns the executor, timers, and IO reactor.
- `event_loop.spawn(async { ... })`
- `event_loop.run()`
- `sleep(duration).await`
- `http_get(url).await`
- Clear shutdown behavior when no tasks, timers, or IO registrations remain.

## Requirements

- Async tasks can sleep and then perform HTTP work.
- Multiple tasks can run concurrently on the single-threaded event loop.
- The event loop remains responsive while timers and sockets are pending.
- Old APIs should keep working if you intentionally kept them public.
- The implementation should favor clarity over production-grade completeness.

## Test-Oriented Examples

```rust
let mut event_loop = EventLoop::new();

event_loop.spawn(async {
    sleep(Duration::from_millis(10)).await;

    let response = http_get("http://127.0.0.1:8080/").await.unwrap();
    println!("{}", response.status);
});

event_loop.run();
```

Use cases to test:

- A task sleeps, resumes, performs HTTP, and completes.
- Two tasks with different sleep durations complete in deadline order.
- Two HTTP requests run concurrently after a sleep.
- Event loop exits after all spawned work is complete.
- Event loop does not spin CPU while all tasks are waiting.
