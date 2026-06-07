# Step 7: Async HTTP Future

**Builds on:** [Step 6 — Async TCP](06-async-tcp.md), [Steps 4–5](README.md)  
**Difficulty:** hard

## In this stage

You'll add `http_get(url).await` — plain HTTP GET built on step 6's async TCP. No TLS; you should be able to see request/response bytes on the wire.

## What you'll implement

- `http_get(url).await` returning status, headers, and body
- URL parsing for common `http://` forms
- Request formatting and response parsing

## Where to extend your code

- `src/http.rs` (new file)

## What to expect at the end

Run a local server: `python -m http.server 8080`

```rust
event_loop.spawn(async {
    let response = http_get("http://127.0.0.1:8080/").await.unwrap();
    assert_eq!(response.status, 200);
});

event_loop.spawn(async {
    let _ = http_get("http://127.0.0.1:8080/fast").await;
    let _ = http_get("http://127.0.0.1:8080/slow").await;
});

event_loop.run();
```

GET `/` returns status 200 with a body. Both spawns complete. Connection refused returns `Err`, not a panic.

## Hints

- Responses often arrive in multiple chunks — buffer until you can parse.
- `Connection: close` keeps body handling simple (read until EOF).

## Things to verify

- [ ] GET `/` from a local server returns status `200` and body
- [ ] Two concurrent requests both complete
- [ ] Request to a closed port reports an error
- [ ] `sleep().await` and `http_get().await` work in the same task
