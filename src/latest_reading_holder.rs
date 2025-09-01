use atomic_float::AtomicF64;
use std::sync::atomic::{AtomicBool, Ordering};
pub struct LatestReadingHolder {
    // pub segment_size: usize,
    // pub readings: Vec<Mutex<Vec<f64>>>,
    pub readings: Vec<AtomicF64>,
    pub changed: Vec<AtomicBool>,
}
impl LatestReadingHolder {
    pub fn new(capacity: usize) -> Self {
        Self {
            readings: (0..capacity).map(|_| AtomicF64::new(0.0)).collect(),
            changed: (0..capacity).map(|_| AtomicBool::new(false)).collect(),
        }
    }

    #[inline(always)]
    pub fn set_value(&self, index: usize, value: f64) {
        self.readings[index].store(value, Ordering::Relaxed);
        self.changed[index].store(true, Ordering::Relaxed);
    }

    pub fn snapshot(&self) -> (Vec<f64>, Vec<bool>) {
        (
            self.readings
                .iter()
                .map(|a| a.load(Ordering::Relaxed))
                .collect(),
            self.changed
                .iter()
                .map(|a| a.swap(false, Ordering::Relaxed))
                .collect(),
        )
    }
}
