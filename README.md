## Introduction

The expression problem: [1](https://en.wikipedia.org/wiki/Expression_problem), [2](https://eli.thegreenplace.net/2016/the-expression-problem-and-its-solutions/) is the challenge of adding new data types that satisfy given operations and adding new operations on those types without modifying existing code, while maintaining type safety.

The problem is about programming language design and interaction with the outer world.

This memo is just a glimpse into a rich world of abstractions which can be rigorously grouped along the axes of [Barendregt's lambda cube](https://en.wikipedia.org/wiki/Lambda_cube). Help yourself.

## DeepSeek with Go, Rust, and Haskell

```console
go run add_type.go
rustc add_operation.rs -o output && ./output
runghc add_type_and_operation.hs
```

- add_type.go. Basic use of the Go `interface`. Easy to add a new type.

- add_operation.rs. Basic use of the Rust `enum`. Easy to add a new operation. Exhaustive checks.

- add_operation.go. A new operation can be added by **embedding** the original type first (wrapping or composition, "has a", not "is a"). Note that in Go, one cannot add methods to existing types from other packages. So this is verbose and loses type identity, but Go still allows to have those "open interfaces".

- add_type.rs. A new type can be added by using `<dyn Trait>` instead of `enum`, but this relies on dynamic dispatch, and we lose exhaustive checks. 

- add_type_static.rs. When asked to remove dynamic dispatch, DeepSeek hallucinates with the Rust `PhantomData` and weird type erasure codes which produce compiler errors. When hinted to use generics or macros, it does the job. This particular code manages to remove dynamic dispatch, but is limited to a few manual additions of types as one cannot have heterogeneous containers to iterate over.

- add_type_macro.rs. One of those rare motivations for a macro. It allows to add a type by generating the code which essentially results in `add_operation.rs`. This is still fairly limited as the macro must belong to the same crate with all the types (the orphan rule blocks implementing foreign traits for foreign types). This will typically demand forking the whole crate of the library.

- add_type_and_operation.hs. Considered as the ideal solution by DeepSeek. Do not ask me how it works, but it has everything. Open types, open operations (type classes), the ability to iterate over a heterogeneous list (existential types), type class resolution at compile time. 
 
