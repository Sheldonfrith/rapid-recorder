# RapidRecorder

A high-performance library for tracking internal variables in concurrent Rust applications.

## Features

- **High Performance**: Minimal overhead in performance-critical code paths
- **Thread-Safe**: Safe for multi-threaded and parallel applications
- **Configurable Sampling**: Record values at specified intervals, or on every index change
- **Flexible Indexing**: Index recordings by time, step, run, or custom dimensions

## Use Cases

- Recording thread-local and/or deeply-nested variables in performance-sensitive simulation code
- Tracking state in multi-threaded applications without disrupting execution flow

## Usage Example

```rust
use rapid_recorder::prelude::*;

#[repr(u32)]
pub enum MyReadings {
    Temperature,
    Pressure,
}
impl_rapid_recorder_named_usize!(MyReadings);

fn main() {
    // Initialize recorder
    let recorder: RapidRecorder<_, MyReadings> = RapidRecorder::new(1_000_000, 10);

    // Create a recording group
    let mut group = recorder.add_group(
        RapidRecorderGroup::new()
            .sample_rate(DefaultSamplingFrequency::EveryOne)
            .index_type(DefaultIndexDimmension::Step),
    );

    // Record values in a loop
    for step in 0..1000 {
        group.start_record(step);

        // Deep in your code, record values
        let temperature = calculate_temperature();
        group.add(MyReadings::Temperature, temperature);

        let pressure = calculate_pressure();
        group.add(MyReadings::Pressure, pressure);
    }

    // After processing, analyze the data
    let history = recorder.sorted_history();
    // Process history...
}
```

# Benchmarks:

Run `cargo run --example performance_test`

# Best Practices

### Recommended Usage Patterns

1. **Single Index Management:** Manage record indexes in a single thread, while adding readings from multiple threads

```rust
for i in 0..1000 { // single threaded loop
    group.start_record(i);  // Call start_record in main thread

    // It's safe to add readings from parallel threads, nested as deep as you want
    (0..10).into_par_iter().for_each(|_| {
        // Deep in parallel code
        group.add(MyReadings::Temperature, 42.0);
    });
}
```

2. **Different Sampling Rates:** Use multiple groups for different sampling needs

```rust
let mut high_frequency_group = recorder.add_group(
    RapidRecorderGroup::new()
        .sample_rate(DefaultSamplingFrequency::EveryOne)
        .index_type(DefaultIndexDimmension::Step),
);

let mut low_frequency_group = recorder.add_group(
    RapidRecorderGroup::new()
        .sample_rate(DefaultSamplingFrequency::EveryHundred)
        .index_type(DefaultIndexDimmension::Step),
);
```

3. **Multiple Index Dimensions:** Track different types of indexes

```rust
let mut step_group = recorder.add_group(
    RapidRecorderGroup::new()
        .index_type(DefaultIndexDimmension::Step),
);

let mut time_group = recorder.add_group(
    RapidRecorderGroup::new()
        .index_type(DefaultIndexDimmension::Time),
);
```

### Patterns to Avoid

1. **Multiple Values for Same Reading:** Adding the same reading name multiple times per record from different threads

```rust
// Not recommended - only one value will be recorded and we can't even guarantee it was the last value
(0..10).into_par_iter().for_each(|_| {
    group.add(MyReadings::Temperature, get_random_value());
});
```

2. **Distributing Primary Record Indexes:** Creating new groups in parallel threads

```rust
// Not recommended - poor performance
(0..100).into_par_iter().for_each(|i| {
    let mut group = recorder.add_group(...);
    group.start_record(i);
    // Record things...
});
```
