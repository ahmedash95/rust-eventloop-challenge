pub struct Timer {
    pub duration: std::time::Instant,
    pub callback: Box<dyn FnOnce() + 'static>,
}

impl PartialEq for Timer {
    fn eq(&self, other: &Self) -> bool {
        self.duration == other.duration
    }
}

impl Eq for Timer {}

impl PartialOrd for Timer {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.duration.cmp(&other.duration).reverse())
    }
}

impl Ord for Timer {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // as max heap has the largest element at the top, we reverse the order to get the smallest
        // duration at the top
        other.duration.cmp(&self.duration)
    }
}
