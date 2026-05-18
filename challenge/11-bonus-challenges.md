# Step 11: Bonus Challenges

Extend the event loop once the core educational version works.

## Easy Extensions

- Recurring intervals.
- Task IDs.
- Event loop shutdown signals.
- Cancellation.
- Graceful socket closing.

## Medium Extensions

- Task priorities.
- Async file reads.
- `select` and `join` combinators.
- Timeout futures.

## Test-Oriented Examples

Use cases to test:

- An interval fires repeatedly until cancelled.
- A task ID can be used to cancel a pending task.
- Event loop shutdown stops accepting new work or drains existing work according to your chosen policy.
- `timeout(duration, future).await` returns a timeout error when the inner future is too slow.
- `join(a, b).await` completes only after both futures complete.
- `select(a, b).await` completes when the first future completes and handles the loser predictably.

## Design Notes

Define semantics before implementing each bonus feature. For example, cancellation should specify whether dropped tasks run cleanup, whether timers are removed immediately, and what happens to registered sockets.
