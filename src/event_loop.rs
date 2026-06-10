use crate::timer::{Timer, TimerKind};
use mio::{Events, Poll};
use std::cell::Cell;
use std::collections::{BinaryHeap, VecDeque};
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Waker};
use std::time::{Duration, Instant};

thread_local! {
    static CURRENT_LOOP: Cell<*mut EventLoop> = const {
        Cell::new(std::ptr::null_mut())
    };
}

pub(crate) fn set_current_loop(loop_ptr: *mut EventLoop) {
    CURRENT_LOOP.with(|ev| ev.set(loop_ptr));
}

pub(crate) fn with_current_loop<F, R>(f: F) -> R
where
    F: FnOnce(*mut EventLoop) -> R,
{
    CURRENT_LOOP.with(|ev| f(ev.get()))
}

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
        self.timers.push(Timer {
            duration: Instant::now() + _duration,
            kind: TimerKind::Callback(Box::new(_f)),
        });
    }

    pub(crate) fn register_wakeup(&mut self, deadline: Instant, waker: Waker) {
        self.timers.push(Timer {
            duration: deadline,
            kind: TimerKind::Waker(waker),
        });
    }

    fn run_timers(&mut self) {
        let now = Instant::now();
        while self.timers.peek().is_some_and(|t| t.duration <= now) {
            let timer = self.timers.pop().unwrap();
            match timer.kind {
                TimerKind::Callback(f) => f(),
                TimerKind::Waker(waker) => waker.wake(),
            }
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

        let timeout = self
            .timers
            .peek()
            .map(|t| t.duration.saturating_duration_since(Instant::now()));

        // check timers
        if timeout.is_some_and(|d| d > Duration::ZERO) {
            // todo: avoid allocating events every time
            // 1024 capacity is default value. keep it for now
            let mut events = Events::with_capacity(1024);
            self.poll.poll(&mut events, timeout).unwrap();
        }
    }

    fn run_futures(&mut self) {
        set_current_loop(self as *mut Self);
        let waker = Waker::noop();
        let mut cx = Context::from_waker(&waker);
        self.futures
            .retain_mut(|future| future.as_mut().poll(&mut cx).is_pending());
        set_current_loop(std::ptr::null_mut());
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
