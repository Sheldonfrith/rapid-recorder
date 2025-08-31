use rapid_recorder::prelude::*;

#[repr(u32)] // must have this above any enum you pass to impl_rapid_recorder_named_usize!
pub enum ExampleReadingName {
    InternalVariable0,
    InternalVariable1,
    InternalVariable2,
}
// must use this macro for any enum you pass to RapidRecorder
impl_rapid_recorder_named_usize!(ExampleReadingName);

#[repr(u32)]
pub enum DifferentReadingName {
    InternalVariable3,
    InternalVariable4,
    InternalVariable5,
    InternalVariable6,
    InternalVariable7,
}
impl_rapid_recorder_named_usize!(DifferentReadingName);

pub fn main() {
    // initialize the history store
    // you must specify the reading name enum type here
    let rapid_recorder: RapidRecorder<_, ExampleReadingName> = RapidRecorder::new(1_000_000, 1);
    // creating 3 different groups for demonstration purposes
    // often you will only need one index dimmension, and maybe 1-3 different sample rates (so 1-3 groups)
    // create a group to help organize sample rates and indexes you are recording against
    let mut exhaustive_step_recording_group = rapid_recorder.add_group(
        RapidRecorderGroup::new()
            .sample_rate(DefaultSamplingFrequency::EveryOne)
            .index_type(DefaultIndexDimmension::Step),
    );
    let mut occasional_step_recording_group = rapid_recorder.add_group(
        RapidRecorderGroup::new()
            .sample_rate(DefaultSamplingFrequency::EveryHundred)
            .index_type(DefaultIndexDimmension::Step),
    );
    let mut occasional_run_recording_group = rapid_recorder.add_group(
        RapidRecorderGroup::new()
            .sample_rate(DefaultSamplingFrequency::EveryThousand)
            .index_type(DefaultIndexDimmension::Run),
    );

    for run in 0..10 {
        occasional_run_recording_group.start_record(run);
        for step in 0..100_000 {
            occasional_step_recording_group.start_record(run);
            exhaustive_step_recording_group.start_record(step);
            // enter deeply nested/parallel scopes here...
            // record internal variables without having to pass them around or return them
            let internal_variable_0 = 5.0;
            exhaustive_step_recording_group
                .add(ExampleReadingName::InternalVariable0, internal_variable_0);
            let internal_variable_1 = 10.0;
            occasional_run_recording_group
                .add(ExampleReadingName::InternalVariable1, internal_variable_1); // only records every 1000 RUNS
            let internal_variable_2 = 15.0;
            occasional_step_recording_group
                .add(ExampleReadingName::InternalVariable2, internal_variable_2); // only records every 100 STEPS
        }
    }
    // process the results
    let raw_history = rapid_recorder.raw_history(); // retrieves all saved observations, Will have to sort through them, as there is no guarantee of order and no filtering by group or type, since the point is to make runtime observation as cheap as possible
    let oldest_event = rapid_recorder.convenient_pop(); // a bit more expensive than just getting the raw history and popping yourself, but the event returned by this has usizes converted back to the enum types for easier reading and matching
    let mut sorted_history = rapid_recorder.sorted_history(); // retrieves all saved observations, sorted by group and type, and in order
}
