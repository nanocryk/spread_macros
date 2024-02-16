/// Create a value of an anonymous type with provided fields whose types must be infered from their
/// usage. The anonymous type automatically derive `Copy`, `Clone`, `Debug`, `PartialEq` and `Eq` if
/// all fields do so. macro.
/// ```rust
/// # use nanotricks::anon;
/// let anon = anon!(
///     one: 1,
///     two: "two",
/// );
///
/// println!("{anon:?}");
/// // Anon { one: 1, two: "two" }
/// ```
///
/// It also supports spreading fields from other structs like the [`spread!`](crate::spread!) macro, however it
/// doesn't support the regular struct update syntax:
/// ```rust
/// # use nanotricks::anon;
/// let anon1 = anon! {
///     two: "two",
///     three: 3,
/// };
///
/// let anon2 = anon! {
///     one: 1,
///     [two, three]: ..anon1,
///     // the following is not supported:
///     // ..another_struct
/// };
///
/// println!("{anon2:?}");
/// // Anon { one: 1, two: "two", three: 3 }
/// ```
#[macro_export]
macro_rules! anon {
    (
        $(
            $field:ident: $value:expr,
        )*
        $(
            [ $($spread_field:ident),+ ]: .. $spread:expr,
        )*
    ) => {
        {
            #[allow(non_camel_case_types)]
            #[derive(Copy, Clone, Debug, PartialEq, Eq)]
            struct Anon
            <
                $( $field, )*
                $($( $spread_field, )+)*

            > {
                $($field: $field, )*
                $($( $spread_field: $spread_field, )+)*
            }

            $crate::spread!(
                Anon {
                    $( $field: $value, )*
                    $( [ $($spread_field),+ ]: .. $spread, )*
                }
            )
        }
    }
}