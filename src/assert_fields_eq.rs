#[cfg(not(feature = "similar-asserts"))]
pub use core::assert_eq;
#[cfg(feature = "similar-asserts")]
pub use similar_asserts::assert_eq;

#[doc(hidden)]
#[macro_export]
macro_rules! assert_fields_eq_inner {
    // anon syntax
    (
        $left:expr,
        {
            $(
                $field:ident $(: $value:expr)? ,
            )*
            $(
                { $($spread_field:ident),+ } in $spread:expr,
            )*
        }
        $(, $($arg:tt)*)?
    ) => {
        let right = $crate::anon! {
            $($field $(: $value)? , )*
            $(
                { $($spread_field),+ } in &$spread,
            )*
        };

        assert_fields_eq!($left, right, [
            $( $field ,)*
            $( $($spread_field ,)+ )*
        ] $(, $($arg)*)?);
    };

    // list syntax
    (
        $left:expr,
        $right:expr,
        [$($field:ident),+ $(,)?]
        $(, $($arg:tt)*)?
    ) => {
        {
            #[allow(non_camel_case_types)]
            #[derive(Debug, PartialEq, Eq)]
            struct Fields
            <
                'a,
                $( $field, )+
            > {
                $( $field: &'a $field, )+
            }

            let left = &$left;
            let left = Fields {
                $($field: & (left . $field)),+
            };

            let right = &$right;
            let right = Fields {
                $($field: & (right . $field)),+
            };

            $crate::assert_fields_eq::assert_eq!(left, right $(, $($arg)*)?);
        }
    }
}

/// Asserts that some fields of the provided value match the expectation.
///
/// This expectation can be expressed in 2 ways:
/// - Another value can be provided, followed by a list of fields both values have in common
///   and should be equal.
/// - An anonymous struct with the same syntax as [`anon!`](crate::anon!).
///
/// Afterward, the macro accepts a custom panic message like [`assert_eq!`](core::assert_eq!).
///
/// Feature `similar-asserts` allows to internaly replace [`core::assert_eq!`] by
/// [`similar_asserts::assert_eq!`], which provides a pretty diff output.
///
/// [`similar_asserts::assert_eq!`]: https://docs.rs/similar-asserts/1.5.0/similar_asserts/macro.assert_eq.html
///
/// ```rust
/// # use nanotweaks::{anon, assert_fields_eq};
/// #[derive(Clone, Debug)]
/// struct Exemple {
///     _foo: u32,
///     bar: String,
///     baz: bool,
/// }
///
/// let exemple = Exemple {
///     _foo: 42,
///     bar: String::from("exemple"),
///     baz: true,
/// };
///
/// let expected = anon! {
///     bar: String::from("exemple"),
///     baz: true,
///     other: "other",
/// };
///
/// assert_fields_eq!(exemple, {
///     bar: String::from("exemple"),
///     { baz } in expected,
/// });
///
/// assert_fields_eq!(
///     exemple,
///     expected,
///     [bar, baz],
///     "unexpected fields in {exemple:?}"
/// );
/// ```
#[macro_export]
macro_rules! assert_fields_eq {
    ($($inner:tt)*) => { $crate::assert_fields_eq_inner!($($inner)*)}
}
