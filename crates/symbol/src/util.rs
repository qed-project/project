#![allow(dead_code)]

/// check if the variants of an enum are equal
/// this dos not care about the values of an enum field
pub fn variant_eq<T>(lhs: &T, rhs: &T) -> bool {
    std::mem::discriminant::<T>(lhs) == std::mem::discriminant::<T>(rhs)
}
