// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#pragma once

#include <execution>
#include <locale>
#include <memory>
#include <string>
#include <type_traits>
#include <variant>
#include <vector>

#include "rule_tags.h"

namespace Synthesis {
template <typename...> struct Slot;
using BindExisting = void;
using BindAny = void *;
} // namespace Synthesis

#define ARG(...) __VA_ARGS__

#define DECLARE_HINT(name) template <int = 0> struct name

#define DECLARE_PARAMETERIZABLE_HINT(name)                                     \
  struct [[clang::annotate(CPP2RUST_PARAMETERIZABLE_RULE_TAG)]] name

#define DECLARE_BUILTIN_HINT(name, type)                                       \
  using name [[clang::annotate(CPP2RUST_BUILTIN_RULE_TAG)]] = type;

#define DECLARE_PARAMETERIZABLE_BUILTIN_HINT(name, type)                       \
  using name [[clang::annotate(CPP2RUST_BUILTIN_RULE_TAG),                     \
               clang::annotate(CPP2RUST_PARAMETERIZABLE_RULE_TAG)]] = type;

#define DECLARE_NON_TYPE_HINT(name, type, expr) constexpr type name = expr;

DECLARE_BUILTIN_HINT(Integer, int)

DECLARE_BUILTIN_HINT(Long, long)

DECLARE_BUILTIN_HINT(Char, char)

DECLARE_BUILTIN_HINT(WChar, wchar_t)

DECLARE_BUILTIN_HINT(UnsignedInteger, unsigned)

DECLARE_BUILTIN_HINT(ErrorCodeEnum, std::io_errc)

DECLARE_BUILTIN_HINT(ErrorConditionEnum, std::errc)

#if defined(__linux__)
DECLARE_BUILTIN_HINT(ExecutionPolicy, std::execution::parallel_policy)
#endif

DECLARE_HINT(Plain){};

DECLARE_HINT(Comparable) {
  template <typename Other> bool operator==(const Other &) const;
  template <typename Other> bool operator!=(const Other &) const;
  template <typename Other> bool operator<(const Other &) const;
  template <typename Other> bool operator>(const Other &) const;
  template <typename Other> bool operator<=(const Other &) const;
  template <typename Other> bool operator>=(const Other &) const;
#if __cplusplus >= 202002L
  template <typename Other>
  std::strong_ordering operator<=>(const Other &) const;
#endif
};

DECLARE_HINT(MoveAssignable) {
  template <typename Other> MoveAssignable &operator=(Other &&);
  template <typename Other> MoveAssignable &operator=(Other &&) const;
};

DECLARE_HINT(NonConvertibleComparable) {
  template <typename Other> bool operator==(const Other &) const;
  template <typename Other> bool operator!=(const Other &) const;
  template <typename Other> bool operator<(const Other &) const;
  template <typename Other> bool operator>(const Other &) const;
  template <typename Other> bool operator<=(const Other &) const;
  template <typename Other> bool operator>=(const Other &) const;
#if __cplusplus >= 202002L
  template <typename Other>
  std::strong_ordering operator<=>(const Other &) const;
#endif
};

template <typename> struct is_non_convertible : std::false_type {};

template <auto N>
struct is_non_convertible<NonConvertibleComparable<N>> : std::true_type {};

template <typename Tp>
inline constexpr bool is_non_convertible_v = is_non_convertible<Tp>::value;

template <typename T>
using enable_unless_non_convertible_t = std::enable_if_t<!is_non_convertible_v<
    std::remove_cv_t<std::remove_pointer_t<std::remove_reference_t<T>>>>>;

DECLARE_HINT(ExplicitlyConvertible) {
  template <typename Other, typename = enable_unless_non_convertible_t<Other>>
  explicit operator Other() const;
};

DECLARE_HINT(ImplicitlyConvertible) {
  template <typename Other, typename = enable_unless_non_convertible_t<Other>>
  operator Other() const;
};

template <typename T = Synthesis::Slot<ImplicitlyConvertible<>>>
DECLARE_PARAMETERIZABLE_BUILTIN_HINT(Variant, std::variant<T>)

template <typename T1 = Synthesis::Slot<ImplicitlyConvertible<>>,
          typename T2 = Synthesis::Slot<ImplicitlyConvertible<>>>
DECLARE_PARAMETERIZABLE_BUILTIN_HINT(TupleLike, ARG(std::tuple<T1, T2>))

DECLARE_HINT(StringLike) {
  template <typename CharT, typename Traits,
            typename = enable_unless_non_convertible_t<CharT>>
  operator std::basic_string<CharT, Traits>() const;

  template <typename CharT, typename Traits,
            typename = enable_unless_non_convertible_t<CharT>>
  operator std::basic_string_view<CharT, Traits>() const;
};

template <typename T1, typename T2>
using enable_any_convertible_t =
    std::enable_if_t<std::is_convertible_v<T1, T2> ||
                     std::is_convertible_v<T2, T1>>;

template <typename InnerT = Synthesis::Slot<
              Synthesis::BindExisting, Comparable<>, ExplicitlyConvertible<>,
              ImplicitlyConvertible<>, MoveAssignable<>>,
          typename DiffT = Synthesis::Slot<Long>>
DECLARE_PARAMETERIZABLE_HINT(Iterator) {
  using value_type = InnerT;
  using difference_type = DiffT;
  using pointer = value_type *;
  using reference = value_type &;
  using iterator_category = std::random_access_iterator_tag;
#if __cplusplus >= 202002L
  using iterator_concept = std::contiguous_iterator_tag;
#endif

  reference operator*() const;
  pointer operator->() const;
  reference operator[](difference_type) const;

  Iterator &operator++();
  Iterator operator++(int) const;
  Iterator &operator--();
  Iterator operator--(int) const;

  Iterator &operator+=(difference_type);
  Iterator &operator-=(difference_type);
  Iterator operator+(difference_type) const;
  Iterator operator-(difference_type) const;
  difference_type operator-(const Iterator &) const;
  friend Iterator operator+(difference_type, const Iterator &);

  template <typename OInnerT, typename ODiffT,
            typename = enable_any_convertible_t<OInnerT, InnerT>,
            typename = enable_any_convertible_t<ODiffT, DiffT>>
  bool operator==(const Iterator<OInnerT, ODiffT> &) const;

  template <typename OInnerT, typename ODiffT,
            typename = enable_any_convertible_t<OInnerT, InnerT>,
            typename = enable_any_convertible_t<ODiffT, DiffT>>
  bool operator!=(const Iterator<OInnerT, ODiffT> &) const;

  template <typename OInnerT, typename ODiffT,
            typename = enable_any_convertible_t<OInnerT, InnerT>,
            typename = enable_any_convertible_t<ODiffT, DiffT>>
  bool operator<(const Iterator<OInnerT, ODiffT> &) const;

  template <typename OInnerT, typename ODiffT,
            typename = enable_any_convertible_t<OInnerT, InnerT>,
            typename = enable_any_convertible_t<ODiffT, DiffT>>
  bool operator>(const Iterator<OInnerT, ODiffT> &) const;

  template <typename OInnerT, typename ODiffT,
            typename = enable_any_convertible_t<OInnerT, InnerT>,
            typename = enable_any_convertible_t<ODiffT, DiffT>>
  bool operator<=(const Iterator<OInnerT, ODiffT> &) const;

  template <typename OInnerT, typename ODiffT,
            typename = enable_any_convertible_t<OInnerT, InnerT>,
            typename = enable_any_convertible_t<ODiffT, DiffT>>
  bool operator>=(const Iterator<OInnerT, ODiffT> &) const;

#if __cplusplus >= 202002L
  template <typename OInnerT, typename ODiffT,
            typename = enable_any_convertible_t<OInnerT, InnerT>,
            typename = enable_any_convertible_t<ODiffT, DiffT>>
  std::strong_ordering operator<=>(const Iterator<OInnerT, ODiffT> &) const;
#endif

#if defined(__linux__)
  template <typename Other = InnerT,
            typename = std::enable_if_t<std::is_same_v<Other, bool>>>
  operator std::_Bit_const_iterator() const;

  template <typename Other = InnerT,
            typename = std::enable_if_t<std::is_same_v<Other, bool>>>
  operator std::_Bit_iterator() const;
#endif

  template <typename OInnerT, typename ODiffT,
            typename = enable_any_convertible_t<OInnerT, InnerT>,
            typename = enable_any_convertible_t<ODiffT, DiffT>>
  operator Iterator<OInnerT, ODiffT>() const;
};

template <typename InnerT = Synthesis::Slot<
              Synthesis::BindExisting, Comparable<>, ExplicitlyConvertible<>,
              ImplicitlyConvertible<>, MoveAssignable<>>,
          typename DiffT = Synthesis::Slot<Long>>
DECLARE_PARAMETERIZABLE_HINT(InputIterator) {
  using value_type = InnerT;
  using difference_type = DiffT;
  using pointer = const value_type *;
  using reference = const value_type &;
  using iterator_concept = std::input_iterator_tag;
  using iterator_category = std::input_iterator_tag;

  reference operator*() const;
  pointer operator->() const;
  InputIterator &operator++();
  InputIterator operator++(int);

  template <typename OInnerT, typename ODiffT,
            typename = enable_any_convertible_t<OInnerT, InnerT>,
            typename = enable_any_convertible_t<ODiffT, DiffT>>
  bool operator==(const InputIterator<OInnerT, ODiffT> &) const;

  template <typename OInnerT, typename ODiffT,
            typename = enable_any_convertible_t<OInnerT, InnerT>,
            typename = enable_any_convertible_t<ODiffT, DiffT>>
  bool operator!=(const InputIterator<OInnerT, ODiffT> &) const;

  template <typename OInnerT, typename ODiffT,
            typename = enable_any_convertible_t<OInnerT, InnerT>,
            typename = enable_any_convertible_t<ODiffT, DiffT>>
  bool operator<(const InputIterator<OInnerT, ODiffT> &) const;

  template <typename OInnerT, typename ODiffT,
            typename = enable_any_convertible_t<OInnerT, InnerT>,
            typename = enable_any_convertible_t<ODiffT, DiffT>>
  bool operator>(const InputIterator<OInnerT, ODiffT> &) const;

  template <typename OInnerT, typename ODiffT,
            typename = enable_any_convertible_t<OInnerT, InnerT>,
            typename = enable_any_convertible_t<ODiffT, DiffT>>
  bool operator<=(const InputIterator<OInnerT, ODiffT> &) const;

  template <typename OInnerT, typename ODiffT,
            typename = enable_any_convertible_t<OInnerT, InnerT>,
            typename = enable_any_convertible_t<ODiffT, DiffT>>
  bool operator>=(const InputIterator<OInnerT, ODiffT> &) const;

#if __cplusplus >= 202002L
  template <typename OInnerT, typename ODiffT,
            typename = enable_any_convertible_t<OInnerT, InnerT>,
            typename = enable_any_convertible_t<ODiffT, DiffT>>
  std::strong_ordering operator<=>(const InputIterator<OInnerT, ODiffT> &)
      const;
#endif

  template <typename OInnerT, typename ODiffT,
            typename = enable_any_convertible_t<OInnerT, InnerT>,
            typename = enable_any_convertible_t<ODiffT, DiffT>>
  operator InputIterator<OInnerT, ODiffT>() const;
};

DECLARE_HINT(Callable) {
  using is_transparent = void;

  template <typename... Args> int operator()(Args...) noexcept;
  template <typename... Args> int operator()(Args...) const noexcept;

  template <typename Return, typename... Args>
  Return operator()(Args...) noexcept;

  template <typename Return, typename... Args>
  Return operator()(Args...) const noexcept;

  template <typename T> operator std::default_delete<T>() const;
};

template <typename T = Synthesis::Slot<Synthesis::BindExisting, Plain<>>>
DECLARE_PARAMETERIZABLE_HINT(Allocator) {
  using value_type = T;

  Allocator() noexcept;
  template <typename U> Allocator(const Allocator<U> &) noexcept;

  T *allocate(std::size_t);
  void deallocate(T *, std::size_t);

  template <typename U> bool operator==(const Allocator<U> &) const noexcept;
  template <typename U> bool operator!=(const Allocator<U> &) const noexcept;
};

DECLARE_HINT(BoolConstant) {
  static constexpr bool value = false;
  using value_type = bool;
  constexpr operator value_type() const noexcept { return value; }
  constexpr value_type operator()() const noexcept { return value; }
};

template <typename InnerT = Synthesis::Slot<
              Synthesis::BindExisting, Comparable<>, ExplicitlyConvertible<>,
              ImplicitlyConvertible<>, MoveAssignable<>>,
          typename SizeT = Synthesis::Slot<Long>,
          typename DiffT = Synthesis::Slot<Long>>
DECLARE_PARAMETERIZABLE_HINT(Container) {
  using value_type = InnerT;
  using size_type = SizeT;
  using difference_type = DiffT;
  using reference = value_type &;
  using const_reference = const value_type &;
  using iterator = Iterator<value_type, DiffT>;
  using const_iterator = Iterator<const value_type>;
  using reverse_iterator = Iterator<value_type>;
  using const_reverse_iterator = Iterator<const value_type>;

  reference operator[](size_type) const;

  void push_back(const value_type &);
  void push_back(value_type &&);
  void push_front(const value_type &);
  void push_front(value_type &&);

  iterator insert(const_iterator, const InnerT &);
  iterator insert(const_iterator, InnerT &&);
  iterator insert(const_iterator, size_type, const InnerT &);
  template <typename InputIt> iterator insert(const_iterator, InputIt, InputIt);
  iterator insert(const_iterator, std::initializer_list<InnerT>);

  iterator begin() noexcept;
  const_iterator begin() const noexcept;
  const_iterator cbegin() const noexcept;
  reverse_iterator rbegin() noexcept;

  iterator end() noexcept;
  const_iterator end() const noexcept;
  const_iterator cend() const noexcept;
  reverse_iterator rend() noexcept;

  size_type size() const noexcept;
  bool empty() const noexcept;

  reference front();
  reference back();
  reference at(size_type);
  InnerT *data() noexcept;

  void clear() noexcept;
  void pop_back();
  template <typename... Args> reference emplace_back(Args && ...);
  iterator erase(const_iterator);
  void resize(size_type);
};

template <typename CharT = Synthesis::Slot<Synthesis::BindExisting>,
          typename IntT = Synthesis::Slot<Integer>>
DECLARE_PARAMETERIZABLE_HINT(CharTraits) {
  using char_type = CharT;
  using int_type = IntT;
  using off_type = std::streamoff;
  using pos_type = std::streampos;
  using state_type = std::mbstate_t;

  static void assign(char_type &, const char_type &);
  static char_type *assign(char_type *, std::size_t, char_type);

  static bool eq(char_type, char_type);
  static bool lt(char_type, char_type);
  static int compare(const char_type *, const char_type *, std::size_t);

  static char_type *move(char_type *, const char_type *, std::size_t);
  static char_type *copy(char_type *, const char_type *, std::size_t);

  static std::size_t length(const char_type *);

  static const char_type *find(const char_type *, std::size_t,
                               const char_type &);

  static char_type to_char_type(int_type);
  static int_type to_int_type(char_type);
  static bool eq_int_type(int_type, int_type);

  static int_type eof();
  static int_type not_eof(int_type);
};

template <typename T = Synthesis::Slot<Synthesis::BindExisting>>
DECLARE_PARAMETERIZABLE_HINT(Range) {
  T *begin();
  T *end();

  const T *begin() const;
  const T *end() const;
};

template <typename T = Synthesis::Slot<Synthesis::BindExisting>>
DECLARE_PARAMETERIZABLE_HINT(Derived) : T{};

DECLARE_HINT(Mutex) {
  void lock();
  void unlock();

  bool try_lock();
  template <typename Duration> bool try_lock_for(Duration);
  template <typename Duration> bool try_lock_until(Duration);
};

template <typename InternT = Synthesis::Slot<Char, WChar>,
          typename ExternT = Synthesis::Slot<Char>>
DECLARE_PARAMETERIZABLE_HINT(Facet)
    : public std::codecvt<InternT, ExternT, std::mbstate_t>{};

template <typename T = Synthesis::Slot<UnsignedInteger>>
DECLARE_PARAMETERIZABLE_HINT(NumberGenerator) {
  using result_type = T;
  static constexpr result_type min() { return T{}; }
  static constexpr result_type max() { return T{}; }
  result_type operator()();
};

template <typename T =
              Synthesis::Slot<Synthesis::BindExisting, ImplicitlyConvertible<>>>
DECLARE_PARAMETERIZABLE_BUILTIN_HINT(Const, const T)

template <typename T = Synthesis::Slot<Synthesis::BindExisting>>
DECLARE_PARAMETERIZABLE_BUILTIN_HINT(Pointer, T *)

template <typename T = Synthesis::Slot<Synthesis::BindExisting, Plain<>>>
DECLARE_PARAMETERIZABLE_BUILTIN_HINT(Array, T[])

template <typename T = Synthesis::Slot<Synthesis::BindExisting>>
DECLARE_PARAMETERIZABLE_BUILTIN_HINT(DefaultDelete, std::default_delete<T>)

DECLARE_NON_TYPE_HINT(NonNullInteger, int, 1)

#if __cplusplus >= 202002L
    DECLARE_NON_TYPE_HINT(SizedSubRangeKind, std::ranges::subrange_kind,
                          std::ranges::subrange_kind::sized)
#endif

#define Plain Plain<__COUNTER__>
#define Comparable Comparable<__COUNTER__>
#define MoveAssignable MoveAssignable<__COUNTER__>
#define NonConvertibleComparable NonConvertibleComparable<__COUNTER__>
#define ExplicitlyConvertible ExplicitlyConvertible<__COUNTER__>
#define ImplicitlyConvertible ImplicitlyConvertible<__COUNTER__>
#define StringLike StringLike<__COUNTER__>
#define Callable Callable<__COUNTER__>
#define BoolConstant BoolConstant<__COUNTER__>
#define Mutex Mutex<__COUNTER__>
