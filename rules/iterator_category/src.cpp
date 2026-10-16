// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <iterator>
#include <type_traits>
#include <utility>

#include "../common/init.h"

template <
    typename It, typename T = typename std::iterator_traits<It>::value_type,
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
    typename It, typename T = typename std::iterator_traits<It>::value_type,
    typename = typename std::enable_if<
        std::is_base_of<
            std::output_iterator_tag,
            typename std::iterator_traits<It>::iterator_category>::value &&
        !std::is_pointer<It>::value>::type>
using t2 = It;

template <
    typename It, typename T = typename std::iterator_traits<It>::value_type,
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
    typename It, typename T = typename std::iterator_traits<It>::value_type,
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
    typename It, typename T = typename std::iterator_traits<It>::value_type,
    typename = typename std::enable_if<
        std::is_base_of<
            std::random_access_iterator_tag,
            typename std::iterator_traits<It>::iterator_category>::value &&
        !std::is_pointer<It>::value>::type>
using t5 = It;

template <
    typename It,
    typename = typename std::enable_if<
        (std::is_base_of<
             std::input_iterator_tag,
             typename std::iterator_traits<It>::iterator_category>::value ||
         std::is_base_of<
             std::output_iterator_tag,
             typename std::iterator_traits<It>::iterator_category>::value) &&
        !std::is_pointer<It>::value>::type>
It f1(const It &a0) {
  return It(a0);
}

template <
    typename It,
    typename = typename std::enable_if<
        (std::is_base_of<
             std::input_iterator_tag,
             typename std::iterator_traits<It>::iterator_category>::value ||
         std::is_base_of<
             std::output_iterator_tag,
             typename std::iterator_traits<It>::iterator_category>::value) &&
        !std::is_pointer<It>::value>::type>
It f2(It &&a0) {
  return It(std::move(a0));
}

template <
    typename It,
    typename = typename std::enable_if<
        (std::is_base_of<
             std::input_iterator_tag,
             typename std::iterator_traits<It>::iterator_category>::value ||
         std::is_base_of<
             std::output_iterator_tag,
             typename std::iterator_traits<It>::iterator_category>::value) &&
        !std::is_pointer<It>::value>::type>
It &f3(It &a0, const It &a1) {
  return a0 = a1;
}

template <
    typename It,
    typename = typename std::enable_if<
        (std::is_base_of<
             std::input_iterator_tag,
             typename std::iterator_traits<It>::iterator_category>::value ||
         std::is_base_of<
             std::output_iterator_tag,
             typename std::iterator_traits<It>::iterator_category>::value) &&
        !std::is_pointer<It>::value>::type>
It &f4(It &a0, It &&a1) {
  return a0 = std::move(a1);
}

template <
    typename It,
    typename = typename std::enable_if<
        (std::is_base_of<
             std::input_iterator_tag,
             typename std::iterator_traits<It>::iterator_category>::value ||
         std::is_base_of<
             std::output_iterator_tag,
             typename std::iterator_traits<It>::iterator_category>::value) &&
        !std::is_pointer<It>::value>::type>
decltype(*std::declval<It &>()) f5(It &a0) {
  return *a0;
}

template <
    typename It,
    typename = typename std::enable_if<
        (std::is_base_of<
             std::input_iterator_tag,
             typename std::iterator_traits<It>::iterator_category>::value ||
         std::is_base_of<
             std::output_iterator_tag,
             typename std::iterator_traits<It>::iterator_category>::value) &&
        !std::is_pointer<It>::value>::type>
It &f6(It &a0) {
  return ++a0;
}

template <
    typename It,
    typename = typename std::enable_if<
        (std::is_base_of<
             std::input_iterator_tag,
             typename std::iterator_traits<It>::iterator_category>::value ||
         std::is_base_of<
             std::output_iterator_tag,
             typename std::iterator_traits<It>::iterator_category>::value) &&
        !std::is_pointer<It>::value>::type>
decltype(std::declval<It &>()++) f7(It &a0, int) {
  return a0++;
}

template <typename It, typename = typename std::enable_if<
                           std::is_base_of<std::input_iterator_tag,
                                           typename std::iterator_traits<
                                               It>::iterator_category>::value &&
                           !std::is_pointer<It>::value>::type>
typename std::iterator_traits<It>::pointer f8(const It &a0) {
  return a0.operator->();
}

template <typename It, typename = typename std::enable_if<
                           std::is_base_of<std::input_iterator_tag,
                                           typename std::iterator_traits<
                                               It>::iterator_category>::value &&
                           !std::is_pointer<It>::value>::type>
bool f9(const It &a0, const It &a1) {
  return a0 == a1;
}

template <typename It, typename = typename std::enable_if<
                           std::is_base_of<std::input_iterator_tag,
                                           typename std::iterator_traits<
                                               It>::iterator_category>::value &&
                           !std::is_pointer<It>::value>::type>
bool f10(const It &a0, const It &a1) {
  return a0 != a1;
}

template <typename It, typename = typename std::enable_if<
                           std::is_base_of<std::bidirectional_iterator_tag,
                                           typename std::iterator_traits<
                                               It>::iterator_category>::value &&
                           !std::is_pointer<It>::value>::type>
It &f11(It &a0) {
  return --a0;
}

template <typename It, typename = typename std::enable_if<
                           std::is_base_of<std::bidirectional_iterator_tag,
                                           typename std::iterator_traits<
                                               It>::iterator_category>::value &&
                           !std::is_pointer<It>::value>::type>
decltype(std::declval<It &>()--) f12(It &a0, int) {
  return a0--;
}

template <typename It, typename = typename std::enable_if<
                           std::is_base_of<std::random_access_iterator_tag,
                                           typename std::iterator_traits<
                                               It>::iterator_category>::value &&
                           !std::is_pointer<It>::value>::type>
It &f13(It &a0, typename std::iterator_traits<It>::difference_type a1) {
  return a0 += a1;
}

template <typename It, typename = typename std::enable_if<
                           std::is_base_of<std::random_access_iterator_tag,
                                           typename std::iterator_traits<
                                               It>::iterator_category>::value &&
                           !std::is_pointer<It>::value>::type>
It &f14(It &a0, typename std::iterator_traits<It>::difference_type a1) {
  return a0 -= a1;
}

template <typename It, typename = typename std::enable_if<
                           std::is_base_of<std::random_access_iterator_tag,
                                           typename std::iterator_traits<
                                               It>::iterator_category>::value &&
                           !std::is_pointer<It>::value>::type>
It f15(const It &a0, typename std::iterator_traits<It>::difference_type a1) {
  return a0 + a1;
}

template <typename It, typename = typename std::enable_if<
                           std::is_base_of<std::random_access_iterator_tag,
                                           typename std::iterator_traits<
                                               It>::iterator_category>::value &&
                           !std::is_pointer<It>::value>::type>
It f16(typename std::iterator_traits<It>::difference_type a0, const It &a1) {
  return a0 + a1;
}

template <typename It, typename = typename std::enable_if<
                           std::is_base_of<std::random_access_iterator_tag,
                                           typename std::iterator_traits<
                                               It>::iterator_category>::value &&
                           !std::is_pointer<It>::value>::type>
It f17(const It &a0, typename std::iterator_traits<It>::difference_type a1) {
  return a0 - a1;
}

template <typename It, typename = typename std::enable_if<
                           std::is_base_of<std::random_access_iterator_tag,
                                           typename std::iterator_traits<
                                               It>::iterator_category>::value &&
                           !std::is_pointer<It>::value>::type>
typename std::iterator_traits<It>::difference_type f18(const It &a0,
                                                       const It &a1) {
  return a0 - a1;
}

template <typename It, typename = typename std::enable_if<
                           std::is_base_of<std::random_access_iterator_tag,
                                           typename std::iterator_traits<
                                               It>::iterator_category>::value &&
                           !std::is_pointer<It>::value>::type>
typename std::iterator_traits<It>::reference
f19(const It &a0, typename std::iterator_traits<It>::difference_type a1) {
  return a0[a1];
}

template <typename It, typename = typename std::enable_if<
                           std::is_base_of<std::random_access_iterator_tag,
                                           typename std::iterator_traits<
                                               It>::iterator_category>::value &&
                           !std::is_pointer<It>::value>::type>
bool f20(const It &a0, const It &a1) {
  return a0 < a1;
}

template <typename It, typename = typename std::enable_if<
                           std::is_base_of<std::random_access_iterator_tag,
                                           typename std::iterator_traits<
                                               It>::iterator_category>::value &&
                           !std::is_pointer<It>::value>::type>
bool f21(const It &a0, const It &a1) {
  return a0 > a1;
}

template <typename It, typename = typename std::enable_if<
                           std::is_base_of<std::random_access_iterator_tag,
                                           typename std::iterator_traits<
                                               It>::iterator_category>::value &&
                           !std::is_pointer<It>::value>::type>
bool f22(const It &a0, const It &a1) {
  return a0 <= a1;
}

template <typename It, typename = typename std::enable_if<
                           std::is_base_of<std::random_access_iterator_tag,
                                           typename std::iterator_traits<
                                               It>::iterator_category>::value &&
                           !std::is_pointer<It>::value>::type>
bool f23(const It &a0, const It &a1) {
  return a0 >= a1;
}
