[![spread_macros
crate](https://img.shields.io/crates/v/spread_macros.svg)](https://crates.io/crates/spread_macros)
[![spread_macros documentation](https://docs.rs/spread_macros/badge.svg)](https://docs.rs/spread_macros)

Macros around an extended spread syntax.

## `spread!`

An extension of the spread/struct update syntax that allow taking fields from different type
structs, as long as the listed fields have the same type in both structs. It supports modifier
prefixes allowing to perform common transformations, such as cloning, converting, taking a
reference; or even custom transformation by providing a function path.

```rust
use spread_macros::spread;

struct Foo {
    one: String,
    two: u32,
    three: u32,
    four: &'static str,
}

struct Bar<'a> {
    one: String,
    two: u64,
    three: &'a u32,
    four: String,
}

let foo = Foo {
    one: "Hello".to_string(),
    two: 2,
    three: 3,
    four: "HELLO",
};

let two = 2u32;
let bar = spread!(Bar {
    >two, // calls .into()
    {
        +one, // calls .clone()
        &three, // takes a reference
        [str::to_lowercase] four,
    } in &foo,
});
```

## `anon!`

Generate a value of an anonymous struct with provided fields whose types are inferred. Can be used
to bundle many variables in a single struct to then be used in `spread!`. It supports the same
features as `spread!` (lists and modifiers) except for the final struct update syntax.

```rust
use spread_macros::anon;

struct Foo {
    one: String,
    two: u32,
    three: u32,
}

let foo = Foo {
    one: "Hello".to_string(),
    two: 2,
    three: 3,
};
let four = 4u32;

// Creates an anonymous struct with the given fields.
let exemple = anon! {
    { +one, >two, &three } in &foo,
    four,
};

// When using `>` (into) the field must be used for its type to be inferred.
let inferred: u64 = exemple.two;
println!("{exemple:?})");
```

## `slet!`

Avoids having to write a lot of transforations like `let variable_with_long_name =
variable_with_long_name.clone()` (which is common with closures and async blocks) by listing all the
identifiers and transformations with the same syntax as `anon!`. In additation, each field name can
be prefixed by `mut` (before a potential modifier) to make a `let mut` binding.

```rust
use spread_macros::slet;

let foo = "Hello".to_string();
let bar = 42u32;

slet! {mut +foo, >bar};
let inferred: u64 = bar;
```

## `fn_struct!`

Generates a struct representing the arguments of a given function or method, allowing to use Rust's
struct update syntax, `spread!` and `Default` with function arguments. The fields listed can use
modifiers from `spread!` like `&`, which allows for exemple to call functions with reference
arguments using a struct without references, which can thus implement `Default`. The struct can be
generic over the types of the function arguments, while the `call` function can also be generic over
types not appearing in the arguments.

It is targeted to be used when writing tests in which a function with many parameters is called
often and for which repeated arguments can be applied using `spread!`.

Asserts that some fields of the provided value match the expectation.

```rust
use spread_macros::fn_struct;

fn foo(foo: u32, bar: u32, baz: &u32) -> u32 {
    foo + bar + baz
}

fn_struct!(
    struct Foo
    for fn foo(
        one: u32 = 1,
        >two: u16 = 2, // converts from struct's u16 to functions u32
        &three: u32 = 3 // struct stores value, function takes reference
    ) -> u32
);

let res = Foo {
    three: 33,
    ..Default::default()
}
.call();

assert_eq!(res, 1 + 2 + 33);
```

## `assert_fields_eq!`

This expectation can be expressed in 2 ways:
- Another value can be provided, followed by a list of fields both values have in common and should
  be equal.
- An anonymous struct with the same syntax as `anon!`.

Afterward, the macro accepts a custom panic message with formating like `assert_eq!`.

It uses the in-scope `assert_eq!` macro, which allows to use alternative macros like
`similar_asserts::assert_eq!` if wanted.

```rust
use spread_macros::{anon, assert_fields_eq};

#[derive(Clone, Debug)]
struct Exemple {
    _foo: u32,
    bar: String,
    baz: bool,
}

let exemple = Exemple {
    _foo: 42,
    bar: String::from("exemple"),
    baz: true,
};

let expected = anon! {
    bar: String::from("exemple"),
    baz: true,
    other: "other",
};

assert_fields_eq!(exemple, {
    bar: String::from("exemple"),
    { +baz } in &expected,
});

assert_fields_eq!(
    exemple,
    expected,
    [bar, baz],
    "unexpected fields in {exemple:?}"
);
```