use rapid_recorder::prelude::*;
use rayon::prelude::*;
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
fn test_parallel_add() {
    // Test that we can add readings from multiple threads
    let recorder: RapidRecorder<_, TestReadings> = RapidRecorder::new(1000, 3);
    let mut group = recorder.add_group(
        RapidRecorderGroup::new()
            .sample_rate(DefaultSamplingFrequency::EveryOne)
            .index_type(DefaultIndexDimmension::Step),
    );

    group.start_record(1);

    // Add readings from multiple threads
    (0..100).into_par_iter().for_each(|i| {
        let reading = match i % 3 {
            0 => TestReadings::Reading0,
            1 => TestReadings::Reading1,
            _ => TestReadings::Reading2,
        };
        group.add(reading, 5.0);
    });

    // Force save
    group.start_record(2);

    // Verify we have one event
    let event = recorder.convenient_pop().unwrap();
    assert_eq!(event.id, 1);

    // At least one reading should be present
    assert!(
        event.values.contains_key(&TestReadings::Reading0)
            || event.values.contains_key(&TestReadings::Reading1)
            || event.values.contains_key(&TestReadings::Reading2)
    );
}

#[test]
fn test_parallel_multi_level() {
    // Test nested parallel code similar to example
    let recorder: RapidRecorder<_, TestReadings> = RapidRecorder::new(1000, 3);
    let mut group = recorder.add_group(
        RapidRecorderGroup::new()
            .sample_rate(DefaultSamplingFrequency::EveryOne)
            .index_type(DefaultIndexDimmension::Step),
    );

    group.start_record(1);

    // Top-level parallel
    (0..3).into_par_iter().for_each(|j| {
        match j {
            0 => {
                group.add(TestReadings::Reading0, 5.0);
            }
            1 => {
                group.add(TestReadings::Reading1, 10.0);
            }
            2 => {
                // Nested parallel
                (0..3).into_par_iter().for_each(|k| match k {
                    0 => {
                        group.add(TestReadings::Reading0, 15.0);
                    }
                    1 => {
                        group.add(TestReadings::Reading1, 20.0);
                    }
                    _ => {
                        group.add(TestReadings::Reading2, 25.0);
                    }
                });
                group.add(TestReadings::Reading2, 30.0);
            }
            _ => {}
        }
    });

    // Force save
    group.start_record(2);

    // Verify we recorded something
    assert_eq!(recorder.raw_history().len(), 1);
}
