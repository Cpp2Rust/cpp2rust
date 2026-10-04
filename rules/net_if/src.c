// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#ifdef CPP2RUST_USE_INCLUDES
#include <net/if.h>
#endif

unsigned int CPP2RUST_EXPR_RULE(1)(const char *ifname) {
  return if_nametoindex(ifname);
}
