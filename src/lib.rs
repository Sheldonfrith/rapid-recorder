use crossbeam::queue::ArrayQueue;

use std::{collections::HashMap, sync::atomic::AtomicUsize};

use crate::event::{LightweightRREvent, RREvent};
use crate::group::RapidRecorderGroup;
use crate::group_handle::RapidRecorderGroupHandle;
use crate::latest_reading_holder::LatestReadingHolder;
use crate::named_usize::ValidRapidRecorderNamedUsize;

pub mod defaults;
pub mod event;
pub mod group;
pub mod group_handle;
pub mod latest_reading_holder;
pub mod named_usize;
pub mod prelude {
    pub use crate::RapidRecorder;
    pub use crate::defaults::{DefaultIndexDimmension, DefaultSamplingFrequency};
    pub use crate::group::RapidRecorderGroup;
    pub use crate::impl_rapid_recorder_named_usize;
    pub use crate::named_usize::ValidRapidRecorderNamedUsize;
}
pub struct RapidRecorder<
    IndexDimmension: ValidRapidRecorderNamedUsize,
    ReadingName: ValidRapidRecorderNamedUsize,
> {
    buffer: ArrayQueue<LightweightRREvent>,
    latest_readings: LatestReadingHolder,
    phantom: std::marker::PhantomData<IndexDimmension>,
    phantom2: std::marker::PhantomData<ReadingName>,
}
impl<IterationIndex: ValidRapidRecorderNamedUsize, ReadingName: ValidRapidRecorderNamedUsize>
    RapidRecorder<IterationIndex, ReadingName>
{
    pub fn new(max_history_length: usize, max_reading_types: usize) -> Self {
        Self {
            buffer: ArrayQueue::new(max_history_length),
            latest_readings: LatestReadingHolder::new(max_reading_types),
            phantom: std::marker::PhantomData,
            phantom2: std::marker::PhantomData,
        }
    }

    pub fn add_group<SampleRate: ValidRapidRecorderNamedUsize>(
        &self,
        group: RapidRecorderGroup<SampleRate, IterationIndex>,
    ) -> RapidRecorderGroupHandle<SampleRate, ReadingName, IterationIndex> {
        RapidRecorderGroupHandle::new(group, self)
    }
    /// Get a reference to the raw ArrayQueue buffer containing all recorded events. These events are NOT SORTED, either temporally or by index value. This is the fastest way to access the data, but it is up to the user to sort and filter it as needed.
    pub fn raw_history(&self) -> &ArrayQueue<LightweightRREvent> {
        &self.buffer
    }

    /// a bit more expensive than just getting the raw history and popping yourself, but the event returned by this has usizes converted back to the enum types for easier reading and matching
    pub fn convenient_pop(&self) -> Option<RREvent<ReadingName, IterationIndex>> {
        if let Some(event) = self.buffer.pop() {
            Some(event.to_rr_event())
        } else {
            None
        }
    }
    /// You will need to pass in your reading types enum as a generic parameter
    ///  This function is far slower that `unordered_history` because it has to sort through all the events and organize them by group and then sort them by id within each group.
    /// Included as a convenience function, since this is exactly what many users will want to do with the data anyways
    pub fn sorted_history(
        &self,
    ) -> HashMap<IterationIndex, Vec<RREvent<ReadingName, IterationIndex>>> {
        let mut organized: HashMap<IterationIndex, Vec<RREvent<ReadingName, IterationIndex>>> =
            HashMap::new();
        while let Some(event) = self.buffer.pop() {
            let rr_event: RREvent<ReadingName, IterationIndex> = event.to_rr_event();
            // This is a bit of a hack, but it works for now. We assume that the record_id is unique to each group and type combination.
            // In a real implementation, we would want to store this information in the LightweightRREvent itself.
            organized
                .entry(rr_event.iteration_index)
                .or_insert_with(Vec::new)
                .push(rr_event);
        }
        for events in organized.values_mut() {
            events.sort_by_key(|e| e.id);
        }
        organized
    }

    pub fn _add_reading(&self, variable_name: usize, value: f64) {
        self.latest_readings.set_value(variable_name, value);
    }
    #[inline(always)]
    pub fn _save_event(&self, index_type: usize, id: usize) {
        let readings: Vec<f64> = self.latest_readings.snapshot();
        let event = LightweightRREvent {
            record_id: id as usize,
            id_type: index_type,
            readings,
        };
        let _ = self.buffer.push(event);
    }
}
