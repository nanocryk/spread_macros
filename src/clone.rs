/// Clone one or many variables and bind them to the same name. When dealing
/// with `move` blocks/closures it is common to clone variables before moving
/// them. This macro reduces the boilerplate.
///
/// ## Exemple
/// ```rust
/// # use nanotweaks::clone;
/// # fn consume<T>(_: T) {
/// #     // ...
/// # }
///
///
/// let s1 = String::from("foo");
/// let s2 = String::from("bar");
///
/// {
///     clone!(s1, mut s2);
///     s2.push('t');
///     consume((s1, s2));
/// }
///
/// // Without the macro.
/// {
///     let s1 = s1.clone();
///     let mut s2 = s2.clone();
///     s2.push('t');
///     consume((s1, s2));
/// }
///
/// println!("{s1}{s2}");
/// ```
#[macro_export]
macro_rules! clone {
    () => {};
    (mut $name:ident $(, $($tail:tt)+)?) => {
        let mut $name = $name.clone();
        $(clone!($($tail)+);)?
    };
    ($name:ident $(, $($tail:tt)+)?) => {
        let $name = $name.clone();
        $(clone!($($tail)+);)?
    };
}
