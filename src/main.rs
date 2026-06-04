mod event_loop;
use event_loop::EventLoop;

fn main() {
    let mut event_loop = EventLoop::new();

    event_loop.spawn(|| println!("first")); 
    event_loop.spawn(|| println!("second"));
    event_loop.run();
}
