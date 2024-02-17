mod anon;
mod clone;
mod spread;

// public to re-export `assert_eq!` from either `core` or `similar_asserts` based on the
// `similar-asserts` feature.
#[doc(hidden)]
pub mod assert_fields_eq;

/// Standalone [`Default::default()`] function.
pub fn default<T: Default>() -> T {
    T::default()
}
