// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <iterator>
#include <type_traits>
#include <utility>

#include "../common/init.h"

template <
    typename It, typename T1 = typename std::iterator_traits<It>::value_type,
    typename = typename std::enable_if<
        std::is_base_of<
            std::input_iterator_tag,
            typename std::iterator_traits<It>::iterator_category>::value &&
        !std::is_base_of<
            std::forward_iterator_tag,
            typename std::iterator_traits<It>::iterator_category>::value &&
        !std::is_pointer<It>::value>::type>
using t1 = It;

template <
    typename It, typename T1 = typename std::iterator_traits<It>::value_type,
    typename = typename std::enable_if<
        std::is_base_of<
            std::output_iterator_tag,
            typename std::iterator_traits<It>::iterator_category>::value &&
        !std::is_pointer<It>::value>::type>
using t2 = It;

template <
    typename It, typename T1 = typename std::iterator_traits<It>::value_type,
    typename = typename std::enable_if<
        std::is_base_of<
            std::forward_iterator_tag,
            typename std::iterator_traits<It>::iterator_category>::value &&
        !std::is_base_of<
            std::bidirectional_iterator_tag,
            typename std::iterator_traits<It>::iterator_category>::value &&
        !std::is_pointer<It>::value>::type>
using t3 = It;

template <
    typename It, typename T1 = typename std::iterator_traits<It>::value_type,
    typename = typename std::enable_if<
        std::is_base_of<
            std::bidirectional_iterator_tag,
            typename std::iterator_traits<It>::iterator_category>::value &&
        !std::is_base_of<
            std::random_access_iterator_tag,
            typename std::iterator_traits<It>::iterator_category>::value &&
        !std::is_pointer<It>::value>::type>
using t4 = It;

template <
    typename It, typename T1 = typename std::iterator_traits<It>::value_type,
    typename = typename std::enable_if<
        std::is_base_of<
            std::random_access_iterator_tag,
            typename std::iterator_traits<It>::iterator_category>::value &&
        !std::is_pointer<It>::value>::type>
using t5 = It;

template <
    typename T1, typename T2 = typename std::iterator_traits<T1>::value_type,
    typename = typename std::enable_if<
        (std::is_base_of<
             std::input_iterator_tag,
             typename std::iterator_traits<T1>::iterator_category>::value ||
         std::is_base_of<
             std::output_iterator_tag,
             typename std::iterator_traits<T1>::iterator_category>::value) &&
        !std::is_pointer<T1>::value>::type>
T1 f1(const T1 &a0) {
  return T1(a0);
}

template <
    typename T1, typename T2 = typename std::iterator_traits<T1>::value_type,
    typename = typename std::enable_if<
        (std::is_base_of<
             std::input_iterator_tag,
             typename std::iterator_traits<T1>::iterator_category>::value ||
         std::is_base_of<
             std::output_iterator_tag,
             typename std::iterator_traits<T1>::iterator_category>::value) &&
        !std::is_pointer<T1>::value && !std::is_reference<T1>::value>::type>
typename std::remove_reference<T1>::type f2(T1 &&a0) {
  return T1(std::move(a0));
}

template <
    typename T1, typename T2 = typename std::iterator_traits<T1>::value_type,
    typename = typename std::enable_if<
        (std::is_base_of<
             std::input_iterator_tag,
             typename std::iterator_traits<T1>::iterator_category>::value ||
         std::is_base_of<
             std::output_iterator_tag,
             typename std::iterator_traits<T1>::iterator_category>::value) &&
        !std::is_pointer<T1>::value>::type>
T1 &f3(T1 &a0, const T1 &a1) {
  return a0 = a1;
}

template <
    typename T1, typename T2 = typename std::iterator_traits<T1>::value_type,
    typename = typename std::enable_if<
        (std::is_base_of<
             std::input_iterator_tag,
             typename std::iterator_traits<T1>::iterator_category>::value ||
         std::is_base_of<
             std::output_iterator_tag,
             typename std::iterator_traits<T1>::iterator_category>::value) &&
        !std::is_pointer<T1>::value>::type>
T1 &f4(T1 &a0, T1 &&a1) {
  return a0 = std::move(a1);
}

template <
    typename T1, typename T2 = typename std::iterator_traits<T1>::value_type,
    typename = typename std::enable_if<
        (std::is_base_of<
             std::input_iterator_tag,
             typename std::iterator_traits<T1>::iterator_category>::value ||
         std::is_base_of<
             std::output_iterator_tag,
             typename std::iterator_traits<T1>::iterator_category>::value) &&
        !std::is_pointer<T1>::value>::type>
decltype(*std::declval<const T1 &>()) f5(const T1 &a0) {
  return *a0;
}

template <
    typename T3, typename T1 = typename std::remove_reference<T3>::type,
    typename T2 = typename std::iterator_traits<T1>::value_type,
    typename = typename std::enable_if<
        (std::is_base_of<
             std::input_iterator_tag,
             typename std::iterator_traits<T1>::iterator_category>::value ||
         std::is_base_of<
             std::output_iterator_tag,
             typename std::iterator_traits<T1>::iterator_category>::value) &&
        !std::is_pointer<T1>::value &&
        std::is_lvalue_reference<T3>::value>::type>
T1 &f6(T3 &&a0) {
  return ++a0;
}

template <
    typename T1, typename T2 = typename std::iterator_traits<T1>::value_type,
    typename = typename std::enable_if<
        (std::is_base_of<
             std::input_iterator_tag,
             typename std::iterator_traits<T1>::iterator_category>::value ||
         std::is_base_of<
             std::output_iterator_tag,
             typename std::iterator_traits<T1>::iterator_category>::value) &&
        !std::is_pointer<T1>::value>::type>
decltype(std::declval<T1 &>()++) f7(T1 &a0, int) {
  return a0++;
}

template <
    typename T1, typename T2 = typename std::iterator_traits<T1>::value_type,
    typename = typename std::enable_if<
        std::is_base_of<
            std::input_iterator_tag,
            typename std::iterator_traits<T1>::iterator_category>::value &&
        !std::is_pointer<T1>::value>::type>
typename std::iterator_traits<T1>::pointer f8(const T1 &a0) {
  return a0.operator->();
}

template <
    typename T1, typename T2 = typename std::iterator_traits<T1>::value_type,
    typename = typename std::enable_if<
        std::is_base_of<
            std::input_iterator_tag,
            typename std::iterator_traits<T1>::iterator_category>::value &&
        !std::is_pointer<T1>::value>::type>
bool f9(const T1 &a0, const T1 &a1) {
  return a0 == a1;
}

template <
    typename T1, typename T2 = typename std::iterator_traits<T1>::value_type,
    typename = typename std::enable_if<
        std::is_base_of<
            std::input_iterator_tag,
            typename std::iterator_traits<T1>::iterator_category>::value &&
        !std::is_pointer<T1>::value>::type>
bool f10(const T1 &a0, const T1 &a1) {
  return a0 != a1;
}

template <typename T3, typename T1 = typename std::remove_reference<T3>::type,
          typename T2 = typename std::iterator_traits<T1>::value_type,
          typename = typename std::enable_if<
              std::is_base_of<std::bidirectional_iterator_tag,
                              typename std::iterator_traits<
                                  T1>::iterator_category>::value &&
              !std::is_pointer<T1>::value &&
              std::is_lvalue_reference<T3>::value>::type>
T1 &f11(T3 &&a0) {
  return --a0;
}

template <
    typename T1, typename T2 = typename std::iterator_traits<T1>::value_type,
    typename = typename std::enable_if<
        std::is_base_of<
            std::bidirectional_iterator_tag,
            typename std::iterator_traits<T1>::iterator_category>::value &&
        !std::is_pointer<T1>::value>::type>
decltype(std::declval<T1 &>()--) f12(T1 &a0, int) {
  return a0--;
}

template <
    typename T1, typename T2 = typename std::iterator_traits<T1>::value_type,
    typename = typename std::enable_if<
        std::is_base_of<
            std::random_access_iterator_tag,
            typename std::iterator_traits<T1>::iterator_category>::value &&
        !std::is_pointer<T1>::value>::type>
T1 &f13(T1 &a0, typename std::iterator_traits<T1>::difference_type a1) {
  return a0 += a1;
}

template <
    typename T1, typename T2 = typename std::iterator_traits<T1>::value_type,
    typename = typename std::enable_if<
        std::is_base_of<
            std::random_access_iterator_tag,
            typename std::iterator_traits<T1>::iterator_category>::value &&
        !std::is_pointer<T1>::value>::type>
T1 &f14(T1 &a0, typename std::iterator_traits<T1>::difference_type a1) {
  return a0 -= a1;
}

template <
    typename T1, typename T2 = typename std::iterator_traits<T1>::value_type,
    typename = typename std::enable_if<
        std::is_base_of<
            std::random_access_iterator_tag,
            typename std::iterator_traits<T1>::iterator_category>::value &&
        !std::is_pointer<T1>::value>::type>
T1 f15(const T1 &a0, typename std::iterator_traits<T1>::difference_type a1) {
  return a0 + a1;
}

template <
    typename T1, typename T2 = typename std::iterator_traits<T1>::value_type,
    typename = typename std::enable_if<
        std::is_base_of<
            std::random_access_iterator_tag,
            typename std::iterator_traits<T1>::iterator_category>::value &&
        !std::is_pointer<T1>::value>::type>
T1 f16(typename std::iterator_traits<T1>::difference_type a0, const T1 &a1) {
  return a0 + a1;
}

template <
    typename T1, typename T2 = typename std::iterator_traits<T1>::value_type,
    typename = typename std::enable_if<
        std::is_base_of<
            std::random_access_iterator_tag,
            typename std::iterator_traits<T1>::iterator_category>::value &&
        !std::is_pointer<T1>::value>::type>
T1 f17(const T1 &a0, typename std::iterator_traits<T1>::difference_type a1) {
  return a0 - a1;
}

template <
    typename T1, typename T2 = typename std::iterator_traits<T1>::value_type,
    typename = typename std::enable_if<
        std::is_base_of<
            std::random_access_iterator_tag,
            typename std::iterator_traits<T1>::iterator_category>::value &&
        !std::is_pointer<T1>::value>::type>
typename std::iterator_traits<T1>::difference_type f18(const T1 &a0,
                                                       const T1 &a1) {
  return a0 - a1;
}

template <
    typename T1, typename T2 = typename std::iterator_traits<T1>::value_type,
    typename = typename std::enable_if<
        std::is_base_of<
            std::random_access_iterator_tag,
            typename std::iterator_traits<T1>::iterator_category>::value &&
        !std::is_pointer<T1>::value>::type>
typename std::iterator_traits<T1>::reference
f19(const T1 &a0, typename std::iterator_traits<T1>::difference_type a1) {
  return a0[a1];
}

template <
    typename T1, typename T2 = typename std::iterator_traits<T1>::value_type,
    typename = typename std::enable_if<
        std::is_base_of<
            std::random_access_iterator_tag,
            typename std::iterator_traits<T1>::iterator_category>::value &&
        !std::is_pointer<T1>::value>::type>
bool f20(const T1 &a0, const T1 &a1) {
  return a0 < a1;
}

template <
    typename T1, typename T2 = typename std::iterator_traits<T1>::value_type,
    typename = typename std::enable_if<
        std::is_base_of<
            std::random_access_iterator_tag,
            typename std::iterator_traits<T1>::iterator_category>::value &&
        !std::is_pointer<T1>::value>::type>
bool f21(const T1 &a0, const T1 &a1) {
  return a0 > a1;
}

template <
    typename T1, typename T2 = typename std::iterator_traits<T1>::value_type,
    typename = typename std::enable_if<
        std::is_base_of<
            std::random_access_iterator_tag,
            typename std::iterator_traits<T1>::iterator_category>::value &&
        !std::is_pointer<T1>::value>::type>
bool f22(const T1 &a0, const T1 &a1) {
  return a0 <= a1;
}

template <
    typename T1, typename T2 = typename std::iterator_traits<T1>::value_type,
    typename = typename std::enable_if<
        std::is_base_of<
            std::random_access_iterator_tag,
            typename std::iterator_traits<T1>::iterator_category>::value &&
        !std::is_pointer<T1>::value>::type>
bool f23(const T1 &a0, const T1 &a1) {
  return a0 >= a1;
}

template <
    typename T1, typename T2,
    typename = typename std::enable_if<
        (std::is_base_of<
             std::input_iterator_tag,
             typename std::iterator_traits<T1>::iterator_category>::value ||
         std::is_base_of<
             std::output_iterator_tag,
             typename std::iterator_traits<T1>::iterator_category>::value) &&
        (std::is_base_of<
             std::input_iterator_tag,
             typename std::iterator_traits<T2>::iterator_category>::value ||
         std::is_base_of<
             std::output_iterator_tag,
             typename std::iterator_traits<T2>::iterator_category>::value) &&
        !std::is_same<T1, T2>::value && std::is_convertible<T2, T1>::value &&
        !std::is_pointer<T1>::value && !std::is_pointer<T2>::value>::type>
T1 f24(const T2 &a0) {
  return T1(a0);
}

template <
    typename T3, typename T1 = typename std::remove_reference<T3>::type,
    typename T2 = typename std::iterator_traits<T1>::value_type,
    typename = typename std::enable_if<
        (std::is_base_of<
             std::input_iterator_tag,
             typename std::iterator_traits<T1>::iterator_category>::value ||
         std::is_base_of<
             std::output_iterator_tag,
             typename std::iterator_traits<T1>::iterator_category>::value) &&
        !std::is_pointer<T1>::value && !std::is_reference<T3>::value>::type>
T1 &f25(T3 &&a0) {
  return ++a0;
}

template <
    typename T3, typename T1 = typename std::remove_reference<T3>::type,
    typename T2 = typename std::iterator_traits<T1>::value_type,
    typename = typename std::enable_if<
        std::is_base_of<
            std::bidirectional_iterator_tag,
            typename std::iterator_traits<T1>::iterator_category>::value &&
        !std::is_pointer<T1>::value && !std::is_reference<T3>::value>::type>
T1 &f26(T3 &&a0) {
  return --a0;
}

template <
    typename T1, typename T2,
    typename = typename std::enable_if<
        std::is_base_of<
            std::input_iterator_tag,
            typename std::iterator_traits<T1>::iterator_category>::value &&
        std::is_base_of<
            std::input_iterator_tag,
            typename std::iterator_traits<T2>::iterator_category>::value &&
        !std::is_same<T1, T2>::value && !std::is_pointer<T1>::value &&
        !std::is_pointer<T2>::value>::type>
auto f27(const T1 &a0, const T2 &a1) -> decltype(a0 == a1) {
  return a0 == a1;
}
