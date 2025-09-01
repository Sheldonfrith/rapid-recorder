use std::hash::Hash;

pub trait ValidRapidRecorderNamedUsize:
    Into<usize> + From<usize> + Clone + Eq + PartialEq + Hash + Copy
{
}

/// Implements the necessary traits for an enum to be used with RapidRecorder.
///
/// # Safety
///
/// The enum MUST be marked with `#[repr(u32)]` to ensure stable discriminant values.
/// Without this representation, the enum's memory layout is not guaranteed, which would
/// make the transmute operations in this macro unsafe and potentially cause undefined behavior.
///
/// # Example
///
/// ```
/// use rapid_recorder::prelude::*;
///
/// #[repr(u32)]
/// pub enum MyReadings {
///     Temperature,
///     Pressure,
/// }
/// impl_rapid_recorder_named_usize!(MyReadings);
/// ```
#[macro_export]
macro_rules! impl_rapid_recorder_named_usize {
    ($enum_type:ty) => {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[repr(u32)]
    #[derive(Debug)]
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
