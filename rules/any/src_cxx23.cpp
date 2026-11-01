#include <algorithm>
#include <any>
#include <exception>
#include <utility>

using t1 = std::any;

std::any f1() { return std::any(); }

void f6(std::any &a0) { return a0.reset(); }

bool f8(const std::any &a0) { return a0.has_value(); }

template <typename T1, typename... T2>
typename std::enable_if<
    std::integral_constant<
        bool, std::is_copy_constructible<std::decay_t<T1>>::value &&
                  std::is_constructible<std::decay_t<T1>, T2...>::value>::value,
    std::decay_t<T1> &>::type
f14(std::any &a0, T2 &&...a1) {
  return a0.template emplace<T1, T2...>(std::forward<T2>(a1)...);
}

template <typename T1> T1 *f19(std::any *a0) { return std::any_cast<T1>(a0); }

template <typename T1, typename... T2>
std::enable_if_t<
    std::is_constructible_v<std::any, std::in_place_type_t<T1>, T2...>,
    std::any>
f27(T2 &&...a0) {
  return std::make_any<T1>(std::forward<T2>(a0)...);
}
