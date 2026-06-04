use std::collections::VecDeque;

pub struct EventLoop {
    queue: VecDeque<Box<dyn FnOnce()>>,
}

impl EventLoop {
    pub fn new() -> Self {
        EventLoop {
            queue: VecDeque::new(),
        }
    }

    pub fn spawn<F>(&mut self, f: F)
    where
        F: FnOnce() + 'static,
    {
        self.queue.push_back(Box::new(f));
    }

    pub fn run(&mut self) {
        while let Some(task) = self.queue.pop_front() {
            task();
        }
    }
}
