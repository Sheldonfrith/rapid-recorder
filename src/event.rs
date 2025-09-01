use std::collections::HashMap;

use crate::named_usize::ValidRapidRecorderNamedUsize;

pub struct RawRREvent {
    pub readings: Vec<f64>,
    pub changed: Vec<bool>,
    pub record_id: usize,
    pub id_type: usize,
}
impl RawRREvent {
    pub fn to_rr_event<
        ReadingName: ValidRapidRecorderNamedUsize,
        IndexDimmension: ValidRapidRecorderNamedUsize,
    >(
        &self,
    ) -> RREvent<ReadingName, IndexDimmension> {
        let mut values_map = HashMap::new();
        for (i, value) in self.readings.iter().enumerate() {
            if !self.changed[i] {
                continue;
            }
            values_map.insert(ReadingName::from(i), *value);
        }
        RREvent {
            values: values_map,
            id: self.record_id,
            iteration_index: IndexDimmension::from(self.id_type),
        }
    }
}
pub struct RREvent<
    ReadingName: ValidRapidRecorderNamedUsize,
    IndexDimmension: ValidRapidRecorderNamedUsize,
> {
    pub values: HashMap<ReadingName, f64>,
    pub id: usize,
    pub iteration_index: IndexDimmension,
}
