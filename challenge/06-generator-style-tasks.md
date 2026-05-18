# Step 6: Generator-Style Tasks

Model pausable execution before using Rust's `Future` trait directly.

## What To Implement

- A task abstraction that can return "done" or "not done yet".
- A way for tasks to yield control back to the event loop.
- State stored inside the task so it can continue later.
- Event loop scheduling that resumes yielded tasks without blocking other work.

You can implement this manually with enums and structs instead of unstable generators.

## Requirements

- A long task can make progress over multiple event loop ticks.
- Yielded tasks are rescheduled fairly with other queued tasks.
- Completed tasks are removed and not polled again.
- Existing callback, timer, and IO behavior should still work.

## Test-Oriented Examples

```rust
enum TaskState {
    Start,
    Waiting,
    Done,
}

// A task transitions through states each time the event loop resumes it.
```

Use cases to test:

- A task that yields three times eventually completes.
- Two yielding tasks interleave instead of one monopolizing the event loop.
- A yielded task can wait for a timer before continuing.
- Completed tasks are not resumed again.
- A normal callback task still executes while generator-style tasks are yielding.
