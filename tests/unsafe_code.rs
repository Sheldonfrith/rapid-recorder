use rapid_recorder::prelude::*;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use strum_macros::EnumIter;

// Test enum with standard sequential variants
#[repr(u32)]
#[derive(EnumIter)]

enum BasicEnum {
    Variant0,
    Variant1,
    Variant2,
}
impl_rapid_recorder_named_usize!(BasicEnum);

// Test enum with explicit discriminants
#[repr(u32)]
#[derive(EnumIter)]

enum ExplicitEnum {
    VariantA = 5,
    VariantB = 10,
    VariantC = 42,
}
impl_rapid_recorder_named_usize!(ExplicitEnum);

#[test]
fn test_into_usize_conversion() {
    // Test basic sequential enum
    assert_eq!(
        <BasicEnum as Into<usize>>::into(BasicEnum::Variant0),
        0usize
    );
    assert_eq!(
        <BasicEnum as Into<usize>>::into(BasicEnum::Variant1),
        1usize
    );
    assert_eq!(
        <BasicEnum as Into<usize>>::into(BasicEnum::Variant2),
        2usize
    );

    // Test enum with explicit discriminants
    assert_eq!(
        <ExplicitEnum as Into<usize>>::into(ExplicitEnum::VariantA),
        5usize
    );
    assert_eq!(
        <ExplicitEnum as Into<usize>>::into(ExplicitEnum::VariantB),
        10usize
    );
    assert_eq!(
        <ExplicitEnum as Into<usize>>::into(ExplicitEnum::VariantC),
        42usize
    );
}

#[test]
fn test_from_usize_conversion() {
    // Test basic sequential enum
    assert!(matches!(BasicEnum::from(0usize), BasicEnum::Variant0));
    assert!(matches!(BasicEnum::from(1usize), BasicEnum::Variant1));
    assert!(matches!(BasicEnum::from(2usize), BasicEnum::Variant2));

    // Test enum with explicit discriminants
    assert!(matches!(ExplicitEnum::from(5usize), ExplicitEnum::VariantA));
    assert!(matches!(
        ExplicitEnum::from(10usize),
        ExplicitEnum::VariantB
    ));
    assert!(matches!(
        ExplicitEnum::from(42usize),
        ExplicitEnum::VariantC
    ));
}

#[test]
fn test_clone_impl() {
    let original1 = BasicEnum::Variant1;
    let cloned1 = original1.clone();
    assert_eq!(original1, cloned1);

    let original2 = ExplicitEnum::VariantB;
    let cloned2 = original2.clone();
    assert_eq!(original2, cloned2);
}

#[test]
fn test_eq_impl() {
    // Same variants should be equal
    assert_eq!(BasicEnum::Variant1, BasicEnum::Variant1);
    assert_eq!(ExplicitEnum::VariantB, ExplicitEnum::VariantB);

    // Different variants should not be equal
    assert_ne!(BasicEnum::Variant0, BasicEnum::Variant1);
    assert_ne!(ExplicitEnum::VariantA, ExplicitEnum::VariantC);
}

#[test]
fn test_hash_impl() {
    // Test hash consistency for same variant
    let mut hasher1 = DefaultHasher::new();
    let mut hasher2 = DefaultHasher::new();

    BasicEnum::Variant1.hash(&mut hasher1);
    BasicEnum::Variant1.hash(&mut hasher2);

    assert_eq!(hasher1.finish(), hasher2.finish());

    // Test hash differs for different variants
    let mut hasher1 = DefaultHasher::new();
    let mut hasher2 = DefaultHasher::new();

    BasicEnum::Variant0.hash(&mut hasher1);
    BasicEnum::Variant1.hash(&mut hasher2);

    assert_ne!(hasher1.finish(), hasher2.finish());
}

#[test]
fn test_roundtrip_conversion() {
    // Test that into->from conversions preserve the value
    let variants = [
        BasicEnum::Variant0,
        BasicEnum::Variant1,
        BasicEnum::Variant2,
    ];

    for variant in variants {
        let as_usize: usize = variant.into();
        let back_to_enum = BasicEnum::from(as_usize);
        assert_eq!(variant, back_to_enum);
    }

    let variants = [
        ExplicitEnum::VariantA,
        ExplicitEnum::VariantB,
        ExplicitEnum::VariantC,
    ];

    for variant in variants {
        let as_usize: usize = variant.into();
        let back_to_enum = ExplicitEnum::from(as_usize);
        assert_eq!(variant, back_to_enum);
    }
}

// test that passing the wrong count corresponding to the enum panics with "doesn't match the actual number of enum variants"
#[test]
#[should_panic(expected = "doesn't match the actual number of enum variants")]
fn test_enum_variant_count_mismatch() {
    let _recorder: RapidRecorder<DefaultIndexDimmension, BasicEnum> = RapidRecorder::new(100, 5);
}

// Test with an enum with missing repr(u32)
// throws a compiler error, so its safe
// enum DataEnum {
//     A = 0,
//     B = 5,
// }
// impl_rapid_recorder_named_usize!(DataEnum);

// #[test]
// fn test_data_enum() {
//     let a = DataEnum::A;
//     let b = DataEnum::B;

//     // Test discriminants
//     assert_eq!(<DataEnum as Into<usize>>::into(a), 0usize);
//     assert_eq!(<DataEnum as Into<usize>>::into(b), 5usize);

//     // Test from -> into roundtrip
//     // (just for discriminant, not data)
//     let reconstructed_a = DataEnum::from(0usize);
//     assert_eq!(<DataEnum as Into<usize>>::into(reconstructed_a), 0usize);
// }

// Test with an enum containing data
// already throws compiler error, so no need to test
// #[repr(u32)]
// enum DataEnum {
//     A(i32) = 5,
//     B { value: f64 } = 10,
//     C = 15,
// }
// impl_rapid_recorder_named_usize!(DataEnum);

// #[test]
// fn test_data_enum() {
//     let a = DataEnum::A(42);
//     let b = DataEnum::B { value: 3.14 };
//     let c = DataEnum::C;

//     // Test discriminants
//     assert_eq!(a.into(), 5usize);
//     assert_eq!(b.into(), 10usize);
//     assert_eq!(c.into(), 15usize);

//     // Test from -> into roundtrip
//     // (just for discriminant, not data)
//     let reconstructed_a = DataEnum::from(5usize);
//     assert_eq!(reconstructed_a.into(), 5usize);
// }
