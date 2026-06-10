# Design issues — post step 5 review

Tracked improvements from the step 5 implementation review. These are **not blockers** for marking step 5 complete; they are follow-ups to harden the runtime before and during later phases.

## Suggested order

| # | Issue | Priority | Best time to tackle |
|---|-------|----------|---------------------|
| 01 | [Thread-local raw pointer bridge](01-thread-local-raw-pointer-bridge.md) | medium | Before step 6 — reduces footguns when adding TCP futures |
| 02 | [Noop waker + full-scan scheduling](02-noop-waker-full-scan-scheduling.md) | medium | Before or during step 6 — real wake-driven scheduling |
| 05 | [`ensure_work` ignores pending futures](05-ensure-work-ignores-pending-futures.md) | medium | Required for step 6 — prevents busy-spin on I/O |
| 03 | [Waker not updated on re-poll](03-waker-not-updated-on-repoll.md) | low → medium | After issue 02 |
| 04 | [Sleep timer not cancelled on drop](04-sleep-timer-not-cancelled-on-drop.md) | low → medium | Step 8 (join / shutdown) |
| 06 | [Sleep deadline at construction](06-sleep-deadline-at-construction.md) | low | Anytime — small semantic fix |

## Dependency graph

```text
02 (real wakers / ready queue)
 └── 03 (update waker on re-poll)
 └── 04 (timer cancel — shares task ids)

01 (remove raw pointer bridge) — independent, do before 6

05 (ensure_work + mio wait) — needed for step 6 I/O

06 — independent
```

## Issue template

Each file includes:

- **Problem** — what is wrong or fragile today
- **Why it matters** — when it becomes user-visible
- **Suggested solution** — concrete approaches with tradeoffs
- **Acceptance criteria** — how to know it is done
