use criterion::{Criterion, black_box, criterion_group, criterion_main};
use rapid_recorder::latest_reading_holder::LatestReadingHolder;
use rapid_recorder::prelude::*;
use strum_macros::EnumIter;

#[repr(u32)]
#[derive(EnumIter)]
enum BenchReadings {
    Reading0,
    Reading1,
    Reading2,
}
impl_rapid_recorder_named_usize!(BenchReadings);

fn bench_latest_reading_holder(c: &mut Criterion) {
    let holder = LatestReadingHolder::new(10);

    c.bench_function("LatestReadingHolder.set_value", |b| {
        let mut i = 0;
        b.iter(|| {
            holder.set_value(i % 10, 42.0);
            i = (i + 1) % 10;
        });
    });
}

fn bench_recorder_add(c: &mut Criterion) {
    let recorder: RapidRecorder<_, BenchReadings> = RapidRecorder::new(1000, 5);
    let mut group = recorder.add_group(
        RapidRecorderGroup::new()
            .sample_rate(DefaultSamplingFrequency::EveryOne)
            .index_type(DefaultIndexDimmension::Step),
    );

    // First call (not measured)
    group.start_record(0);

    c.bench_function("RapidRecorderGroupHandle.add", |b| {
        let mut i = 0;
        b.iter(|| {
            let reading = match i % 3 {
                0 => BenchReadings::Reading0,
                1 => BenchReadings::Reading1,
                _ => BenchReadings::Reading2,
            };
            group.add(reading, i as f64);
            i = (i + 1) % 3;
        });
    });
}

fn bench_start_record(c: &mut Criterion) {
    let recorder: RapidRecorder<_, BenchReadings> = RapidRecorder::new(1000, 5);
    let mut group = recorder.add_group(
        RapidRecorderGroup::new()
            .sample_rate(DefaultSamplingFrequency::EveryOne)
            .index_type(DefaultIndexDimmension::Step),
    );

    // First call already done in setup
    group.start_record(0);
    group.add(BenchReadings::Reading0, 42.0);

    c.bench_function("start_record", |b| {
        let mut i = 1;
        b.iter(|| {
            group.start_record(i);
            i += 1;
        });
    });
}

criterion_group!(
    benches,
    bench_latest_reading_holder,
    bench_recorder_add,
    bench_start_record
);
criterion_main!(benches);
