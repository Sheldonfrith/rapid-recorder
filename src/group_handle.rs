use crate::{RapidRecorder, group::RapidRecorderGroup, named_usize::ValidRapidRecorderNamedUsize};

pub struct RapidRecorderGroupHandle<
    'a,
    SamplingFrequency: ValidRapidRecorderNamedUsize,
    ReadingName: ValidRapidRecorderNamedUsize,
    IndexDimmension: ValidRapidRecorderNamedUsize,
> {
    group: RapidRecorderGroup<SamplingFrequency, IndexDimmension>,
    recorder: &'a RapidRecorder<IndexDimmension, ReadingName>,
    sample_rate: usize,
    index_value: Option<usize>,
    should_save_next: bool,
}

impl<
    'a,
    SamplingFrequency: ValidRapidRecorderNamedUsize,
    ReadingName: ValidRapidRecorderNamedUsize,
    IndexDimmension: ValidRapidRecorderNamedUsize,
> RapidRecorderGroupHandle<'a, SamplingFrequency, ReadingName, IndexDimmension>
{
    pub fn new(
        group: RapidRecorderGroup<SamplingFrequency, IndexDimmension>,
        recorder: &'a RapidRecorder<IndexDimmension, ReadingName>,
    ) -> Self {
        let sample_rate = group.sample_rate_value.unwrap();
        Self {
            group,
            recorder,
            sample_rate,
            index_value: None,
            should_save_next: true,
        }
    }
    #[inline(always)]
    pub fn add(&self, reading_name: ReadingName, value: f64) {
        if self.should_save_next {
            self.recorder._add_reading(reading_name.into(), value);
        }
    }
    /// This causes the previously started record to be saved, and starts a new record with the given unique_id. If you do not call this method, no records will be saved.
    /// The final record will be saved when the RapidRecorderGroupHandle is dropped, no need to call this method at the end of your recording loop manually.
    /// This should be called in the same thread where the group was created
    pub fn start_record(&mut self, unique_id: usize) {
        if self.should_save_next {
            self._save_record();
        }
        self.index_value = Some(unique_id);
        if self.sample_rate > 1 {
            self.should_save_next = self.index_value.unwrap().is_multiple_of(self.sample_rate);
        }
    }
    /// Not recommended to call this manually...
    /// You ONLY need to call this function manually if you are retrieving the history
    /// DURING iterations, and you want to make sure the data from the currently in-progress iteration is included
    /// Note that if you call this and then start another record then there will be two records in the history with the same index value, but not necessarily the same readings
    /// So you will have to account for that when processing the history
    pub fn _save_record(&self) {
        if self.index_value.is_some() {
            self.recorder._save_event(
                self.group.index_type_value.unwrap(),
                self.index_value.unwrap(),
            );
        }
    }
}

/// Required otherwise the last event in a group will never be saved
impl<
    'a,
    SamplingFrequency: ValidRapidRecorderNamedUsize,
    ReadingName: ValidRapidRecorderNamedUsize,
    IndexDimmension: ValidRapidRecorderNamedUsize,
> Drop for RapidRecorderGroupHandle<'a, SamplingFrequency, ReadingName, IndexDimmension>
{
    fn drop(&mut self) {
        self._save_record();
    }
}
