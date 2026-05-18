# Eventloop

Hands-on exercises for building a small educational async event loop in Rust: from a minimal task queue through timers, non-blocking I/O, polling, futures, and async/await.

The point is to understand how schedulers, wakeups, and readiness-based I/O fit together—not to ship a replacement for Tokio or another production runtime.

By the end, the API you are aiming for looks roughly like this (names and types are for illustration—your crate will grow into them over the phases):

```rust
use std::time::Duration;

let mut event_loop = EventLoop::new();

event_loop.spawn(async {
    println!("task started");

    sleep(Duration::from_secs(1)).await;

    let response = http_get("http://example.com").await;

    println!("{}", response.status);

    println!("done");
});

event_loop.run();
```

## Repository layout

- [`challenge.md`](challenge.md) — Full write-up: rules, phases 1–10, bonus ideas, and the target API shape.
- [`challenge/`](challenge/) — Same path split into smaller steps; see [`challenge/README.md`](challenge/README.md) for the ordered list.

Read `challenge.md` for context, or jump into the numbered files under `challenge/` and implement one layer at a time.

## Guidelines

- Prefer implementing pieces yourself before leaning on Tokio.
- Start single-threaded; optimize for clarity over a “perfect” design.
- Add features incrementally and keep earlier behavior working.

## Steps

1. [Minimal task queue](challenge/01-minimal-task-queue.md)
2. [Timers](challenge/02-timers.md)
3. [Non-blocking TCP](challenge/03-non-blocking-tcp.md)
4. [Polling / I/O event loop](challenge/04-polling-io-event-loop.md)
5. [Simple HTTP client](challenge/05-simple-http-client.md)
6. [Generator-style tasks](challenge/06-generator-style-tasks.md)
7. [Tiny executor](challenge/07-tiny-executor.md)
8. [Sleep future](challenge/08-sleep-future.md)
9. [Async HTTP future](challenge/09-async-http-future.md)
10. [Mini async event loop](challenge/10-mini-async-event-loop.md)
11. [Bonus challenges](challenge/11-bonus-challenges.md)

## Getting started

Create a Rust crate and implement each phase against the docs here, using the Rust snippets in each file as examples for tests or small programs.

```sh
cargo new eventloop-demo
cd eventloop-demo
```

There is no prize for finishing quickly; the useful part is when async behavior and runtime internals actually make sense.
