use crate::event_loop::with_current_loop;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::{Duration, Instant};

pub struct Sleep {
    deadline: Instant,
    registered: bool,
}

pub fn sleep(duration: Duration) -> Sleep {
    Sleep {
        deadline: Instant::now() + duration,
        registered: false,
    }
}

impl Future for Sleep {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        if Instant::now() >= self.deadline {
            return Poll::Ready(());
        }

        if !self.registered {
            let waker = cx.waker().clone();
            with_current_loop(|ptr| {
                assert!(!ptr.is_null(), "sleep() was polled outside the event loop");

                // SAFETY: CURRENT_LOOP is set for the duration of run_futures().
                unsafe {
                    (*ptr).register_wakeup(self.deadline, waker);
                }
            });
            self.get_mut().registered = true;
        }

        Poll::Pending
    }
}
