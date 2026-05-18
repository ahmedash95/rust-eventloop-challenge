# Step 5: Simple HTTP Client

Use the event loop to perform real asynchronous work over TCP.

## What To Implement

- `event_loop.http_get(url, callback)` or an equivalent callback-based API.
- URL parsing for at least plain `http://host/path`.
- TCP connection to port 80 unless a port is specified.
- HTTP/1.0 or HTTP/1.1 request writing.
- Response buffering and minimal response parsing.

TLS is not required for this step. Prefer plain HTTP so the event loop concepts stay visible.

## Requirements

- Multiple requests can be in flight at once.
- Waiting on one request must not block the event loop.
- The callback receives a response object or error.
- Response parsing should expose at least status code, headers, and body bytes/string.

## Test-Oriented Examples

```rust
event_loop.http_get("http://example.com/", |result| {
    let response = result.unwrap();
    assert_eq!(response.status, 200);
});

event_loop.run();
```

Use cases to test:

- GET `/` from a local test server returns status `200` and body content.
- Two concurrent requests both complete.
- A request to a closed port reports an error.
- A response split across multiple TCP reads is assembled correctly.
- A URL with an explicit port, such as `http://127.0.0.1:8080/hello`, connects to that port.
