use crate::timer::Timer;
use std::collections::{BinaryHeap, VecDeque};
use std::time::Instant;

pub struct EventLoop {
    queue: VecDeque<Box<dyn FnOnce()>>,
    timers: BinaryHeap<Timer>,
}

impl EventLoop {
    pub fn new() -> Self {
        EventLoop {
            queue: VecDeque::new(),
            timers: BinaryHeap::new(),
        }
    }

    pub fn spawn<F>(&mut self, f: F)
    where
        F: FnOnce() + 'static,
    {
        self.queue.push_back(Box::new(f));
    }

    pub fn set_timeout(&mut self, _duration: std::time::Duration, _f: impl FnOnce() + 'static) {
        let timer = Timer {
            duration: Instant::now() + _duration,
            callback: Box::new(_f),
        };

        self.timers.push(timer);
    }

    fn run_timers(&mut self) {
        let now = Instant::now();
        while self.timers.peek().is_some_and(|t| t.duration <= now) {
            let timer = self.timers.pop().unwrap();
            (timer.callback)();
        }
    }

    fn run_tasks(&mut self) {
        while let Some(task) = self.queue.pop_front() {
            task();
        }
    }

    // Blocks until work becomes available.
    //
    // Checks for ready tasks, expired timers, and other pending events.
    // If no work is currently ready, the thread sleeps until the next
    // scheduled wake-up time (e.g. the nearest timer expiry).
    fn ensure_work(&mut self) {
        // check tasks first
        if !self.queue.is_empty() {
            return;
        }

        // check timers
        if let Some(timer) = self.timers.peek() {
            let now = Instant::now();
            if timer.duration > now {
                let sleep_duration = timer.duration - now;
                std::thread::sleep(sleep_duration);
            }
        }
    }

    fn all_done(&self) -> bool {
        self.queue.is_empty() && self.timers.is_empty()
    }

    pub fn run(&mut self) {
        loop {
            self.ensure_work();
            self.run_timers();
            self.run_tasks();

            if self.all_done() {
                break;
            }
        }
    }
}
