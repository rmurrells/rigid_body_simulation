use std::{
    thread,
    time::{
	Duration,
	Instant,
    },
};

pub struct FPSManager {
    pub frame_duration: Duration,
    last: Instant,
}

impl FPSManager {
    pub fn new(fps: u64) -> Self {
        Self{
	    frame_duration: Duration::from_micros(1_000_000/fps),
            last: Instant::now(),
	}
    }
    
    pub fn sleep_to_next_frame(&mut self) {
        let elapsed = self.last.elapsed();
        if elapsed < self.frame_duration {
            thread::sleep(self.frame_duration-elapsed);
        }
        self.last = Instant::now();
    }
}
