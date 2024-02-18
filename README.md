# nanotweaks

[![nanotweaks crate](https://img.shields.io/crates/v/nanotweaks.svg)](https://crates.io/crates/nanotweaks)
[![nanotweaks documentation](https://docs.rs/nanotweaks/badge.svg)](https://docs.rs/nanotweaks)

A collection of tools to write cleaner and shorter code.

## `spread!`

An extension of the spread/struct update syntax that allow taking fields from different type
structs, as long as the listed fields have the same type in both structs. It supports `#[clone]`
attributes that clone individual fields instead of the entire source struct.

## `anon!`

Generate a value of an anonymous struct with provided fields whose types are inferred. Can be used
to bundle many variables in a single struct to then be used in `spread!`. It supports the same
features as `spread!` (list and `#[clone]`) except for the final struct update syntax.

## `fn_struct!`

Generates a struct representing the arguments of a function in order, with a `call` function to
actually call the function. Only the name of the fields are listed and the generated struct is
generic, while the `call` function is generic on everything implementing `FnOnce` with proper
arguments. The return type of `call` is also generic, returning what the provided function returns.

It is targeted to be used when writing tests in which a function with many parameters is called
often and for which repeated arguments can be applied using `spread!`.

## `assert_fields_eq!`

Asserts that some fields of the provided value match the expectation.

This expectation can be expressed in 2 ways:
- Another value can be provided, followed by a list of fields both values have in common and should
  be equal.
- An anonymous struct with the same syntax as `anon!`.

Feature `similar-asserts` allows to internaly replace `core::assert_eq!` by
`similar_asserts::assert_eq!`, which provides a pretty diff output.

## `clone!`

Avoids having to write a lot of `let variable_with_long_name = variable_with_long_name.clone()`
(which is common in async code) by listing all the identifiers to clone. Each identifier can be
prefixed by `mut` if needed.

## `default`

Turbofish variant of `Default::default()`, mostly to use with struct update syntax or `spread!`.