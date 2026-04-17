// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

// https://doc.rust-lang.org/reference/keywords.html
#define KW(X)                                                                  \
  X(as)                                                                        \
  X(async)                                                                     \
  X(await)                                                                     \
  X(crate)                                                                     \
  X(dyn)                                                                       \
  X(fn)                                                                        \
  X(impl)                                                                      \
  X(in)                                                                        \
  X(let)                                                                       \
  X(loop)                                                                      \
  X(match)                                                                     \
  X(mod)                                                                       \
  X(move)                                                                      \
  X(mut)                                                                       \
  X(pub)                                                                       \
  X(ref)                                                                       \
  X(self)                                                                      \
  X(Self)                                                                      \
  X(super)                                                                     \
  X(trait)                                                                     \
  X(type)                                                                      \
  X(unsafe)                                                                    \
  X(use)                                                                       \
  X(where)                                                                     \
  X(abstract)                                                                  \
  X(become)                                                                    \
  X(box)                                                                       \
  X(final)                                                                     \
  X(gen)                                                                       \
  X(macro) X(override) X(priv) X(unsized) X(yield) X(macro_rules) X(raw) X(safe)

// Field declaration
#define F(n) int n;
struct S {
  KW(F)
};
#undef F

// Function arguments
#define F(n) int n,
int foo(KW(F) int dummy) { return 0; }
#undef F

int main() {
// Init list
#define F(n) .n = 0,
  S s = {KW(F)};
#undef F

// Local variable
#define F(n) int n = 0;
  KW(F)
#undef F

// Function call
#define F(n) 0,
  return foo(KW(F) 0);
#undef F
}
