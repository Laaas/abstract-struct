# abstract_struct

A Rust macro for automatically generating a corresponding trait.

## Why?

The purpose of creating a corresponding trait is to hide
away the generic parameters of a struct.

A lot of the time, you don't really care about what types
a generic struct was instantiated with; you just want to use it.

This way you can also add new generic parameters without breaking
existing code.

## Requirements

You must be using a nightly compiler, since attribute-like procedural macros
haven't been stabilized yet.

## Example

There are thorough examples in the examples directory in the repository.

```rust
extern crate abstract_struct;

use abstract_struct::{abstract_struct, abstract_struct_debug};

trait A {fn a(&self) {}}
trait B {fn b(&self) {}}
trait C {fn c(&self) {}}
trait D {fn d(&self) {}}
trait E {fn e(&self) {}}
trait F {fn f(&self) {}}

// use abstract_struct_debug if you want to inspect the generated code.
#[abstract_struct]
pub struct MyAwesomeStruct<T: A + B, U: C + D, V: E + F>
{
	t: T,
	u: U,
	v: V,
}

fn use_awesome_struct<U: C + D + std::fmt::Debug>(s: impl MyAwesomeStructAbstract<U = U>) {
	s.t.a();
	s.t.b();
	println!("{:?}", s.u);
}
```

This creates a public struct called MyAwesomeStruct and a corresponding trait
called MyAwesomeStructAbstract, which is also public.
The publicity of the trait matches the publicity of the struct.
