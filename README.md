# Eventloop

Hands-on exercises for building a small educational async event loop in Rust: from a minimal task queue through timers, a mio reactor, futures, wakers, async TCP/HTTP, and a mini-Tokio MVP.

The point is to understand how schedulers, wakeups, and readiness-based I/O fit together—not to ship a replacement for Tokio or another production runtime.

**You implement all code in `src/` by hand.** The challenge docs guide structure and verify behavior; they do not ship full solutions.

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

- [`challenge.md`](challenge.md) — Full write-up: rules, phases 1–9, bonus ideas, and the target API shape.
- [`challenge/`](challenge/) — Step-by-step instructions with expected outcomes, hints, and verify checklists; see [`challenge/README.md`](challenge/README.md).

Read `challenge.md` for the high-level map, or jump into the numbered files under `challenge/` and implement one stage at a time.

## Guidelines

- Prefer implementing pieces yourself before leaning on Tokio.
- Start single-threaded; optimize for clarity over a “perfect” design.
- Add features incrementally and keep earlier behavior working.

## Progress

- [x] [1. Minimal task queue](challenge/01-minimal-task-queue.md) — `EventLoop::new`, `spawn`, `run`, FIFO `VecDeque`, `FnOnce() + 'static`
- [x] [2. Timers](challenge/02-timers.md) — `set_timeout`, timer heap, sleep until next deadline
- [x] [3. Reactor foundation](challenge/03-reactor-foundation.md) — internal mio, `poll.poll` replaces `thread::sleep`
- [x] [4. Tiny executor](challenge/04-tiny-executor.md) — `spawn_async`, `Future::poll`, noop waker
- [x] [5. Sleep future](challenge/05-sleep-future.md) — `sleep(duration).await` via timers + executor
- [ ] [6. Async TCP](challenge/06-async-tcp.md) — `TcpStream::connect(...).await`, read/write futures
- [ ] [7. Async HTTP future](challenge/07-async-http-future.md) — `http_get(url).await`
- [ ] [8. MVP runtime](challenge/08-mini-async-event-loop.md) — integration, shutdown, timeout, join
- [ ] [9. Bonus challenges](challenge/09-bonus-challenges.md)

## Getting started

Create a Rust crate and implement each stage using the numbered files under `challenge/`.

```sh
cargo new eventloop-demo
cd eventloop-demo
```

There is no prize for finishing quickly; the useful part is when async behavior and runtime internals actually make sense.
