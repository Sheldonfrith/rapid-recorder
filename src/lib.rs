use crossbeam::queue::ArrayQueue;

use std::collections::HashMap;

use crate::event::{RREvent, RawRREvent};
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

pub enum RRDuplicateEventIdHandling {
    KeepOnlyFirst,
    KeepOnlyLast,
    KeepBoth,
}

/// The main recorder for storing events and managing recording groups.
///
/// RapidRecorder is responsible for tracking internal variables efficiently in
/// concurrent code, with minimal performance impact.
///
/// # Type Parameters
///
/// * `IndexDimmension` - The type used for indexing records, should be an enum that maps to usize
/// * `ReadingName` - The type used for naming readings, should be an enum that maps to usize
pub struct RapidRecorder<
    IndexDimmension: ValidRapidRecorderNamedUsize,
    ReadingName: ValidRapidRecorderNamedUsize,
> {
    buffer: ArrayQueue<RawRREvent>,
    latest_readings: LatestReadingHolder,
    phantom: std::marker::PhantomData<IndexDimmension>,
    phantom2: std::marker::PhantomData<ReadingName>,
}
impl<IterationIndex: ValidRapidRecorderNamedUsize, ReadingName: ValidRapidRecorderNamedUsize>
    RapidRecorder<IterationIndex, ReadingName>
{
    pub fn new(max_history_length: usize, max_reading_types: usize) -> Self {
        // In debug mode, validate the enum
        #[cfg(debug_assertions)]
        {
            use crate::named_usize::validate_enum_for_recorder;

            let actual_count = match validate_enum_for_recorder::<ReadingName>() {
                Ok(count) => count,
                Err(err) => panic!("ReadingName enum validation failed: {}", err),
            };

            if max_reading_types != actual_count {
                panic!(
                    "max_reading_types ({}) doesn't match the actual number of enum variants ({}).\n\
                    You should pass the exact number of variants in your enum to avoid memory safety issues.",
                    max_reading_types, actual_count
                );
            }
        }
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
    ) -> RapidRecorderGroupHandle<'_, SampleRate, ReadingName, IterationIndex> {
        RapidRecorderGroupHandle::new(group, self)
    }
    /// Get a reference to the raw ArrayQueue buffer containing all recorded events. These events are NOT SORTED, either temporally or by index value. This is the fastest way to access the data, but it is up to the user to sort and filter it as needed.
    /// There are commonly duplicate events, you will have to handle that yourself
    pub fn raw_history(&self) -> &ArrayQueue<RawRREvent> {
        &self.buffer
    }

    /// a bit more expensive than just getting the raw history and popping yourself, but the event returned by this has usizes converted back to the enum types for easier reading and matching
    pub fn convenient_pop(&self) -> Option<RREvent<ReadingName, IterationIndex>> {
        self.buffer.pop().map(|e| e.to_rr_event())
    }
    /// Returns a map of iteration indices to events, sorted by event ID within each iteration.
    ///
    /// This function organizes all recorded events by their iteration index and sorts them by ID.
    /// It's slower than `unordered_history` because it must process and sort all events.
    ///
    /// There is no filtering of duplicate event IDs, you will have to handle that yourself, or use sorted_history_with_duplicate_handling instead
    ///
    /// # Returns
    /// A HashMap where:
    /// - Keys are iteration indices
    /// - Values are vectors of events (sorted by event ID) that occurred during that iteration
    pub fn sorted_history(
        &self,
    ) -> HashMap<IterationIndex, Vec<RREvent<ReadingName, IterationIndex>>> {
        let mut organized: HashMap<IterationIndex, Vec<RREvent<ReadingName, IterationIndex>>> =
            HashMap::new();

        while let Some(event) = self.buffer.pop() {
            let rr_event = event.to_rr_event();
            organized
                .entry(rr_event.iteration_index)
                .or_default()
                .push(rr_event);
        }

        // Sort events by ID within each iteration
        for events in organized.values_mut() {
            events.sort_by_key(|e| e.id);
        }

        organized
    }

    /// Returns a map of iteration indices to events with control over how duplicate event IDs are handled.
    ///
    /// If you never pass the same index value to start_record more than once during an iteration, and you never manuall call `RapidRecorderGroupHandle::_save_record` then this function behaves the same as sorted_history
    ///
    /// # Arguments
    /// * `duplicate_event_id_handling` - Specifies how to handle events with duplicate IDs:
    ///   - `KeepOnlyFirst`: Retains only the first occurrence of each event ID
    ///   - `KeepOnlyLast`: Retains only the most recent occurrence of each event ID
    ///   - `KeepBoth`: Keeps all events regardless of duplicate IDs
    ///
    /// # Returns
    /// A HashMap where:
    /// - Keys are iteration indices
    /// - Values are vectors of events (sorted by event ID) that occurred during that iteration,
    ///   with duplicate handling applied according to the specified strategy
    pub fn sorted_history_with_duplicate_handling(
        &self,
        duplicate_event_id_handling: RRDuplicateEventIdHandling,
    ) -> HashMap<IterationIndex, Vec<RREvent<ReadingName, IterationIndex>>> {
        let mut organized: HashMap<IterationIndex, Vec<RREvent<ReadingName, IterationIndex>>> =
            HashMap::new();

        while let Some(event) = self.buffer.pop() {
            let rr_event = event.to_rr_event();
            organized
                .entry(rr_event.iteration_index)
                .or_default()
                .push(rr_event);
        }

        for events in organized.values_mut() {
            // Handle duplicate event IDs according to the specified strategy
            match duplicate_event_id_handling {
                RRDuplicateEventIdHandling::KeepOnlyFirst => {
                    let mut seen_ids = std::collections::HashSet::new();
                    events.retain(|e| seen_ids.insert(e.id));
                }
                RRDuplicateEventIdHandling::KeepOnlyLast => {
                    let mut seen_ids = std::collections::HashSet::new();
                    events.reverse();
                    events.retain(|e| seen_ids.insert(e.id));
                    events.reverse();
                }
                RRDuplicateEventIdHandling::KeepBoth => {
                    // Keep all events, no filtering needed
                }
            }

            // Sort events by ID
            events.sort_by_key(|e| e.id);
        }

        organized
    }
    pub fn _add_reading(&self, variable_name: usize, value: f64) {
        self.latest_readings.set_value(variable_name, value);
    }
    #[inline(always)]
    pub fn _save_event(&self, index_type: usize, id: usize) {
        let (readings, changed): (Vec<f64>, Vec<bool>) = self.latest_readings.snapshot();
        let event = RawRREvent {
            record_id: id,
            id_type: index_type,
            readings,
            changed,
        };
        let _ = self.buffer.push(event);
    }
}
