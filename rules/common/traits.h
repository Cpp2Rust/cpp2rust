// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

template <bool B, typename T = void> struct EnableIf {};

template <typename T> struct EnableIf<true, T> {
  using type = T;
};

template <typename A, typename B> struct IsSame {
  static constexpr bool value = false;
};

template <typename T> struct IsSame<T, T> {
  static constexpr bool value = true;
};

template <typename T> struct RemoveConst {
  using type = T;
};

template <typename T> struct RemoveConst<const T> {
  using type = T;
};

template <typename T, typename D, typename... Args> struct FindAlloc {
  using type = D;
};

template <typename T, typename D, template <typename, typename> class C,
          typename A, typename... Args>
struct FindAlloc<T, D, C<T, A>, Args...> {
  using type = A;
};

template <typename T, typename D, typename Arg, typename... Args>
struct FindAlloc<T, D, Arg, Args...> : FindAlloc<T, D, Args...> {};

template <typename It, typename T, typename D> struct AllocOf {
  using type = D;
};

template <template <typename...> class I, typename... Args, typename T,
          typename D>
struct AllocOf<I<Args...>, T, D> : FindAlloc<T, D, Args...> {};

template <typename It, typename T>
using SameAs = typename EnableIf<IsSame<It, T>::value>::type;

template <typename It> using ValueOf = typename It::value_type;

template <typename It>
using KeyOf = typename RemoveConst<typename It::value_type::first_type>::type;

template <typename It> using MappedOf = typename It::value_type::second_type;

template <typename It, typename D>
using AllocatorOf = typename AllocOf<It, ValueOf<It>, D>::type;
