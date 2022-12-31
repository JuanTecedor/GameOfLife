use std::time::{Duration, Instant};

pub struct FpsLimiter {
    timepoint: Instant,
    ms_per_frame: u128,
}

impl FpsLimiter {
    pub fn new(max_fps: f32) -> Self {
        Self {
            timepoint: Instant::now(),
            ms_per_frame: (1000.0 / max_fps) as u128,
        }
    }

    pub fn start(&mut self) {
        self.timepoint = Instant::now();
    }

    pub fn wait(&self) {
        let delta = self.timepoint.elapsed().as_millis();
        if delta < self.ms_per_frame {
            let remaining_time = self.ms_per_frame - delta;
            std::thread::sleep(Duration::from_millis(remaining_time as u64));
        }
    }
}
