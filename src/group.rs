use crate::{
    defaults::{DefaultIndexDimmension, DefaultSamplingFrequency},
    named_usize::ValidRapidRecorderNamedUsize,
};

pub struct RapidRecorderGroup<
    SampleRate: ValidRapidRecorderNamedUsize = DefaultSamplingFrequency,
    IndexDimmension: ValidRapidRecorderNamedUsize = DefaultIndexDimmension,
> {
    pub sample_rate: Option<SampleRate>,
    pub sample_rate_value: Option<usize>,
    pub index_type: Option<IndexDimmension>,
    pub index_type_value: Option<usize>,
}

impl<SamplingFrequency: ValidRapidRecorderNamedUsize, IndexDimmension: ValidRapidRecorderNamedUsize>
    RapidRecorderGroup<SamplingFrequency, IndexDimmension>
{
    pub fn new() -> Self {
        Self {
            sample_rate: None,
            sample_rate_value: None,
            index_type: None,
            index_type_value: None,
        }
    }
    pub fn sample_rate(
        self,
        rate: SamplingFrequency,
    ) -> RapidRecorderGroup<SamplingFrequency, IndexDimmension> {
        RapidRecorderGroup {
            sample_rate: Some(rate.clone()),
            sample_rate_value: Some(rate.into()),
            index_type: self.index_type,
            index_type_value: self.index_type_value,
        }
    }

    pub fn index_type(
        self,
        index_type: IndexDimmension,
    ) -> RapidRecorderGroup<SamplingFrequency, IndexDimmension> {
        RapidRecorderGroup {
            sample_rate: self.sample_rate,
            sample_rate_value: self.sample_rate_value,
            index_type: Some(index_type),
            index_type_value: Some(index_type.into()),
        }
    }
}
