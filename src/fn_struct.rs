/// Generates a struct representing the arguments of a function in order, with a `call` function to
/// actually call the function. Only the name of the fields are listed and the generated struct is
/// generic, while the `call` function is generic on everything implementing [`FnOnce`] with proper
/// arguments. The return type of `call` is also generic, returning what the provided function
/// returns.
///
/// It is targeted to be used when writing tests in which a function with many parameters is called
/// often and for which repeated arguments can be applied using [`spread!`](crate::spread!).
///
/// ```rust
/// use nanotweaks::{anon, spread, fn_struct};
///
/// fn_struct!(VecValue(vec, value));
///
/// let mut v = vec!["foo"];
///
/// // Can store the arguments and use them afterward.
/// let push = VecValue {
///     vec: &mut v,
///     value: "bar",
/// };
/// push.call(Vec::push);
///
/// // Called function can return a value.
/// // Type of the fields is generic, however the name of the fields may not be appropriate.
/// let remove = VecValue {
///     vec: &mut v,
///     value: 0,
/// };
/// let item = remove.call(Vec::remove);
/// assert_eq!(item, "foo");
///
/// // You can even use it with lambdas.
/// let insert = VecValue {
///     vec: &mut v,
///     value: "baz",
/// };
/// insert.call(|vec, v| vec.insert(0, v));
///
/// // The goal is of course to use with `spread!` or struct update syntax.
/// // Here there is only one argument, but it may come handy when writing tests
/// // with functions having many arguments.
/// let anon = anon! { value: "hey", };
/// let push = spread!(VecValue {
///     vec: &mut v,
///     { value } in anon,
/// });
/// push.call(Vec::push);
///
/// assert_eq!(v, vec!["baz", "bar", "hey"]);
/// ```
#[macro_export]
macro_rules! fn_struct {
    ($vis:vis $name:ident (
        $(
            $arg:ident
        ),+ $(,)?
    )) => (
        #[allow(non_camel_case_types)]
        #[derive(Copy, Clone, Debug, PartialEq, Eq)]
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
    )
}
