# Loading and Matching

## Finding the rules directory

`cpp2rust` takes the Rules IR directory via `--rules <dir>`. If the flag is
omitted it tries `./rules` and then `<executable dir>/../rules`, accepting the
first candidate that contains (recursively) a subdirectory with `ir_src.json`
plus `ir_unsafe.json` or `ir_refcount.json`. Since the build writes the Rules IR
to `<build>/rules` and the binary lands in `<build>/bin`, the default resolution
picks up the generated Rules IR without any flags.

## Loading

Rules are loaded once per process by `Mapper::LoadTranslationRules`:

1. Built-in type mappings are registered first. Every scalar is mapped with its
   width taken from the host (`int` maps to `i32`, `unsigned long` to `u64`),
   together with its `const` form and its pointer forms: `*mut`/`*const` in the
   unsafe model, `Ptr<T>` in the refcount model, where constness is dropped.
   `char` maps to `libc::c_char` in the unsafe model and to `u8` in refcount;
   `size_t`/`ssize_t` map to `usize`/`isize`; `void *` maps to
   `*mut ::libc::c_void`, or in refcount to `AnyPtr`.
2. Every subdirectory of the rules directory is loaded with
   `TranslationRule::Load`, which reads `ir_unsafe.json`, overlays
   `ir_refcount.json` when translating with the refcount model, and then
   attaches the C++ signature from `ir_src.json` to each rule by name.

Loading is strict: an `ir_src.json` entry with no matching target rule is a
fatal error (this is what catches mismatched `#if`/`#[cfg]` gating), every
generic declared by a rule must appear in its C++ signature, and two type rules
for the same C++ type are rejected.

## Matching

Loaded rules are indexed in two multimaps, one for expression rules and one for
type rules. The multimap key is only a coarse bucket for collecting candidate
rules; whether a candidate actually matches is decided by
[the matching engine](./matching.md). The bucket key is derived from the C++
signature:

- For expressions, the qualified function name with the return type, the
  parameter list, and all template arguments stripped, so the rule for
  `_Bool std::vector<T1>::empty() const` lands in the `std::vector::empty`
  bucket.
- For types, the text up to the first `<`, so all `std::vector<...>` rules land
  in the `std::vector` bucket.

During translation, the converter prints the construct it encounters with the
same canonical printer used by `cpp-rule-preprocessor`, which is what makes the
two sides comparable:

- Functions and methods print as
  `<return type> <qualified::name>(<param types>[, ...])[ const][ volatile][ &|&&]`.
- Enum constants and global variables print as their qualified name.
- Integer literals expanded from a macro print as the macro name.

All rules in the matching bucket are then unified against this string by
[the matching engine](./matching.md), which binds `T1`...`T9` to the concrete
types at the use site and picks the most specific rule when several match. Type
lookups first try the sugar-preserving spelling (so a rule can match `size_t` as
written) and retry with the desugared type on failure.

Running `cpp2rust` with `--verbose` logs every lookup and the rule it matched,
which is the quickest way to see why a rule does or does not fire.

## Application

When a rule matches, the converter walks its body fragments and emits:

- `text` fragments verbatim,
- `placeholder` fragments as the translated call-site argument. How the argument
  is emitted depends on the placeholder's access and on whether the argument and
  the declared parameter type are pointers:
  - Read access emits the argument as a plain value, with an implicit numeric
    cast when the parameter type asks for one.
  - Write access emits the argument as an lvalue.
  - Move access wraps the argument in `std::mem::take(&mut ...)`; temporaries
    are moved as-is.
  - If the rule declares a pointer parameter but the argument is not a pointer,
    the converter takes a fresh pointer to it,
    [materializing a temporary](../codegen/temporaries.md) when the argument has
    no address of its own. For example, the refcount `std::max` rule declares
    `Ptr<T1>` parameters, since the C++ side takes `const T1 &` and the refcount
    model [translates references as `Ptr`](../codegen/pointers.md), so
    `std::max(x1, x2)` on plain locals substitutes `x1.as_pointer()` and
    `x2.as_pointer()` for the placeholders, while `std::max(30, 40)` first
    materializes `__tmp_0` and `__tmp_1` values for the literals and points into
    those.
  - If the receiver argument is a pointer but the rule expects a value, the
    converter dereferences it; this only happens for receivers, not for ordinary
    arguments.
- `generic` fragments as the Rust mapping of the bound C++ type,
- `va_args` fragments as the converted variadic tail,
- `method_call` fragments as receiver followed by body, possibly rewritten (see
  [Rule Rewriting](./rewriting.md)).

Multi-statement bodies are wrapped in `{ }` so they remain a single expression.
Rules for user-defined C++ types are injected through the same mechanism at
translation time.
