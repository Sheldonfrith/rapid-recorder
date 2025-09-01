use std::hash::Hash;

use strum::IntoEnumIterator;

pub trait ValidRapidRecorderNamedUsize:
    Into<usize>
    + From<usize>
    + Clone
    + Eq
    + PartialEq
    + Hash
    + Copy
    + std::fmt::Debug
    + IntoEnumIterator
{
}

/// Implements the necessary traits for an enum to be used with RapidRecorder.
///
/// # Safety
///
/// The enum MUST be marked as so:
/// ```rust
/// use strum_macros::EnumIter;
/// use rapid_recorder::prelude::*;
///
/// #[repr(u32)]
/// #[derive(EnumIter)]
/// enum MyEnum {
///     A,
///     B,
///     C,
/// }
/// impl_rapid_recorder_named_usize!(MyEnum);
/// ```
///
/// to ensure stable discriminant values.
/// Without this representation, the enum's memory layout is not guaranteed, which would
/// make the transmute operations in this macro unsafe and potentially cause undefined behavior.
///
/// # Example
///
/// ```
/// use rapid_recorder::prelude::*;
/// use strum_macros::EnumIter;
/// #[repr(u32)]
/// #[derive(strum_macros::EnumIter)]
/// pub enum MyReadings {
///     Temperature,
///     Pressure,
/// }
/// impl_rapid_recorder_named_usize!(MyReadings);
/// ```
#[macro_export]
macro_rules! impl_rapid_recorder_named_usize {
    ($enum_type:ty) => {
        #[allow(clippy::from_over_into)]
        impl Into<usize> for $enum_type {
            fn into(self) -> usize {
                self as usize
            }
        }
        /// Make sure you've got `#[repr(u32)]` on your enum!
        impl From<usize> for $enum_type {
            fn from(value: usize) -> Self {
                // Use a match statement or if-else structure for safe conversion
                unsafe { std::mem::transmute(value as u32) }
            }
        }
        /// Make sure you've got `#[repr(u32)]` on your enum!
        #[allow(clippy::non_canonical_clone_impl)]
        impl Clone for $enum_type {
            fn clone(&self) -> Self {
                // Get the discriminant safely without moving from self
                unsafe {
                    let ptr = self as *const Self;
                    let value = *(ptr as *const u32);
                    std::mem::transmute(value)
                }
            }
        }
        impl Copy for $enum_type {}
        impl PartialEq for $enum_type {
            fn eq(&self, other: &Self) -> bool {
                // Compare the discriminants safely without moving from self or other
                unsafe {
                    let self_ptr = self as *const Self;
                    let other_ptr = other as *const Self;
                    let self_value = *(self_ptr as *const u32);
                    let other_value = *(other_ptr as *const u32);
                    self_value == other_value
                }
            }
        }
        impl Eq for $enum_type {}
        impl std::fmt::Debug for $enum_type {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                // Use the default debug formatting
                write!(f, "{:?}", *self)
            }
        }
        impl std::hash::Hash for $enum_type {
            fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                // Hash the discriminant safely without moving from self
                unsafe {
                    let ptr = self as *const Self;
                    let value = *(ptr as *const u32);
                    value.hash(state);
                }
            }
        }

        impl $crate::prelude::ValidRapidRecorderNamedUsize for $enum_type {}
    };
}

/// Validates that an enum is compatible with RapidRecorder and returns its variant count
pub fn validate_enum_for_recorder<T: ValidRapidRecorderNamedUsize>() -> Result<usize, String> {
    let variant_count = T::iter().count();

    // Test conversion for each variant
    for variant in T::iter() {
        let as_usize: usize = variant.into();
        let back_to_enum = T::from(as_usize);

        // Verify roundtrip conversion works
        if variant != back_to_enum {
            return Err(format!(
                "Conversion error: {:?} converted to {} and back to {:?}. These should be identical.",
                variant, as_usize, back_to_enum
            ));
        }

        // Check that variant value is within expected range
        if as_usize >= variant_count {
            return Err(format!(
                "Variant {:?} has discriminant {} which is >= the enum's variant count {}. \
                Ensure your variants have sequential values starting from 0.",
                variant, as_usize, variant_count
            ));
        }
    }

    Ok(variant_count)
}

#[cfg(test)]
mod tests {
    use super::*;
    use strum::IntoEnumIterator;
    use strum_macros::EnumIter;
    #[repr(u32)]
    #[derive(EnumIter)]
    enum TestEnum {
        A,
        B = 5,
        C,
    }
    impl_rapid_recorder_named_usize!(TestEnum);

    #[test]
    fn test_trait_implementations() {
        // Test Into<usize>
        assert_eq!(<TestEnum as Into<usize>>::into(TestEnum::A), 0 as usize);
        assert_eq!(<TestEnum as Into<usize>>::into(TestEnum::B), 5 as usize);
        assert_eq!(<TestEnum as Into<usize>>::into(TestEnum::C), 6 as usize); // C comes after B which is 5

        // Test From<usize>
        assert_eq!(TestEnum::from(0), TestEnum::A);
        assert_eq!(TestEnum::from(5), TestEnum::B);
        assert_eq!(TestEnum::from(6), TestEnum::C);

        // Test Clone
        let a = TestEnum::A;
        let cloned = a.clone();
        assert_eq!(a, cloned);

        // Test equality
        assert_eq!(TestEnum::A, TestEnum::A);
        assert_ne!(TestEnum::A, TestEnum::B);

        // Test hashing (indirectly via Hash trait)
        use std::collections::HashMap;
        let mut map = HashMap::new();
        map.insert(TestEnum::A, "A");
        map.insert(TestEnum::B, "B");

        assert_eq!(map.get(&TestEnum::A), Some(&"A"));
        assert_eq!(map.get(&TestEnum::B), Some(&"B"));
    }
}
