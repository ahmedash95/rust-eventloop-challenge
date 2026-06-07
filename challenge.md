# Build Your Own Rust Event Loop

A cumulative hands-on challenge for building a small educational async runtime in Rust.

Goal:
Build a tiny event loop step-by-step to deeply understand:

- event loops
- timers
- non-blocking IO
- polling
- futures
- wakeups
- scheduling
- async/await internals

This is NOT about building production Tokio.
This is about understanding the concepts deeply.

---

# Rules

- **Implement everything in `src/` by hand.** Challenge docs describe what to build — they do not ship solutions. Do not let tools or agents edit your implementation for you unless you explicitly want help on a specific problem.
- Prefer implementing things manually first
- Avoid Tokio initially
- Keep everything single-threaded first
- Optimize for understanding, not perfection
- Build incrementally
- Keep old APIs working while adding features

Each numbered step under [`challenge/`](challenge/) describes what to build, **what to expect at the end**, short **hints**, and a **things to verify** checklist. Start there when implementing; this file is the high-level map.

---

# Final Dream Usage

By the end, you should be able to write something conceptually like:

```rust
event_loop.spawn(async {
    println!("task started");

    sleep(Duration::from_secs(1)).await;

    let response = http_get("http://example.com").await;

    println!("{}", response.status);

    println!("done");
});

event_loop.run();
```

---

# PHASE 1 — Minimal Task Queue

> Full instructions: [challenge/01-minimal-task-queue.md](challenge/01-minimal-task-queue.md)

## In this stage

Understand the core loop.

---

## Requirements

Implement:

```rust
let mut event_loop = EventLoop::new();

event_loop.spawn(|| {
    println!("hello");
});

event_loop.run();
```

---

## Concepts

- task queue
- scheduler loop
- callbacks
- cooperative execution

---

## Success Criteria

- tasks execute in insertion order
- event loop exits when queue empty

---

# PHASE 2 — Timers

> Full instructions: [challenge/02-timers.md](challenge/02-timers.md)

## In this stage

Teach your event loop about time.

---

## Requirements

Implement:

```rust
event_loop.set_timeout(Duration::from_secs(2), || {
    println!("after 2 sec");
});
```

---

## Add Support For

- multiple timers
- timers expiring at different moments
- recurring intervals (optional; bonus in step 9)

---

## Concepts

- scheduling
- delayed execution
- sleeping efficiently
- timer queues

---

## Success Criteria

- timer fires roughly on time
- event loop does not burn CPU aggressively

---

# PHASE 3 — Reactor Foundation

> Full instructions: [challenge/03-reactor-foundation.md](challenge/03-reactor-foundation.md)

## In this stage

Add the mio reactor — unified wait via `poll()` instead of `thread::sleep`. Plumbing only; public async TCP comes in phase 6.

---

## Requirements

- Unified wait via `mio` instead of `thread::sleep`
- Steps 1–2 behavior unchanged (timers + tasks still work)

---

## Concepts

- reactor pattern (mio, epoll/kqueue)
- readiness notification
- unified wait (timers + I/O in one blocking call)

---

## Success Criteria

- timers fire correctly through poll-based wait
- no busy-spin while idle

---

# PHASE 4 — Tiny Executor

> Full instructions: [challenge/04-tiny-executor.md](challenge/04-tiny-executor.md)

## In this stage

Understand Rust async internals — `Future::poll`, `Waker`, `Pin`.

---

## Requirements

Implement:

```rust
event_loop.spawn(async {
    println!("hello");
});
```

without Tokio. Keep callback APIs from earlier phases working.

---

## Learn About

- Future trait
- Poll
- Context
- Waker
- Pin

---

## Success Criteria

- async tasks can run on your event loop

---

# PHASE 5 — Implement sleep().await

> Full instructions: [challenge/05-sleep-future.md](challenge/05-sleep-future.md)

## In this stage

Connect timers with async futures.

---

## Requirements

Implement:

```rust
sleep(Duration::from_secs(1)).await;
```

---

## Concepts

- waking suspended tasks
- task scheduling
- timer-driven futures

---

## Mental Model

```text
future not ready
→ store waker
→ timer expires
→ wake task
→ event loop polls again
```

---

## Success Criteria

- task pauses without blocking the event loop

---

# PHASE 6 — Async TCP I/O

> Full instructions: [challenge/06-async-tcp.md](challenge/06-async-tcp.md)

## In this stage

Add async TCP on the reactor — connect, read, write as futures.

---

## Requirements

```rust
event_loop.spawn(async {
    let mut stream = TcpStream::connect("127.0.0.1:8080").await.unwrap();
    stream.write_all(b"hello").await.unwrap();
    let data = stream.read_to_end().await.unwrap();
});
```

---

## Concepts

- non-blocking sockets (`set_nonblocking`, `WouldBlock`)
- partial reads/writes
- I/O-driven wakeups

---

## Success Criteria

- echo round-trip through async API
- timers still fire while sockets wait
- multiple concurrent connections work

---

# PHASE 7 — Async HTTP Future

> Full instructions: [challenge/07-async-http-future.md](challenge/07-async-http-future.md)

## In this stage

HTTP GET as a future — `http_get(url).await`.

---

## Requirements

Implement:

```rust
let response = http_get(url).await;
```

---

## Concepts

- HTTP over TCP
- state machine (Connect → WriteRequest → ReadHeaders → ReadBody → Done)
- buffering and parsing

---

## Success Criteria

- multiple concurrent requests work
- event loop remains responsive

---

# PHASE 8 — MVP Runtime

> Full instructions: [challenge/08-mini-async-event-loop.md](challenge/08-mini-async-event-loop.md)

## In this stage

Integrate everything into a mini-Tokio MVP with reliability basics.

---

## Final API

```rust
event_loop.spawn(async {
    sleep(Duration::from_secs(1)).await;

    let response = http_get("http://example.com").await;

    println!("{}", response.status);

    println!("done");
});

event_loop.run();
```

---

## MVP additions

- `shutdown()` — graceful stop
- `timeout(duration, future).await`
- `join(fut_a, fut_b).await`

---

# BONUS CHALLENGES

> Full instructions: [challenge/09-bonus-challenges.md](challenge/09-bonus-challenges.md)

## Easy

- recurring intervals
- task IDs and cancellation

## Medium

- task priorities
- async file reads
- `select` combinator

## Hard

- multithreaded executor
- work stealing
- lock-free queues
- io_uring
- async DNS
- reactor/proactor separation

---

# Recommended Learning Topics Per Phase

| Phase | Topics |
|---|---|
| 1 | queues, schedulers |
| 2 | timers, scheduling |
| 3 | mio, reactor pattern, unified wait |
| 4 | Future/Waker/Poll/Pin |
| 5 | wakeups, timer-driven futures |
| 6 | non-blocking TCP, I/O state machines |
| 7 | async HTTP, parsing, buffering |
| 8 | runtime integration, shutdown, timeout, join |

---

# Final Important Advice

Do NOT rush.

The goal is not finishing quickly.

The goal is repeatedly reaching moments like:

```text
OH.
THAT is how async actually works.
```

That understanding compounds massively across:

- backend systems
- databases
- UI frameworks
- networking
- distributed systems
- game engines
- operating systems
- async programming
