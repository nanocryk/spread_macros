#![doc = include_str!("../README.md")]

/// Standalone [`Default::default()`] function.
pub fn default<T: Default>() -> T {
    T::default()
}

/// Create a value of an anonymous struct with provided fields whose types are inferred.
/// The syntax is the same as [`spread!`](crate::spread!) without the struct name, and without
/// the ability to use the `..remaining` syntax.
/// ```rust
/// use nanotweaks::anon;
///
/// #[derive(Clone, Debug, Default)]
/// struct Bar {
///     spread: u32,
///     spread_ref: u32,
///     spread_ref_mut: u32,
///     spread_into: u32,
///     spread_clone: u32,
///     spread_clone_into: u32,
/// }
///
/// let mut bar = Bar::default();
/// let name = 42u32;
/// let name_ref = 42u32;
/// let name_into = 42u32;
/// let name_clone = 42u32;
/// let name_clone_into = 42u32;
/// let mut name_ref_mut = 42u32;
///
/// let anon = anon!{
///     name,
///     &name_ref,
///     &mut name_ref_mut,
///     >name_into,
///     +name_clone,
///     +>name_clone_into,
///     value: 42,
///     {
///         spread,
///         &spread_ref,
///         &mut spread_ref_mut,
///         >spread_into,
///         +spread_clone,
///         +>spread_clone_into,
///     } in &mut bar,
/// };
///
/// // Fields with `>` (Into) needs to be used for their type to be inferred.
/// let infered: u64 = anon.name_into;
/// let infered: u64 = anon.name_clone_into;
/// let infered: u64 = anon.spread_into;
/// let infered: u64 = anon.spread_clone_into;
/// ```
pub use nanotweaks_proc::anon;

/// Allows to perform multiple `let` bindings with the same syntax as [`anon!`](crate::anon!),
/// modifiers included. It is expected to be used in places where a lot of transformations are
/// performed, such as lots of clones before moving values in a closure or async block.
///
/// Each field name can be prefixed (before a potential modifier) but `mut` to perform a `let mut`
/// binding.
///
/// ```rust
/// use nanotweaks::slet;
///
/// #[derive(Clone, Debug, Default)]
/// struct Bar {
///     spread: u32,
///     spread_ref: u32,
///     spread_ref_mut: u32,
///     spread_into: u32,
///     spread_clone: u32,
///     spread_clone_into: u32,
/// }
///
/// let mut bar = Bar::default();
/// let name = 42u32;
/// let name_ref = 42u32;
/// let name_into = 42u32;
/// let name_clone = 42u32;
/// let name_clone_into = 42u32;
/// let mut name_ref_mut = 42u32;
///
/// {
///     slet! {
///         name,
///         mut &name_ref,
///         &mut name_ref_mut,
///         mut >name_into,
///         +name_clone,
///         mut +>name_clone_into,
///         value: 42,
///         {
///             mut spread,
///             &spread_ref,
///             mut &mut spread_ref_mut,
///             >spread_into,
///             mut +spread_clone,
///             +>spread_clone_into,
///         } in &mut bar,
///     };
///
///     // Fields with `>` (Into) needs to be used for their type to be inferred.
///     let infered: u64 = name_into;
///     let infered: u64 = name_clone_into;
///     let infered: u64 = spread_into;
///     let infered: u64 = spread_clone_into;
/// }
/// ```
pub use nanotweaks_proc::slet;

/// Extension of the spread/[struct update syntax] that allow taking fields from different type
/// structs, as long as the listed fields have the same type in both structs.
///
/// It can be used with structs that don't have sensible defaults for each fields by using another
/// struct that only have the fields with sensible defaults.
///
/// [struct update syntax]:
///     https://doc.rust-lang.org/book/ch05-01-defining-structs.html#creating-instances-from-other-instances-with-struct-update-syntax
///
/// Fields can be listed as follows:
/// - `field,`: field which captures a variable of the same name
/// - `field: value,`: field with provided value
/// - `{ field1, field2 } in source,`: fields extracted from another struct
/// - `..remaining`: same as in [struct update syntax], can only appear last without a trailing
///   comma
///
/// Each field name can be prefixed by a modifier allowing to perform common transformations that
/// usually requires repeating the field name. They are placed before the field and mean the
/// following:
/// - `&field`: take the reference, convert a `T` field to `&T`
/// - `&mut field`: take the mutable reference, convert a `T` field to `&mut T`
/// - `+field`: clones the value, can be used with `&source` to not consume the source
/// - `>field`: converts the value with `Into`
/// - `+>field`: clones then converts the value with `Into`, can be used with `&source` to not
/// consume the source
///
/// Here is an exemple showing all the modifers:
///
/// ```rust
/// use nanotweaks::spread;
///
/// #[derive(Debug)]
/// struct Foo<'a> {
///     name: u32,
///     name_ref: &'a u32,
///     name_ref_mut: &'a mut u32,
///     name_into: u64,
///     name_clone: u32,
///     name_clone_into: u64,
///
///     value: u32,
///
///     spread: u32,
///     spread_ref: &'a u32,
///     spread_ref_mut: &'a mut u32,
///     spread_into: u64,
///     spread_clone: u32,
///     spread_clone_into: u64,
///
///     other: u32,
/// }
///
/// #[derive(Clone, Debug, Default)]
/// struct Bar {
///     spread: u32,
///     spread_ref: u32,
///     spread_ref_mut: u32,
///     spread_into: u32,
///     spread_clone: u32,
///     spread_clone_into: u32,
///
///     other: u32,
/// }
///
/// let mut bar = Bar::default();
/// let name = 42u32;
/// let name_ref = 42u32;
/// let name_into = 42u32;
/// let name_clone = 42u32;
/// let name_clone_into = 42u32;
/// let mut name_ref_mut = 42u32;
///
/// let first = spread!(Foo {
///     name,
///     &name_ref,
///     &mut name_ref_mut,
///     >name_into,
///     +name_clone,
///     +>name_clone_into,
///     value: 42,
///     {
///         spread,
///         &spread_ref,
///         &mut spread_ref_mut,
///         >spread_into,
///         +spread_clone,
///         +>spread_clone_into,
///     } in &mut bar,
///     >other: 42u16,
/// });
///
/// let second = spread!(Foo {
///     name,
///     >name_into,
///     +name_clone,
///     +>name_clone_into,
///     value: 42,
///     ..first
/// });
pub use nanotweaks_proc::spread;

/// Generates a struct representing the arguments of a given function or method, allowing to use
/// Rust's struct update syntax, [`spread!`](crate::spread!) and `Default` with function arguments.
/// The fields listed can use modifiers from [`spread!`] like `&`, which allows for exemple to call
/// functions with reference arguments using a struct without references, which can thus implement
/// `Default`.
/// ```rust
/// use nanotweaks::{fn_struct, default};
///
/// fn foo(foo: u32, bar: u32, baz: &u32) -> u32 {
///     foo + bar + baz
/// }
///
/// fn_struct!(
///     Foo
///     for foo(
///         one: u32 = 1,
///         >two: u16 = 2, // converst's from struct's u16 to functions u32
///         &three: u32 = 3 // struct stores value, function takes reference
///     ) -> u32
/// );
///
/// let res = Foo {
///     three: 33,
///     ..default()
/// }
/// .call();
///
/// assert_eq!(res, 1 + 2 + 33);
/// ```
///
/// Note here that `res` is consumed when calling `call`. It can by reused if the name of the struct
/// is prefixed with `&`, but it requires all the fields to either be `Copy`, passed by reference or
/// cloned using `+` modifier (or `+>` to clone then convert it).
/// ```rust
/// # use nanotweaks::{fn_struct, default};
/// #
/// # fn foo(foo: u32, bar: u32, baz: &u32) -> u32 {
/// #     foo + bar + baz
/// # }
/// fn_struct!(
///     &Foo
///     for foo(
///         one: u32 = 1,
///         >two: u16 = 2,
///         &three: u32 = 3
///     ) -> u32
/// );
///
/// let args = Foo {
///     three: 33,
///     ..default()
/// };
/// args.call();
/// args.call();
/// ```
///
/// The struct can be generic over the types of the function arguments, while the `call`
/// function can also be generic over types not appearing in the arguments.
/// ```rust
/// # use nanotweaks::fn_struct;
/// fn_struct!(
///     // `T` must be listed here as one of the arguments use it.
///     &VecPush<T: Clone>
///     for Vec::<T>::push(
///         &mut self,
///         +value: T
///     )
/// );
///
/// fn_struct!(
///     &VecPop
///     // `T` must be listed here as no arguments use it but it is still used by the function.
///     for<T> Vec::<T>::pop(
///         &mut self,
///     ) -> Option<T>
/// );
///
/// let mut list = vec![1, 2, 3, 4];
///
/// let pop = VecPop { };
/// pop.call(&mut list);
/// pop.call(&mut list);
/// assert_eq!(&list, &[1, 2]);
///
/// let push = VecPush { value: 10 };
/// push.call(&mut list);
/// push.call(&mut list);
/// assert_eq!(&list, &[1, 2, 10, 10]);
/// ```
///
/// Struct can be annotated with usual derives and attributes by writing them at the start.
/// ```rust
/// # use nanotweaks::fn_struct;
/// fn_struct!(
///     #[derive(Debug, Clone, Copy, PartialEq, Eq)]
///     pub VecPush<T: Clone>
///     for Vec::<T>::push(
///         &mut self,
///         value: T
///     )
/// );
///
pub use nanotweaks_proc::fn_struct;

// // public to re-export `assert_eq!` from either `core` or `similar_asserts` based on the
// // `similar-asserts` feature.
// #[doc(hidden)]
// pub mod assert_fields_eq;
