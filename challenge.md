# Build Your Own Rust Event Loop

A cumulative hands-on challenge inspired by systems programming and async event loops.

Goal:
Build a tiny educational async event loop step-by-step to deeply understand:

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

- Prefer implementing things manually first
- Avoid Tokio initially
- Keep everything single-threaded first
- Optimize for understanding, not perfection
- Build incrementally
- Keep old APIs working while adding features

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

## Goal

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

## Suggested Structures

```rust
VecDeque<Box<dyn FnMut()>>
```

---

## Success Criteria

- tasks execute in insertion order
- event loop exits when queue empty

---

# PHASE 2 — Timers

## Goal

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
- recurring intervals (optional)

---

## Concepts

- scheduling
- delayed execution
- sleeping efficiently
- timer queues

---

## Suggested Structures

```rust
struct Timer {
    execute_at: Instant,
    callback: Box<dyn FnMut()>,
}
```

---

## Success Criteria

- timer fires roughly on time
- event loop does not burn CPU aggressively

---

# PHASE 3 — Non-Blocking TCP

## Goal

Handle real IO.

---

## Requirements

Open non-blocking sockets.

Learn:

```rust
TcpStream::set_nonblocking(true)
```

---

## Build

Tiny TCP echo client or server.

---

## Concepts

- sockets
- readable/writable states
- blocking vs non-blocking IO
- partial reads/writes

---

## Success Criteria

- event loop does not freeze while waiting for network

---

# PHASE 4 — Polling / IO Event Loop

## Goal

Make the event loop wait efficiently.

---

## Requirements

Use:

- mio
- epoll/kqueue indirectly via mio

---

## Build

The event loop should:

```text
register sockets
wait for readiness
wake handlers
```

---

## Concepts

- reactor pattern
- readiness notifications
- event dispatching
- OS polling

---

## Suggested APIs

```rust
event_loop.watch_readable(socket, callback)
```

---

## Success Criteria

- multiple sockets can exist simultaneously
- event loop sleeps efficiently
- callbacks fire only when sockets ready

---

# PHASE 5 — Simple HTTP Client

## Goal

Use your event loop for real async work.

---

## Requirements

Implement:

```rust
event_loop.http_get("http://example.com", |response| {
    println!("{}", response.status);
});
```

---

## Concepts

- HTTP over TCP
- async response handling
- buffering
- parsing
- callbacks

---

## Success Criteria

- event loop can perform multiple simultaneous HTTP requests
- requests do not block the event loop

---

# PHASE 6 — Generator-Style Tasks

## Goal

Understand pausable execution.

---

## Requirements

Create tasks that can:

```text
pause
resume
continue later
```

---

## Concepts

- coroutines
- resumable tasks
- state machines
- yielding execution

---

## Suggested Direction

You can simulate this manually before real Rust futures.

---

## Success Criteria

- long tasks no longer block the event loop entirely

---

# PHASE 7 — Build A Tiny Executor

## Goal

Understand Rust async internals.

---

## Requirements

Implement:

```rust
event_loop.spawn(async {
    println!("hello");
});
```

without Tokio.

---

## Learn About

- Future trait
- Poll
- Context
- Waker
- Pin

---

## Core Insight

Your event loop repeatedly:

```rust
future.poll()
```

---

## Success Criteria

- async tasks can run on your event loop

---

# PHASE 8 — Implement sleep().await

## Goal

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

# PHASE 9 — Async HTTP Future

## Goal

Combine sockets + futures + wakeups.

---

## Requirements

Implement:

```rust
let response = http_get(url).await;
```

---

## Concepts

- socket registration
- readiness wakeups
- future polling
- state machines

---

## Core Flow

```text
poll()
→ socket not ready
→ store waker
→ return Pending

socket readable
→ wake()
→ event loop polls again

response complete
→ return Ready(response)
```

---

## Success Criteria

- multiple concurrent requests work
- event loop remains responsive

---

# PHASE 10 — Mini Async Event Loop

## Goal

Put everything together.

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

# BONUS CHALLENGES

## Easy

- recurring intervals
- task IDs
- event loop shutdown
- cancellation
- graceful socket closing

---

## Medium

- task priorities
- async file reads
- select/join combinators
- timeout futures

---

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
| 3 | sockets |
| 4 | polling, reactor pattern |
| 5 | HTTP internals |
| 6 | coroutines/state machines |
| 7 | Future/Waker/Poll |
| 8 | wakeups |
| 9 | async IO |
| 10 | event loop architecture |

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
