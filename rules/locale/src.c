// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <locale.h>

char *f1(int category, const char *locale) {
  return setlocale(category, locale);
}
