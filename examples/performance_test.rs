use log::info;
use rapid_recorder::latest_reading_holder::LatestReadingHolder;
use rapid_recorder::prelude::*;
use std::hint::black_box;
use std::time::{Duration, Instant};
use strum_macros::EnumIter;

#[repr(u32)]
#[derive(EnumIter)]
pub enum ExampleReadingNames {
    InternalVariable0,
    InternalVariable1,
    InternalVariable2,
    InternalVariable3,
    InternalVariable4,
}
impl_rapid_recorder_named_usize!(ExampleReadingNames);

const ITERATIONS: u64 = 1_000_000;
const READING_NAMES_MAX: usize = 5000;

pub fn main() {
    println!("Running performance tests...");

    // Test 1: LatestValueHolder.set_value
    let latest_value_holder_time = {
        let holder = LatestReadingHolder::new(READING_NAMES_MAX);

        let start = Instant::now();
        for i in 0..ITERATIONS {
            holder.set_value((i as usize % READING_NAMES_MAX) as usize, 5.0);
            black_box(&holder);
        }
        let duration = start.elapsed();

        Duration::from_nanos(duration.as_nanos() as u64 / ITERATIONS)
    };

    // Test 2: RapidRecorderGroupHandle.add
    let add_time = {
        let recorder: RapidRecorder<_, ExampleReadingNames> =
            RapidRecorder::new(1_000_000, READING_NAMES_MAX);
        let mut group = recorder.add_group(
            RapidRecorderGroup::new()
                .sample_rate(DefaultSamplingFrequency::EveryOne)
                .index_type(DefaultIndexDimmension::Step),
        );

        // First start_record call (not measured)
        group.start_record(0);

        let start = Instant::now();
        for i in 0..ITERATIONS {
            match i % 5 {
                0 => group.add(ExampleReadingNames::InternalVariable0, i as f64),
                1 => group.add(ExampleReadingNames::InternalVariable1, i as f64),
                2 => group.add(ExampleReadingNames::InternalVariable2, i as f64),
                3 => group.add(ExampleReadingNames::InternalVariable3, i as f64),
                _ => group.add(ExampleReadingNames::InternalVariable4, i as f64),
            };
            black_box(&group);
        }
        let duration = start.elapsed();

        Duration::from_nanos(duration.as_nanos() as u64 / ITERATIONS)
    };

    // Test 3: First start_record call
    let first_start_record_time = {
        let recorder: RapidRecorder<_, ExampleReadingNames> =
            RapidRecorder::new(1_000_000, READING_NAMES_MAX);
        let mut group = recorder.add_group(
            RapidRecorderGroup::new()
                .sample_rate(DefaultSamplingFrequency::EveryOne)
                .index_type(DefaultIndexDimmension::Step),
        );

        let start = Instant::now();
        group.start_record(0);
        let duration = start.elapsed();

        black_box(&group);
        duration
    };

    // Test 4: Subsequent start_record calls
    let subsequent_start_record_time = {
        let recorder: RapidRecorder<_, ExampleReadingNames> =
            RapidRecorder::new(1_000_000, READING_NAMES_MAX);
        let mut group = recorder.add_group(
            RapidRecorderGroup::new()
                .sample_rate(DefaultSamplingFrequency::EveryOne)
                .index_type(DefaultIndexDimmension::Step),
        );

        // First call (not measured)
        group.start_record(0);
        group.add(ExampleReadingNames::InternalVariable0, 5.0);

        let start = Instant::now();
        for i in 1..ITERATIONS + 1 {
            group.start_record(i as usize);
            black_box(&group);
        }
        let duration = start.elapsed();

        Duration::from_nanos(duration.as_nanos() as u64 / ITERATIONS)
    };

    // Report results
    println!("\nResults:");
    println!(
        "Raw LatestValueHolder.set_value: {:?} per call",
        latest_value_holder_time
    );
    println!("RapidRecorderGroupHandle.add: {:?} per call", add_time);
    println!("First start_record call: {:?}", first_start_record_time);
    println!(
        "Subsequent start_record calls: {:?} per call",
        subsequent_start_record_time
    );
    println!(
        "Only accounting for the `start_record` calls, if you were running a loop at 120 fps, that would be an accumulated time of {:?} per second, which would reduce the fps by about {:.2}. If you were running a simulation at hourly resolution for a 50 year period (50*365*24 = 438000 simulation steps) that would be an accumulated time of {:?} added time to the runtime due to the `start_record`.",
        subsequent_start_record_time.mul_f64(120.0),
        (subsequent_start_record_time.mul_f64(120.0).as_millis() as f64 * (120.0 / 1000.0)),
        subsequent_start_record_time.mul_f64(438000.0)
    );
}
