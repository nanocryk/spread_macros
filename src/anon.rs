#[doc(hidden)]
#[macro_export]
macro_rules! anon_inner {
    (
        $(
            $field:ident $(: $value:expr)? ,
        )*
        $(
            $( #[ $($spread_attr:tt)* ] )?
            {
                $(
                    $( #[ $($spread_field_attr:tt)* ] )?
                    $spread_field:ident
                ),+
            } in
            $spread:expr,
        )*
    ) => {
        {
            #[allow(non_camel_case_types)]
            #[derive(Copy, Clone, Debug, PartialEq, Eq)]
            #[cfg_attr(feature = "serde", derive($crate::serde::Serialize))]
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
                    $( $field $(: $value)? , )*
                    $(
                        $( #[ $( $spread_attr )* ] )?
                        {
                            $(
                                $( #[ $( $spread_field_attr )* ] )?
                                $spread_field
                            ),+
                        }
                        in $spread,
                    )*
                }
            )
        }
    }
}

/// Create a value of an anonymous struct with provided fields whose types are inferred. The anonymous
/// type automatically derive `Copy`, `Clone`, `Debug`, `PartialEq` and `Eq` if all fields do so.
/// ```rust
/// # use nanotweaks::anon;
/// let anon = anon!(
///     one: 1,
///     two: "two",
/// );
///
/// println!("{anon:?}");
/// // Anon { one: 1, two: "two" }
/// ```
///
/// It also supports spreading fields from other structs like the [`spread!`](crate::spread!) macro
/// (like lists and `#[clone]`), however it doesn't support the regular struct update syntax:
/// ```rust
/// # use nanotweaks::anon;
/// let two = "two";
/// let anon1 = anon! {
///     two,
///     three: 3,
/// };
///
/// let anon2 = anon! {
///     one: 1,
///     { two, three } in anon1,
///     // the following is not supported:
///     // ..another_struct
/// };
///
/// println!("{anon2:?}");
/// // Anon { one: 1, two: "two", three: 3 }
/// ```
#[macro_export]
macro_rules! anon {
    ($($inner:tt)*) => { $crate::anon_inner!($($inner)*)}
}
