/// Generates a struct representing the arguments of a function in order, with a `call` function to
/// actually call the function. The struct can be defined in 2 ways :
/// - fields are listed with a type and a default value, and `Default` is implemented using those
/// values. It however cannot depend on any lifetime parameter, which prevent using it with
/// `&self`/`&mut self` methods. It can however be combined with the [`partial!`](crate::partial!)
/// macro to only store non-references.
/// - only the name of the fields are listed and the generated struct is generic, and `Default` will
/// be derived automatically if all fields are `Default`. It can be used if the automatic derive
/// provides the wanted defaults or if defaults are not necessary. It also can be used with
/// multiple functions that have the same number of arguments but different types, however sharing
/// the same field name for arguments with different purposes may be confusing for readers.
///
/// The `call` function is generic on everything implementing [`FnOnce`] with proper arguments. The
/// return type of `call` is also generic, returning what the provided function returns.
///
/// It is targeted to be used when writing tests in which a function with many parameters is called
/// often and for which repeated arguments can be applied using [`spread!`](crate::spread!).
///
/// ```rust
/// use nanotweaks::{anon, partial, spread, fn_struct};
///
/// fn_struct!(NameOnly(vec, value));
/// fn_struct!(pub WithDefaults { value: &'static str = "hello" });
///
/// let mut v = vec!["foo"];
///
/// // Can store the arguments and use them afterward.
/// NameOnly {
///     vec: &mut v,
///     value: "bar",
/// }
/// .call(Vec::push);
///
/// // Called function can return a value.
/// // Type of the fields is generic, however the name of the fields may not be appropriate.
/// let item = NameOnly {
///     vec: &mut v,
///     value: 0,
/// }
/// .call(Vec::remove);
/// assert_eq!(item, "foo");
///
/// // You can even use it with lambdas.
/// NameOnly {
///     vec: &mut v,
///     value: "baz",
/// }
/// .call(|vec, v| vec.insert(0, v));
///
/// // The goal is of course to use with `spread!` or struct update syntax.
/// // Here there is only one argument, but it may come handy when writing tests
/// // with functions having many arguments.
/// let anon = anon! { value: "hey", };
/// spread!(NameOnly {
///     vec: &mut v,
///     { value } in anon,
/// })
/// .call(Vec::push);
///
/// // We can use `WithDefaults` with `partial!` to provide `&mut self`.
/// WithDefaults::default().call(partial!(Vec::push: &mut v, _));
/// WithDefaults { value: "bye" }.call(partial!(Vec::push: &mut v, _));
///
/// assert_eq!(v, vec!["baz", "bar", "hey", "hello", "bye"]);
/// ```
#[macro_export]
macro_rules! fn_struct {
    ($vis:vis $name:ident (
        $(
            $arg:ident
        ),+ $(,)?
    )) => (
        #[allow(non_camel_case_types)]
        #[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
        #[cfg_attr(feature = "serde", derive($crate::serde::Serialize, $crate::serde::Deserialize))]
        $vis struct $name < $( $arg ),+  > {
            $(
                $arg: $arg
            ),+
        }

        #[allow(non_camel_case_types)]
        impl< $( $arg ),+ > $name < $( $arg ),+ > {
            #[allow(dead_code)]
            pub fn call<F, R>(self, f: F) -> R
                where F: FnOnce( $( $arg ),+ ) -> R
            {
                let Self { $( $arg ),+ } = self;
                f( $( $arg ),+ )
            }
        }
    );

    ($vis:vis $name:ident {
        $(
            $arg:ident: $arg_type: ty = $arg_default: expr
        ),+ $(,)?
    }) => (
        #[allow(non_camel_case_types)]
        #[derive(Copy, Clone, Debug, PartialEq, Eq)]
        #[cfg_attr(feature = "serde", derive($crate::serde::Serialize, $crate::serde::Deserialize))]
        $vis struct $name {
            $(
                $arg: $arg_type
            ),+
        }

        #[allow(non_camel_case_types)]
        impl $name {
            #[allow(dead_code)]
            pub fn call<F, R>(self, f: F) -> R
                where F: FnOnce( $( $arg_type ),+ ) -> R
            {
                let Self { $( $arg ),+ } = self;
                f( $( $arg ),+ )
            }
        }

        impl Default for $name {
            fn default() -> Self {
                Self {
                    $(
                        $arg: $arg_default
                    ),+
                }
            }
        }
    );
}
