> "All of the things I once held, precious just don't  
> Mean anything anymore..."  
>
> <div align="right">— *Kim Wilde*</div>

## Introduction

The expression problem: [1](https://en.wikipedia.org/wiki/Expression_problem), [2](https://eli.thegreenplace.net/2016/the-expression-problem-and-its-solutions/) is the challenge of adding new data types that satisfy given operations and adding new operations on those types without modifying existing code, while maintaining type safety.

It is a solid way to assess programming languages.

## Go, Rust, and Haskell

```console
go run add_type.go
rustc add_operation.rs -o output && ./output
runghc add_type_and_operation.hs
```

- add_type.go: `interface`. Easy to add a new type.

- add_operation.rs: `enum`. Easy to add a new operation. Exhaustive checks.

- add_operation.go: **type embedding** (wrapping, composition, "has a", not "is a"). Note that in Go, one cannot simply add methods to existing types from other packages.

- add_type.rs: `<dyn Trait>` instead of `enum`, but this relies on dynamic dispatch, and we lose exhaustive checks. 

- add_type_static.rs. When asked to remove dynamic dispatch, DeepSeek hallucinates with `PhantomData` and weird "type erasure" codes which produce compiler errors. When hinted to use generics or macros, it does the job. This particular code manages to remove dynamic dispatch, but is limited as it does not lead to heterogeneous containers.

- add_type_macro.rs. One of those rare motivations for a macro. This is still fairly limited as the macro must belong to the same crate with all the types (the orphan rule blocks implementing foreign traits for foreign types). This will typically demand forking the whole crate.

- add_type_and_operation.hs. Haskell comes and wipes the floor with 'em all. Open types, open operations (type classes), the ability to iterate over a heterogeneous list (existential types), type class resolution at compile time. 
 
