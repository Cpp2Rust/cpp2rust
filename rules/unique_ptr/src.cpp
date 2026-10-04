// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#ifdef CPP2RUST_USE_INCLUDES
#include <cstddef>
#include <memory>
#endif

template <typename T, typename A> using Init = A;

template <typename T1> using CPP2RUST_TYPE_RULE(1) = std::unique_ptr<T1>;
template <typename T1> using CPP2RUST_TYPE_RULE(2) = std::unique_ptr<T1[]>;

template <typename T2, typename T1>
std::unique_ptr<T1[]> CPP2RUST_EXPR_RULE(1)(std::size_t n) {
  return std::make_unique<T1[]>(n);
}

template <typename T1> T1 *CPP2RUST_EXPR_RULE(2)(std::unique_ptr<T1> &o) {
  return o.get();
}

template <typename T1> std::unique_ptr<T1> CPP2RUST_EXPR_RULE(3)(T1 *p) {
  return std::unique_ptr<T1>(p);
}

template <typename T1> std::unique_ptr<T1[]> CPP2RUST_EXPR_RULE(4)(T1 *p) {
  return std::unique_ptr<T1[]>(p);
}

template <typename T1>
void CPP2RUST_EXPR_RULE(5)(std::unique_ptr<T1> &o, T1 *p) {
  return o.reset(p);
}

template <typename T1>
void CPP2RUST_EXPR_RULE(6)(std::unique_ptr<T1[]> &o, T1 *p) {
  return o.reset(p);
}

template <typename T1> T1 *CPP2RUST_EXPR_RULE(7)(std::unique_ptr<T1[]> &o) {
  return o.get();
}

template <typename T1, typename... Args>
std::unique_ptr<T1> CPP2RUST_EXPR_RULE(8)(Init<T1, Args> &&...args) {
  return std::make_unique<T1>(std::forward<Args>(args)...);
}

template <typename T1> void CPP2RUST_EXPR_RULE(9)(std::unique_ptr<T1[]> &o) {
  return o.reset(nullptr);
}

template <typename T1> std::unique_ptr<T1> CPP2RUST_EXPR_RULE(10)() {
  return std::unique_ptr<T1>();
}

template <typename T1> std::unique_ptr<T1[]> CPP2RUST_EXPR_RULE(11)() {
  return std::unique_ptr<T1[]>();
}

template <typename T1>
std::unique_ptr<T1> CPP2RUST_EXPR_RULE(12)(std::unique_ptr<T1> &&o) {
  return std::unique_ptr<T1>(std::move(o));
}

template <typename T1>
std::unique_ptr<T1[]> CPP2RUST_EXPR_RULE(13)(std::unique_ptr<T1[]> &&o) {
  return std::unique_ptr<T1[]>(std::move(o));
}

template <typename T1>
std::unique_ptr<T1> &CPP2RUST_EXPR_RULE(14)(std::unique_ptr<T1> &dst,
                                            std::unique_ptr<T1> &&src) {
  return dst.operator=(std::move(src));
}

template <typename T1>
std::unique_ptr<T1[]> &CPP2RUST_EXPR_RULE(15)(std::unique_ptr<T1[]> &dst,
                                              std::unique_ptr<T1[]> &&src) {
  return dst.operator=(std::move(src));
}
