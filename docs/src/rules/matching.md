# The Matching Engine

[Loading and Matching](./loading.md) collects the candidate rules for a
construct from a bucket; each candidate's source signature is then _unified_
against the printed construct. The signature is treated as a template whose
`T1`...`T9` slots capture concrete types: `std::vector<T1>::vector()` unifies
with `std::vector<int>::vector()` by binding `T1 = int`.

Unification works on the two strings:

- Whitespace differences are ignored.
- A `TN` slot captures up to the next literal text of the pattern, found at the
  same `<>`/`()`/`[]` nesting depth. This is how `T1` captures all of
  `std::map<int, int>` in `std::vector<std::map<int, int>>` without stopping at
  the inner comma.
- A `TN` that appears again must match its first capture exactly.
- The whole printed string must be consumed; trailing text fails the match.
- Slots may stay unbound (a pattern can use `T2` without `T1`).

A rule matches if unification succeeds. When several rules in the bucket match,
the one with the longest source signature wins, so more specific rules take
precedence; between equally long signatures the choice is unspecified.

## Bucket keys

The bucket keys described in [Loading and Matching](./loading.md#matching) have
two special cases: array types bucket by the text after the first `[` rather
than the text before a `<`, and `operator()` rules are cut at the operator's own
parentheses, so their key ends at `...::operator`.

## Instantiating the target

Captures are C++ spellings. Before being substituted into the rule's Rust
fragments, each capture is itself mapped through the type rules, recursively, so
`T1 = std::vector<int>` substitutes as `Vec<i32>`. A captured type with no type
rule of its own is an error.
