# The Rules IR

Each rule module compiles to up to three JSON files in
`<build>/rules/<module>/`:

- `ir_src.json`: the C++ side, from
  [`cpp-rule-preprocessor`](./preprocessors.md#cpp-rule-preprocessor).
- `ir_unsafe.json`: the Rust side for the unsafe model, from
  [`rule-preprocessor`](./preprocessors.md#rule-preprocessor).
- `ir_refcount.json`: the Rust side for the refcount model, also from
  `rule-preprocessor` (only if the module has a `tgt_refcount.rs`).

All three are objects keyed by rule name (`f1`, `t1`, ...), and the loader joins
them by name.

## Source IR (`ir_src.json`)

A flat map from rule name to the canonical signature of the C++ construct the
rule matches. For `rules/vector`:

```json
{
  "t1": "std::vector<T1>",
  "f3": "_Bool std::vector<T1>::empty() const"
}
```

This signature string is the lookup key for the whole rule: the converter prints
C++ constructs from the input AST with the same printer and compares the
strings.

## Target IR (`ir_unsafe.json` / `ir_refcount.json`)

An expression rule serializes as an `ExprRule` object: the rule's signature plus
its body as a list of _fragments_. For
`unsafe fn f6<T1>(a0: &mut Vec<T1>) -> *mut T1 { a0.as_mut_ptr() }`:

```json
"f6": {
  "body": [
    { "method_call": {
        "receiver": [ { "placeholder": { "arg": 0, "access": "read" } } ],
        "body": [ { "text": ".as_mut_ptr()" } ] } }
  ],
  "generics": { "T1": [] },
  "params": { "a0": { "type": "&mut Vec<T1>" } },
  "return_type": { "type": "*mut T1", "is_unsafe_pointer": true }
}
```

The fragment kinds are:

- `text`: literal Rust source, emitted verbatim.
- `placeholder`: a use of one of the rule's `aN` parameters in the body (not an
  argument of whatever the body calls); the converter substitutes the translated
  call-site argument here. Its fields:
  - `arg`: the parameter index N.
  - `access`: how the body uses the argument: `read`, `write`, or `move`.
  - `is_index_base`: the placeholder is the base of an index expression.
- `generic`: a `TN` slot, replaced with the instantiated Rust type; serialized
  as the 1-based index N.
- `method_call`: a method call split into `receiver` and `body` fragment lists,
  so the code generator can rewrite the pair (see
  [Rule Rewriting](./rewriting.md)).
- `va_args`: the expansion point for a variadic tail.

Every type in the Rules IR (in `params`, `return_type`, and type rules) is a
`TypeInfo` object, the type text plus a set of flags:

- `is_refcount_pointer`: the type is a `Ptr<...>`.
- `is_unsafe_pointer`: the type is a raw `*mut`/`*const` pointer.
- `derives` (type rules only): the standard traits the mapped type implements
  (`Copy`, `Clone`, `Default`, ...).

The two pointer flags are mutually exclusive; the loader rejects a type with
both set.

An `ExprRule` carries two flags of its own:

- `multi_statement`: the body has more than one statement, or a statement
  followed by a tail expression, and must be wrapped in a block to stay a single
  expression.
- `is_extern`: the rule is an extern passthrough declaration and has no body.

Fields that are false, empty, or unset are omitted from the Rules IR. A `va`
parameter is never listed in `params`, and a `()` return type is omitted.

A type rule serializes as a `TypeRule` object: its `TypeInfo` plus the `init`
initializer expression, merged into one object:

```json
"t1": { "type": "Vec<T1>", "init": "Default::default()" }
```

There is no explicit tag distinguishing the two rule kinds: an entry with `body`
is an expression rule, one with `type` and `init` a type rule.

## In-memory form

`cpp2rust` mirrors the Rules IR in C++ structs of the same names, defined in
[`cpp2rust/converter/translation_rule.h`][translation-rule-h].
`TranslationRule::Load` reads one module directory (the `ir_*.json` files
described above) and produces two maps keyed by rule name, one holding
[`ExprRule`]s and one holding [`TypeRule`]s:

[translation-rule-h]:
  https://github.com/cpp2rust/cpp2rust/blob/master/cpp2rust/converter/translation_rule.h
[`ExprRule`]:
  https://github.com/cpp2rust/cpp2rust/blob/master/cpp2rust/converter/translation_rule.h#L71
[`TypeRule`]:
  https://github.com/cpp2rust/cpp2rust/blob/master/cpp2rust/converter/translation_rule.h#L84

- An `ExprRule` holds the body
  [fragments](#target-ir-ir_unsafejson--ir_refcountjson), the parameter and
  return `TypeInfo`s, and the two rule-level flags (`multi_statement` and
  `is_extern`). The name-keyed Rules IR maps become positional vectors:
  parameter `aN` is entry N of `params`, generic `TN` is entry N-1 of `generics`
  (each entry being the bound list). Rules support at most 9 generic parameters
  (`kMaxGenerics`).
- A `TypeRule` holds the mapped type's `TypeInfo` and the `initializer`
  expression. The same struct also represents the built-in type mappings
  (scalars, pointers, ...) that the loader registers directly in code, without
  any Rules IR behind them: for example, `int` maps to `i32`, and `int *` to
  `*mut i32` in the unsafe model or `Ptr<i32>` in the refcount model.

Both also carry `src`, the canonical C++ signature attached from
[`ir_src.json`](#source-ir-ir_srcjson); it is the key the rule is matched by.

How the loader finds the Rules IR directory, overlays the refcount model on the
unsafe one, and indexes the loaded rules for matching is covered in
[Loading and Matching](./loading.md).
