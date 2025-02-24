## Introduction

The expression problem: [1](https://en.wikipedia.org/wiki/Expression_problem), [2](https://eli.thegreenplace.net/2016/the-expression-problem-and-its-solutions/) is the goal to extend programs in two dimensions: adding new data types and adding new operations on those types, without having to modify existing code. The challenge is to do both without modifying existing modules, and keeping type safety.

This is an interesting question to ask as it reveals a lot about a language design, interaction with 3rd party libs, and what is going 
on with one of the axes of [Barendregt's lambda cube](https://en.wikipedia.org/wiki/Lambda_cube).

I am no expert, this is just a glimpse at a fairly rich world of abstractions. Help yourself.

## DeepSeek with Go, Rust, and Haskell

```console
go run add_type.go
rustc add_operation.rs -o output && ./output
runghc add_type_and_operation.hs
```

- add_type.go. Basic use of the Go `interface`. Easy to add a new type.

- add_operation.rs. Basic use of the Rust `enum`. Easy to add a new operation. Exhaustive checks.

- add_operation.go. A new operation can be added by **embedding** the original type first (wrapping or composition, not inheritance). Note that in Go, one cannot add methods to existing types from other packages. So this is verbose and loses type identity, but it is interesting that Go still allows to have those "open interfaces". A lot of thought went into the language design, despite its minimalism prior to v1.18.

- add_type.rs. A new type can be added by using `<dyn Trait>` instead of `enum`, but this relies on dynamic dispatch, and we lose exhaustive checks.

- add_type_static.rs. When asked to remove dynamic dispatch, DeepSeek hallucinates with PhantomData and all sorts of weird type erasure codes which produce compiler errors. When hinted about generics or macros, it does the job. This particular code manages to remove dynamic dispatch, but is limited to a few manual additions of types as there are no heterogeneous containers to iterate over.

- add_type_macro.rs. This can serve as one of those rare motivations for the use of a macro. It allows to add a type by generating the code which essentially follows `add_operation.rs`. This is still fairly limited as the macro must belong to the same crate with all the types (the orphan rule blocks implementing foreign traits for foreign types). This will typically demand forking the whole crate of the library.

- add_type_and_operation.hs. Considered as the ideal solution by DeepSeek. Do not ask me how it works, but it has everything. Open types, open operations (type classes), the ability to iterate over a heterogeneous list (existential types), the type class resolution at compile time. 

They say Ocaml's Modules/Functors are similarly powerful/expressive, but F# deliberately does not go there. Personally, I would take F# or even Go v1.17 and [the rule of least power](https://en.wikipedia.org/wiki/Rule_of_least_power). It is alright not to have a language with a concept/abstraction for every possible problem.
 
