use log::info;
use rapid_recorder::prelude::*;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::thread;
use std::time::Duration;

#[repr(u32)]
pub enum ExampleReadingNames {
    InternalVariable0,
    InternalVariable1,
    InternalVariable2,
    InternalVariable3,
}
impl_rapid_recorder_named_usize!(ExampleReadingNames);

pub fn main() {
    env_logger::init();
    let max_reading_types_to_track = 50; // Your ReadingNames enum should have no more than this many variants, the larger this value is the more overhead there is, but anything below 10k should still be plenty fast
    let rapid_recorder = RapidRecorder::new(1_000_000, max_reading_types_to_track);
    let mut rr_group = rapid_recorder.add_group(
        RapidRecorderGroup::new()
            .sample_rate(DefaultSamplingFrequency::EveryOne)
            .index_type(DefaultIndexDimmension::Step),
    );

    // Create a thread to show "UI" updates by periodically reading from recorder
    let ui_thread = thread::spawn(move || {
        for i in 0..10 {
            thread::sleep(Duration::from_millis(100));
            info!("UI Thread: Checking simulation status...");

            // In a real app, this would access the recorder and display its data
            // through a UI framework. Here we just log the current status.
            info!("UI Thread: Simulation running, iteration {}/10", i + 1);
        }
    });

    // Ideal use case:
    // parallelism within a single-thread primary record index
    // to keep track of internal variables deeply burried within high performance, parallel code
    // avoids having to expose, emit or return the internal variables in performance-sensitive code
    for i in 0..1000 {
        rr_group.start_record(i);
        // spawn threads
        (0..3).into_par_iter().for_each(|j| {
            match j {
                0 => {
                    // do work
                    let internal_variable_0 = 5.0;
                    rr_group.add(ExampleReadingNames::InternalVariable0, internal_variable_0)

                    // do work
                }
                1 => {
                    // do work
                    let internal_variable_1 = 5.0;
                    rr_group.add(ExampleReadingNames::InternalVariable1, internal_variable_1)

                    // do work
                }
                2 => {
                    // do work
                    let internal_variable_2 = 5.0;
                    rr_group.add(ExampleReadingNames::InternalVariable2, internal_variable_2)

                    // do work
                }
                3 => {
                    // spawn more threads
                    (0..3).into_par_iter().for_each(|k| {
                        match k {
                            0 => {
                                // do work
                                let internal_variable_3 = 5.0;
                                rr_group.add(
                                    ExampleReadingNames::InternalVariable3,
                                    internal_variable_3,
                                )

                                // do work
                            }
                            1 => {
                                // do work
                                let internal_variable_1 = 5.0;
                                rr_group.add(
                                    ExampleReadingNames::InternalVariable1,
                                    internal_variable_1,
                                )

                                // do work
                            }
                            2 => {
                                // do work
                                let internal_variable_2 = 5.0;
                                rr_group.add(
                                    ExampleReadingNames::InternalVariable2,
                                    internal_variable_2,
                                )

                                // do work
                            }
                            3 => {
                                // do work
                            }
                            _ => {
                                // Handle any other cases
                                // This branch should never execute with the current 0..3 range
                            }
                        }
                    });
                    let internal_variable_3 = 5.0;
                    rr_group.add(ExampleReadingNames::InternalVariable3, internal_variable_3)

                    // do work
                }
                _ => {
                    // Handle any other cases
                    // This branch should never execute with the current 0..3 range
                }
            }
        });

        // Simulate some work in the main thread
        if i % 200 == 0 {
            info!("Main thread: Simulation progress {}%", i / 10);
        }
    }

    // Wait for UI thread to complete
    ui_thread.join().unwrap();

    // Simulation complete, retrieve and process results
    info!("Simulation complete, processing results...");

    // Get raw history - fastest but unstructured
    let raw_history = rapid_recorder.raw_history();
    info!("Raw history entries: {}", raw_history.len());

    // Get one event for demonstration
    if let Some(event) = rapid_recorder.convenient_pop() {
        info!("Sample event - Step {}", event.id);
        for (var_name, value) in event.values.iter() {
            match var_name {
                ExampleReadingNames::InternalVariable0 => info!("  Variable 0: {}", value),
                ExampleReadingNames::InternalVariable1 => info!("  Variable 1: {}", value),
                ExampleReadingNames::InternalVariable2 => info!("  Variable 2: {}", value),
                ExampleReadingNames::InternalVariable3 => info!("  Variable 3: {}", value),
            }
        }
    }

    // Get sorted history - more structured but slower
    let sorted_history = rapid_recorder.sorted_history();
    info!("Total recorded steps: {}", sorted_history.len());

    // Process the data (in a real app, you might generate graphs, calculate statistics, etc.)
    info!("Analysis complete");
}
