use std::time::{Duration, Instant};

pub struct FpsLimiter {
    timepoint: Instant,
    max_fps: f32,
}

impl FpsLimiter {
    pub fn new(max_fps: f32) -> Self {
        Self {
            timepoint: Instant::now(),
            max_fps,
        }
    }

    pub fn start(&mut self) {
        self.timepoint = Instant::now();
    }

    pub fn wait(&self) {
        let ms_per_frame: u128 = (1000.0 / self.max_fps) as u128;
        let delta = self.timepoint.elapsed().as_millis();
        if delta < ms_per_frame {
            let remaining_time = ms_per_frame - delta;
            std::thread::sleep(Duration::from_millis(remaining_time as u64));
        }
    }
}
