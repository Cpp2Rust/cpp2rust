// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <cstddef>
#include <memory>

template <typename T1> using t1 = std::unique_ptr<T1>;
template <typename T1> using t2 = std::unique_ptr<T1[]>;

template <typename T2, typename T1> std::unique_ptr<T1[]> f1(std::size_t n) {
  return std::make_unique<T1[]>(n);
}

template <typename T1> T1 *f2(std::unique_ptr<T1> &o) { return o.get(); }

template <typename T1> std::unique_ptr<T1> f3(T1 *p) {
  return std::unique_ptr<T1>(p);
}

template <typename T1> std::unique_ptr<T1[]> f4(T1 *p) {
  return std::unique_ptr<T1[]>(p);
}

template <typename T1> void f5(std::unique_ptr<T1> &o, T1 *p) {
  return o.reset(p);
}

template <typename T1> void f6(std::unique_ptr<T1[]> &o, T1 *p) {
  return o.reset(p);
}

template <typename T1> T1 *f7(std::unique_ptr<T1[]> &o) { return o.get(); }

// template <typename T, typename... Args>
// std::unique_ptr<T> f8(Args&&... args) {
//     return std::make_unique<T>(std::forward<Args>(args)...);
// }

// Rust does not have variadic generics. We should consider writing specialized
// versions for make_unique with 1, 2, 3, etc arguments and translate the
// specialized versions.

template <typename T1, typename T2> std::unique_ptr<T1> f8(T2 &&a0) {
  return std::make_unique<T1>(std::move(a0));
}

template <typename T1> void f9(std::unique_ptr<T1[]> &o) {
  return o.reset(nullptr);
}

template <typename T1> std::unique_ptr<T1> f10() {
  return std::unique_ptr<T1>();
}

template <typename T1> std::unique_ptr<T1[]> f11() {
  return std::unique_ptr<T1[]>();
}
