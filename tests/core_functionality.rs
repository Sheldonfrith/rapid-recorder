use rapid_recorder::{RRDuplicateEventIdHandling, prelude::*};
use std::thread;
use std::time::Duration;
use strum_macros::EnumIter;

#[repr(u32)]
#[derive(EnumIter)]

enum TestReadings {
    Reading0,
    Reading1,
    Reading2,
}
impl_rapid_recorder_named_usize!(TestReadings);

#[test]
fn test_basic_recording() {
    // Initialize recorder
    let recorder: RapidRecorder<_, TestReadings> = RapidRecorder::new(1000, 3);
    let mut group = recorder.add_group(
        RapidRecorderGroup::new()
            .sample_rate(DefaultSamplingFrequency::EveryOne)
            .index_type(DefaultIndexDimmension::Step),
    );

    // Record some values
    group.start_record(1);
    group.add(TestReadings::Reading0, 10.0);
    group.add(TestReadings::Reading1, 20.0);

    // Force saving by starting a new record
    group.start_record(2);

    // Now check that values were recorded
    let event = recorder.convenient_pop().unwrap();
    assert_eq!(event.id, 1);
    assert_eq!(event.values.get(&TestReadings::Reading0), Some(&10.0));
    assert_eq!(event.values.get(&TestReadings::Reading1), Some(&20.0));
}

#[test]
fn test_sampling_frequency() {
    let recorder: RapidRecorder<_, TestReadings> = RapidRecorder::new(1000, 3);

    // Test EveryOne sampling rate
    let mut every_one = recorder.add_group(
        RapidRecorderGroup::new()
            .sample_rate(DefaultSamplingFrequency::EveryOne)
            .index_type(DefaultIndexDimmension::Step),
    );

    // Test EveryTen sampling rate
    let mut every_ten = recorder.add_group(
        RapidRecorderGroup::new()
            .sample_rate(DefaultSamplingFrequency::EveryTen)
            .index_type(DefaultIndexDimmension::Step),
    );

    // Record 20 events
    for i in 0..20 {
        every_one.start_record(i);
        every_one.add(TestReadings::Reading0, i as f64);

        every_ten.start_record(i);
        every_ten.add(TestReadings::Reading0, i as f64);
    }
    every_one._save_record();
    every_ten._save_record();
    // Get history and sort it
    let sorted =
        recorder.sorted_history_with_duplicate_handling(RRDuplicateEventIdHandling::KeepOnlyFirst);

    // Count records for each index type
    let mut one_count = 0;
    let mut ten_count = 0;

    for (_, events) in sorted {
        for event in events {
            if event.id < 20 {
                if event.id % 10 == 0 {
                    // These should be recorded by both groups
                    ten_count += 1;
                }
                one_count += 1;
            }
        }
    }

    // We should have all 20 events from EveryOne group
    assert_eq!(one_count, 20);

    // We should have 2 events from EveryTen group (0 and 10)
    assert_eq!(ten_count, 2);
}
