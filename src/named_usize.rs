use std::hash::Hash;

pub trait ValidRapidRecorderNamedUsize:
    Into<usize> + From<usize> + Clone + Eq + PartialEq + Hash + Copy
{
}
// Add this to your library
#[macro_export]
macro_rules! impl_rapid_recorder_named_usize {
    ($enum_type:ty) => {
        impl Into<usize> for $enum_type {
            fn into(self) -> usize {
                self as usize
            }
        }
        impl From<usize> for $enum_type {
            fn from(value: usize) -> Self {
                // Use a match statement or if-else structure for safe conversion
                unsafe { std::mem::transmute(value as u32) }
            }
        }
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
