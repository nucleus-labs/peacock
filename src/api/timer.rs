#![allow(unused_imports)]

use std::collections::HashMap;
use std::time::{SystemTime, Duration};

#[cfg(feature = "perf")]
pub type TimerManager = HashMap<&'static str, Timer>;
#[cfg(not(feature = "perf"))]
pub type TimerManager = crate::DisabledFeature;

pub struct Timer {
    time: SystemTime,
    durs: Vec<Duration>,
    sum: Duration,
}

impl Timer {
    #[cfg(feature = "perf")]
    pub fn push_now(&mut self) {
        let now = SystemTime::now();
        let dur = now.duration_since(self.time).unwrap();
        self.time = now;
        self.sum += dur;
        self.durs.push(dur);
    }

    #[cfg(not(feature = "perf"))]
    pub fn push_now(&mut self) { }

    pub fn get_avg(&self) -> Duration {
        self.sum / (self.durs.len() as u32)
    }

    pub fn get_times(&self) -> Vec<Duration> {
        todo!()
    }
}
