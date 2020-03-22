use std::time::{Duration, Instant};
pub struct SmallWorld {
    timer: Instant,
    pub clock: Duration,
}

impl SmallWorld {
    pub fn new(clock: Duration) -> SmallWorld {
        SmallWorld {
            timer: Instant::now(),
            clock: clock,
        }
    }
    pub fn update(&mut self) {
        if self.timer.elapsed() > self.clock {
            println!("small world hello!");
            self.timer = Instant::now();
        }
    }
}
