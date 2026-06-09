use crate::timer::Timer;
use std::collections::{BinaryHeap, VecDeque};
use std::time::{Duration, Instant};
use mio::{Events, Poll};
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Waker};


pub struct EventLoop {
    poll: Poll,
    queue: VecDeque<Box<dyn FnOnce()>>,
    futures: Vec<Pin<Box<dyn Future<Output = ()> + 'static>>>,
    timers: BinaryHeap<Timer>,
}

impl EventLoop {
    pub fn new() -> Self {
        EventLoop {
            poll: Poll::new().expect("failed to create poll"),
            queue: VecDeque::new(),
            futures: Vec::new(),
            timers: BinaryHeap::new(),
        }
    }

    pub fn spawn<F>(&mut self, f: F)
    where
        F: FnOnce() + 'static,
    {
        self.queue.push_back(Box::new(f));
    }

    pub fn spawn_async<F>(&mut self, f: F)
    where
        F: Future<Output = ()> + 'static,
    {
        self.futures.push(Box::pin(f));
    }


    pub fn set_timeout(&mut self, _duration: Duration, _f: impl FnOnce() + 'static) {
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

        let timeout = self.timers.peek().map(|t| {
            t.duration.saturating_duration_since(Instant::now())
        });

        // check timers
        if timeout.is_some_and(|d| d > Duration::ZERO) {
            // todo: avoid allocating events every time
            // 1024 capacity is default value. keep it for now
            let mut events = Events::with_capacity(1024);
            self.poll.poll(&mut events, timeout).unwrap();
        }
    }

    fn run_futures(&mut self) {
         let waker = Waker::noop();
         let mut cx = Context::from_waker(&waker);
         self.futures.retain_mut(|future| {
             future.as_mut().poll(&mut cx).is_pending()
         });
     }

    fn all_done(&self) -> bool {
        self.queue.is_empty() && self.timers.is_empty() && self.futures.is_empty()
    }

    pub fn run(&mut self) {
        loop {
            self.ensure_work();
            self.run_timers();
            self.run_tasks();
            self.run_futures();

            if self.all_done() {
                break;
            }
        }
    }
}
