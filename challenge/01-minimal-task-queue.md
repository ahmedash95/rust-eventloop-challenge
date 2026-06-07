# Step 1: Minimal Task Queue

**Builds on:** nothing — this is the foundation  
**Difficulty:** very easy

## In this stage

You'll build the smallest possible event loop: a FIFO queue of callbacks and a `run()` loop that drains it. Every later stage extends this same struct — timers, sockets, and futures all eventually schedule work back through this queue.

## What you'll implement

- Create and run an event loop
- Spawn one-shot callbacks onto a FIFO queue
- Drain the queue until empty, then return

## Where to extend your code

- `src/event_loop.rs`
- `src/main.rs`

## What to expect at the end

You can queue callbacks and run them in insertion order. Output:

```text
first
second
```

Then `run()` returns — nothing left to do.

## Hints

- Spawned work must be `'static` — it runs later and cannot borrow stack locals from `main`.
- An empty loop should no-op, not panic.

## Things to verify

- [ ] Empty event loop: `run()` returns immediately
- [ ] Three spawned tasks print in insertion order
- [ ] Each task runs exactly once
- [ ] Calling `run()` again after the queue is drained does nothing
