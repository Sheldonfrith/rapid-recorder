use crate::impl_rapid_recorder_named_usize;

#[repr(u32)]
pub enum DefaultIndexDimmension {
    Time,
    Step,
    Run,
    I,
    J,
    K,
    PositionX,
    PositionY,
    PositionZ,
    Frame,
}
impl_rapid_recorder_named_usize!(DefaultIndexDimmension);
#[repr(u32)]

pub enum DefaultSamplingFrequency {
    EveryOne = 1,
    EveryTen = 10,
    EveryHundred = 100,
    EveryThousand = 1000,
    EveryTenThousand = 10000,
    EveryHundredThousand = 100000,
    EveryMillion = 1000000,
}
impl_rapid_recorder_named_usize!(DefaultSamplingFrequency);
