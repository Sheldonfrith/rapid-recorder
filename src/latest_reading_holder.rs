use atomic_float::AtomicF64;
use rayon::iter::{
    IndexedParallelIterator, IntoParallelIterator, IntoParallelRefIterator, ParallelIterator,
};
use rayon::prelude::*;
use std::sync::{Mutex, atomic::Ordering};
pub struct LatestReadingHolder {
    // pub segment_size: usize,
    // pub readings: Vec<Mutex<Vec<f64>>>,
    pub readings: Vec<AtomicF64>,
}
impl LatestReadingHolder {
    pub fn new(capacity: usize) -> Self {
        Self {
            readings: (0..capacity).map(|_| AtomicF64::new(0.0)).collect(),
        }
    }

    #[inline(always)]
    pub fn set_value(&self, index: usize, value: f64) {
        self.readings[index].store(value, Ordering::Relaxed);
    }

    pub fn snapshot(&self) -> Vec<f64> {
        self.readings
            .iter()
            .map(|a| a.load(Ordering::Relaxed))
            .collect()
    }
}
