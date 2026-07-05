use std::time::{Duration, Instant};

use crate::prelude::FPS;

pub struct FrameTimer {
    target_frame: Duration,
    next_frame: Instant,

    // FPS tracking
    frame_count: usize,
    last_time: Instant,
}

impl FrameTimer {
    pub fn new() -> Self {
        let now = Instant::now();
        let target_frame = Duration::from_secs_f64(1.0 / FPS);
        let next_frame = now + target_frame;

        Self {
            target_frame,
            next_frame,
            frame_count: 0,
            last_time: now,
        }
    }

    pub fn register_frame(&mut self) {
        self.frame_count += 1;
        let now = Instant::now();
        let dt = now.duration_since(self.last_time).as_secs_f64();

        if dt >= 1.0 {
            let fps = self.frame_count as f64 / dt;

            println!("FPS : {fps}");

            self.frame_count = 0;
            self.last_time = now;
        }
    }

    pub fn step(&mut self) {
        self.next_frame += self.target_frame;
    }

    pub fn should_update(&self) -> bool {
        Instant::now() >= self.next_frame
    }

    pub fn get_next_frame(&self) -> Instant {
        self.next_frame
    }

    pub fn delta_time(&self) -> f64 {
        self.target_frame.as_secs_f64()
    }
}
