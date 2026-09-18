# Summary

# The Project

- [Introduction](./project/introduction.md)
- [Building](./project/building.md)
- [Usage](./project/usage.md)
- [Test Suite](./project/test-suite.md)

# Translation Rules

- [Overview](./rules/overview.md)
- [Rule Format](./rules/format.md)
- [Writing Rules](./rules/writing-rules.md)
- [Compat Shims](./rules/compat.md)
- [Conventions](./rules/conventions.md)
- [The Rule Preprocessors](./rules/preprocessors.md)
- [The Rules IR](./rules/ir.md)
- [Loading and Matching](./rules/loading.md)
- [The Matching Engine](./rules/matching.md)
- [Rule Rewriting](./rules/rewriting.md)

# The Runtime Library

- [Overview](./runtime/overview.md)
- [Reference Counting](./runtime/rc.md)
- [C Strings](./runtime/cstr.md)
- [void Pointers](./runtime/void.md)
- [Virtual Classes](./runtime/ptr-dyn.md)
- [Type Reinterpretation](./runtime/reinterpret.md)
- [Increment and Decrement](./runtime/inc-dec.md)
- [Iterators](./runtime/iterators.md)
- [Function Pointers](./runtime/fn-ptr.md)
- [Callable](./runtime/callable.md)
- [Variadic Functions](./runtime/va-args.md)
- [Control Flow Macros](./runtime/control-flow.md)
- [I/O and Formatting](./runtime/io.md)
- [libc Shims](./runtime/libc-shims.md)
- [Compat Helpers](./runtime/compat.md)

# Code Generation

- [Overview](./codegen/overview.md)
- [The Translation Pipeline](./codegen/pipeline.md)
- [Types](./codegen/types.md)
  - [Type Mappings](./codegen/types/mappings.md)
  - [Boxing](./codegen/types/boxing.md)
  - [Naming](./codegen/types/naming.md)
  - [Classes and Structs](./codegen/types/classes.md)
  - [Traits](./codegen/types/traits.md)
  - [Enums](./codegen/types/enums.md)
  - [Unions](./codegen/types/unions.md)
  - [Bit-fields](./codegen/types/bitfields.md)
  - [Pointers and References](./codegen/types/pointers.md)
  - [Casts](./codegen/types/casts.md)
  - [Function Pointers](./codegen/types/fn-pointers.md)
  - [Lambdas](./codegen/types/lambdas.md)
  - [Special-cased Library Types](./codegen/types/special-types.md)
