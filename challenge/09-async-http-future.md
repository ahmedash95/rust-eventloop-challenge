# Step 9: Async HTTP Future

Expose HTTP as an awaitable operation by combining sockets, polling, futures, and wakeups.

## What To Implement

- `http_get(url) -> impl Future<Output = Result<Response, Error>>`
- An internal state machine for connecting, writing the request, reading the response, and completing.
- Socket registration with the reactor when an operation would block.
- Waker storage so readiness events wake the correct task.
- Response buffering and parsing reused from the callback HTTP client where possible.

## Requirements

- `http_get(url).await` returns a response without blocking the event loop.
- Multiple HTTP futures can run concurrently.
- Socket readiness should wake only tasks that can make progress.
- Partial writes and partial reads remain correct.
- Errors are returned through the future output.

## Test-Oriented Examples

```rust
event_loop.spawn(async {
    let response = http_get("http://127.0.0.1:8080/").await.unwrap();
    assert_eq!(response.status, 200);
});

event_loop.run();
```

Use cases to test:

- Awaiting one local HTTP request returns the expected status and body.
- Awaiting two requests concurrently completes both.
- A slow response does not block a fast response.
- Connection failure resolves to an error.
- A response body split across several readiness events is assembled correctly.
