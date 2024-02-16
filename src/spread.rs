/// Extension of the spread/[struct update syntax] that allow taking fields from different type
/// structs, as long as the listed fields have the same type in both structs.
///
/// Its main goal is to be used to provide partial defaults for a struct that can't fully
/// provide a default value for all its fields.
///
/// [struct update syntax]: https://doc.rust-lang.org/book/ch05-01-defining-structs.html#creating-instances-from-other-instances-with-struct-update-syntax
///
/// ```rust
/// # use nanotricks::spread;
/// // `User` don't impl `Default` as they are no proper defaults for `username` and `password`.
/// #[derive(Debug, PartialEq, Eq)]
/// struct User {
///     name: String,
///     password: String,
///     dark_theme: bool,
///     prefered_terminal_font_size: u16,
/// }
///
/// // We can define other structs with default values.
/// // To use fields with the above struct they need to have the same name and
/// // type. There can be other fields.
/// struct DarkUserDefaults {
///     dark_theme: bool,
///     prefered_terminal_font_size: u16,
///     _test: bool,
/// }
///
/// impl Default for DarkUserDefaults {
///     fn default() -> Self {
///         Self {
///             dark_theme: true,
///             prefered_terminal_font_size: 16,
///             _test: true,
///         }
///     }
/// }
///
/// let name = String::from("name");
/// let user = spread!(User {
///     name,
///     password: String::from("password"),
///     [prefered_terminal_font_size, dark_theme]: ..DarkUserDefaults::default(),
/// });
///
/// assert_eq!(
///     user,
///     User {
///         name: String::from("name"),
///         password: String::from("password"),
///         dark_theme: true,
///         prefered_terminal_font_size: 16,
///     }
/// );
/// ```
#[macro_export]
macro_rules! spread {
    ($name:ident {
        $(
            $field:ident $(: $field_value:expr)? ,
        )*
        $(
            [ $($source_field:ident),+ ]: .. $source:expr,
        )*
        $(.. $remainder:expr)?
    }) => {
        $name {
            $($field $(: $field_value)?,)*
            $($(
                $source_field: $source . $source_field,
            )+)*
            $(.. $remainder)?
        }
    }
}