use log::info;
use rapid_recorder::prelude::*;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

#[repr(u32)]
pub enum ExampleReadingNames {
    InternalVariable0,
    InternalVariable1,
    InternalVariable2,
    InternalVariable3,
    InternalVariable4,
}
impl_rapid_recorder_named_usize!(ExampleReadingNames);

pub fn main() {
    let rapid_recorder = RapidRecorder::new(1_000_000, 10);

    let mut rapid_recorder_group_1 = rapid_recorder.add_group(
        RapidRecorderGroup::new()
            .sample_rate(DefaultSamplingFrequency::EveryOne)
            .index_type(DefaultIndexDimmension::Step),
    );

    // Not Recommended Usage

    // Adding same reading name multiple times per record:
    // performance will be good, but the value recorded will be inconsistent, as only one value will be recorded per reading name (whichever is the last thread to write it)
    for i in 0..1000 {
        rapid_recorder_group_1.start_record(i);
        (0..10).into_par_iter().for_each(|j| {
            // do work
            let internal_variable_0 = 5.0;
            rapid_recorder_group_1.add(ExampleReadingNames::InternalVariable0, internal_variable_0)

            // do work
        });
    }

    // Distributing primary record indexes across multiple threads:
    // Its not designed for this, so performance will be poor
    (0..100).into_par_iter().for_each(|i| {
        let mut rapid_recorder_group_some = rapid_recorder.add_group(
            RapidRecorderGroup::new()
                .sample_rate(DefaultSamplingFrequency::EveryOne)
                .index_type(DefaultIndexDimmension::Step),
        );
        rapid_recorder_group_some.start_record(i);
        (0..100).into_par_iter().for_each(|j| {
            // do work
            // record things
            // etc.
        });
    });
}
