mod clone;
mod spread;

/// Standalone [`Default::default()`] function.
pub fn default<T: Default>() -> T { T::default() }