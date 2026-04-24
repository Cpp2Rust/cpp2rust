// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <algorithm>
#include <fstream>
#include <iterator>
#include <string>
#include <vector>

struct T2 {
  friend bool operator<(T2 a, T2 b) { return false; }
};

struct T1 {
  using value_type = T2;
  using difference_type = std::ptrdiff_t;
  using reference = T2 &;
  using pointer = T2 *;
  using iterator_category = std::random_access_iterator_tag;

  pointer p = nullptr;

  T1() = default;

  operator T2() const { return {}; }

  reference operator*() const { return *p; }
  pointer operator->() const { return p; }
  reference operator[](difference_type n) const { return p[n]; }

  T1 &operator++() {
    ++p;
    return *this;
  }
  T1 operator++(int) {
    T1 tmp = *this;
    ++*this;
    return tmp;
  }
  T1 &operator--() {
    --p;
    return *this;
  }
  T1 operator--(int) {
    T1 tmp = *this;
    --*this;
    return tmp;
  }

  T1 &operator+=(difference_type n) {
    p += n;
    return *this;
  }
  T1 &operator-=(difference_type n) {
    p -= n;
    return *this;
  }
  friend T1 operator+(T1 it, difference_type n) {
    it += n;
    return it;
  }
  friend T1 operator+(difference_type n, T1 it) {
    it += n;
    return it;
  }
  friend T1 operator-(T1 it, difference_type n) {
    it -= n;
    return it;
  }
  friend difference_type operator-(T1 a, T1 b) { return a.p - b.p; }

  friend bool operator==(T1 a, T1 b) { return a.p == b.p; }
  friend bool operator!=(T1 a, T1 b) { return a.p != b.p; }
  friend bool operator<(T1 a, T1 b) { return a.p < b.p; }
  friend bool operator>(T1 a, T1 b) { return a.p > b.p; }
  friend bool operator<=(T1 a, T1 b) { return a.p <= b.p; }
  friend bool operator>=(T1 a, T1 b) { return a.p >= b.p; }

  T1 &operator=(const T2 &rhs) { return *this; }
};

template <typename T1> void f1(T1 first, T1 last) {
  return std::sort(first, last);
}

template <typename T1, typename T2> T2 f2(T1 first, T1 last, T2 d_first) {
  return std::copy(first, last, d_first);
}

template <class T1, class T2> T1 f3(T1 first, T1 last, const T2 &value) {
  return std::find(first, last, value);
}

// TODO
auto lambda = [](const T2 &a, const T2 &b) { return false; };
void f6(T1 first, T1 last, decltype(lambda) comp) {
  return std::stable_sort(first, last, comp);
}

template <typename T1, typename T2>
void f7(T1 first, T1 last, bool (*comp)(const T2 &, const T2 &)) {
  return std::stable_sort(first, last, comp);
}

template <typename T1> T1 *f8(T1 *first, T1 *last) {
  return std::max_element(first, last);
}

template <typename T1> void f9(T1 &a0, T1 &a1) { return std::swap(a0, a1); }

template <typename T1>
typename std::vector<T1>::iterator f10(typename std::vector<T1>::iterator a0,
                                       typename std::vector<T1>::iterator a1) {
  return std::unique(a0, a1);
}

template <typename T1, typename T2>
void f12(typename std::vector<T1>::iterator a0,
         typename std::vector<T1>::iterator a1, const T2 &a2) {
  return std::fill(a0, a1, a2);
}

std::ostream_iterator<char> f13(std::string::iterator a0,
                                std::string::iterator a1,
                                std::ostream_iterator<char> a2) {
  return std::copy(a0, a1, a2);
}

// TODO
auto lambda_nref = [](T2 a, T2 b) { return false; };
void f14(T1 *first, T1 *last, decltype(lambda_nref) comp) {
  return std::stable_sort(first, last, comp);
}

template <typename T1, typename T2>
void f15(T1 *first, T1 *last, bool (*comp)(T2, T2)) {
  return std::stable_sort(first, last, comp);
}

template <typename T1> const T1 &f16(const T1 &a, const T1 &b) {
  return std::min(a, b);
}

template <typename T1> const T1 &f17(const T1 &a, const T1 &b) {
  return std::max(a, b);
}
