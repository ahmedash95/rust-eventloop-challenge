mod event_loop;
mod sleep;
mod timer;

use event_loop::EventLoop;
use sleep::sleep;

use std::time::Duration;

fn main() {
    let mut event_loop = EventLoop::new();

    event_loop.spawn(|| println!("first"));
    event_loop.spawn_async(async {
        println!("async before");
        sleep(Duration::from_millis(800)).await;
        println!("async after");
    });
    event_loop.set_timeout(Duration::from_millis(400), || {
        println!("timeout after 400ms")
    });
    event_loop.set_timeout(Duration::from_millis(50), || println!("timeout after 50ms"));
    event_loop.set_timeout(Duration::from_millis(100), || {
        println!("timeout after 100s")
    });
    event_loop.spawn(|| println!("second"));
    event_loop.run();

    // expected output:
    // first
    // second
    // timeout after 50ms
    // timeout after 100ms
    // timeout after 400ms
}
