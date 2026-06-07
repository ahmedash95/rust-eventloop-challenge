# Challenge Steps

Build one `EventLoop` across all steps. Each stage adds a single capability; earlier behavior must keep working.

Recommended order:

1. [01-minimal-task-queue.md](01-minimal-task-queue.md)
2. [02-timers.md](02-timers.md)
3. [03-reactor-foundation.md](03-reactor-foundation.md)
4. [04-tiny-executor.md](04-tiny-executor.md)
5. [05-sleep-future.md](05-sleep-future.md)
6. [06-async-tcp.md](06-async-tcp.md)
7. [07-async-http-future.md](07-async-http-future.md)
8. [08-mini-async-event-loop.md](08-mini-async-event-loop.md)
9. [09-bonus-challenges.md](09-bonus-challenges.md)

## How each step is written

| Section | Purpose |
|---|---|
| **In this stage** | Goal — what you add and why |
| **What you'll implement** | Capabilities and APIs to build |
| **Where to extend your code** | Which files to touch |
| **Concepts** | Terms worth knowing (when relevant) |
| **What to expect at the end** | Usage you should be able to write, and behavior you should see |
| **Hints** | Short tips for common mistakes |
| **Things to verify** | Checklist before moving on |

You implement everything in `src/` by hand. Steps describe outcomes, not full solutions.

## Learning path

Steps 1–2 build the scheduler and timers. Step 3 adds the mio reactor. Steps 4–5 introduce futures, wakers, and `sleep().await`. Steps 6–7 add async TCP and HTTP. Step 8 integrates everything into a mini-Tokio MVP with shutdown, timeout, and join.
