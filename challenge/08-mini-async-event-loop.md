# Step 8: MVP Runtime

**Builds on:** [All prior steps](README.md)  
**Difficulty:** medium (integration)

## In this stage

Wire everything into a **mini-Tokio MVP** — one `run()` loop coordinating callbacks, timers, I/O, and async futures. Add shutdown, timeouts, and concurrent helpers.

## What you'll implement

- One loop that runs all subsystems from steps 1–7 together
- Clean exit when nothing is pending
- `shutdown()`, `timeout(...).await`, and `join(...).await`

## Where to extend your code

- `src/event_loop.rs`
- `src/main.rs` — end-to-end demo

## What to expect at the end

```rust
let mut event_loop = EventLoop::new();

event_loop.spawn(async {
    println!("task started");
    sleep(Duration::from_millis(10)).await;
    let response = http_get("http://127.0.0.1:8080/").await.unwrap();
    println!("status: {}", response.status);
    println!("done");
});

event_loop.spawn(async {
    sleep(Duration::from_millis(20)).await;
    println!("task A done");
});

event_loop.spawn(async {
    sleep(Duration::from_millis(5)).await;
    let r = http_get("http://127.0.0.1:8080/").await.unwrap();
    println!("task B: {}", r.status);
});

event_loop.run();
```

Tasks interleave — shorter sleeps finish first. HTTP doesn't block timers. `run()` returns with no CPU spin. With `shutdown()`, new work is rejected and in-flight work drains per your policy.

## Hints

- Poll futures after wake sources fire (timers, I/O).
- Define shutdown semantics upfront — what happens to in-flight HTTP and timers?

## Things to verify

- [ ] Task sleeps, resumes, performs HTTP, completes
- [ ] Two HTTP requests run concurrently
- [ ] `timeout()` errors when inner future is too slow
- [ ] `run()` exits after all work completes
- [ ] Mixing callback tasks, timers, and async tasks works
- [ ] `shutdown()` stops new work and exits cleanly
