use crate::shadow::smallworld::SmallWorld;
use std::time::{Duration, Instant};
pub struct BigWorld {
    timer: Instant,
    pub clock: Duration,
    smallworlds: Option<Vec<SmallWorld>>,
}

impl BigWorld {
    pub fn new(clock: Duration, smallworlds: Option<Vec<SmallWorld>>) -> BigWorld {
        BigWorld {
            timer: Instant::now(),
            clock: clock,
            smallworlds: smallworlds,
        }
    }
    pub fn update(&mut self) {
        if let Some(smallworlds) = &mut self.smallworlds {
            for smallworld in smallworlds {
                smallworld.update();
            }
        } else {
            println!("It's an empty world!");
        }
    }

    pub fn run(&mut self) {
        loop {
            if self.timer.elapsed() > self.clock {
                self.update();
                self.timer = Instant::now();
                println!("Big world");
            }
        }
    }
}
