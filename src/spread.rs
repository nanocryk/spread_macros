#[doc(hidden)]
#[macro_export]
macro_rules! spread_inner {
    // top level syntax
    ($name:ident {
        $(
            $field:ident $(: $field_value:expr)? ,
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
        $(.. $remainder:expr)?
    }) => {
        {
            $crate::paste!{
                // each field will get its own let binding, which allows to parse
                // the attributes.
                $(
                    $crate::spread_inner!(#field [<__ $field >] $field $(: $field_value)? );
                )*

                $(
                    $crate::spread_inner!(
                        #spread
                        {
                            $( #[ $( $spread_attr )* ] )?
                            [< $( _ $spread_field )+ >]
                            $spread
                        }
                        {
                            $(
                                $( #[ $( $spread_field_attr )* ] )?
                                [<__ $spread_field>]
                                $spread_field
                            ),+
                        }
                    );
                )*

                // we then construct the struct using the bindings
                $name {
                    $( $field: [<__ $field>], )*
                    $( $( $spread_field: [< __ $spread_field>], )+ )*
                    $(.. $remainder)?
                }
            }
        }
    };

    // single field with value
    (#field $__field:ident $field:ident : $value:expr) => {
        let $__field = $value;
    };
    // single field without value
    (#field $__field:ident $field:ident) => {
        let $__field = $field;
    };

    // ----- spread with #[clone] on source
    // it will automatically clone all fields

    // spread with source expr, with bind it first so that the expr is noteevaluated
    // multiple times
    (#spread {#[clone] $__spread:ident $spread:expr} { $($tail:tt)* }) => {
        let $__spread = $spread;
        $crate::spread_inner!(#spread #[clone] $__spread { $($tail)* });
    };
    // no more fields
    (#spread #[clone] $__spread:ident { }) => ();
    // spread one field
    (#spread #[clone] $__spread:ident { $__field:ident $field:ident $( , $($tail:tt)* )? }) => (
        let $__field = $__spread . $field . clone();
        $crate::spread_inner!(#spread #[clone] $__spread { $( $( $tail )* )? })
    );

    // ----- spread with no #[clone] on source

    // spread with source expr, with bind it first so that the expr is note evaluated
    // multiple times
    (#spread {$__spread:ident $spread:expr} { $($tail:tt)* }) => {
        let $__spread = $spread;
        $crate::spread_inner!(#spread $__spread { $($tail)* });
    };
    // no more fields
    (#spread $__spread:ident { }) => ();
    // spread one field
    (#spread $__spread:ident { $__field:ident $field:ident $( , $($tail:tt)* )? }) => (
        let $__field = $__spread . $field;
        $crate::spread_inner!(#spread $__spread { $( $( $tail )* )? })
    );
    // spread one field with clone
    (#spread $__spread:ident { #[clone] $__field:ident $field:ident $( , $($tail:tt)* )? }) => (
        let $__field = $__spread . $field . clone();
        $crate::spread_inner!(#spread $__spread { $( $( $tail )* )? })
    );
}

/// Extension of the spread/[struct update syntax] that allow taking fields from different type
/// structs, as long as the listed fields have the same type in both structs.
///
/// Its main goal is to be used to provide partial defaults for a struct that can't fully provide a
/// default value for all its fields.
///
/// [struct update syntax]:
///     https://doc.rust-lang.org/book/ch05-01-defining-structs.html#creating-instances-from-other-instances-with-struct-update-syntax
///
/// The syntax is the following:
/// ```rust
/// # use nanotweaks::spread;
/// # struct Struct {}
/// let v = spread!(Struct {
///     // list of fields
/// });
/// ```
///
/// This list starts with zero to many fields with or without values:
/// ```rust
/// # use nanotweaks::spread;
/// struct Struct { field: u32 }
/// let field = 42;
/// let v = spread!(Struct {
///     field,
/// });
/// ```
///
/// ```rust
/// # use nanotweaks::spread;
/// struct Struct { field: u32 }
/// let v = spread!(Struct {
///     field: 42,
/// });
/// ```
///
/// It can then be followed by zero to many lists of fields to take from other structs.
/// ```rust
/// # use nanotweaks::{anon,spread};
/// struct Struct { field: u32 }
/// let source = anon! { field: 42, _unused: 0, };
/// let v = spread!(Struct {
///     { field } in source,
/// });
/// ```
///
/// Note here that `source` is consumed if it is not `Copy`. You can use `&source` to extract `Copy`
/// fields, but it will not work if one of the listed fields is not `Copy`.
///
/// While you make use `source.clone()`, it clones the entire source struct which might not be what
/// you want or even possible. For that reason, the macro allow to prefix fields with `#[clone]` to
/// only clone that particular field.
/// ```rust,compile_fail
/// # use nanotweaks::{anon,spread};
/// struct Struct { field: String }
/// let source = anon! { field: String::from("foo"), };
/// let v = spread!(Struct {
///     { field } in source,
/// });
/// println!("{source:?}"); // this will not compile as `source.field` has been moved
/// ```
///
/// ```rust
/// # use nanotweaks::{anon,spread};
/// struct Struct { field: String }
/// let source = anon! { field: String::from("foo"), };
/// let v = spread!(Struct {
///     { #[clone] field } in &source,
/// });
/// println!("{source:?}"); // this compiles as `source.field` is cloned
/// ```
///
/// If all listed fields should be cloned, `#[clone]` can be placed before `{` instead.
/// ```rust
/// # use nanotweaks::{anon,spread};
/// struct Struct { field: String, field2: String }
/// let source = anon! { field: String::from("foo"), field2: String::from("foo"),};
/// let v = spread!(Struct {
///     #[clone] { field, field2 } in &source,
/// });
/// println!("{source:?}");
/// ```
///
/// Finally, it can be ended with the normal Rust's [struct update syntax]:
/// ```rust
/// # use nanotweaks::{anon,spread};
/// struct Struct { field: String, foo: u32, bar: u32}
/// let default = Struct { field: String::from("default"), foo: 1, bar: 2 };
/// let source = anon! { field: String::from("foo"), };
/// let v = spread!(Struct {
///     foo: 42,
///     { #[clone] field } in &source,
///     ..default
/// });
/// println!("{source:?}");
/// ```
///
/// All single fields must be listed first, followed by all spread structs. Each must be ended by a
/// `,`, even if not followed by something else. Rust's [struct update syntax] can only appear once
/// at the end and cannot be followed by a `,`.

#[macro_export]
macro_rules! spread {
    ($($inner:tt)*) => { $crate::spread_inner!($($inner)*)}
}
