// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <dirent.h>

using t1 = DIR *;

DIR *f1(const char *name) { return opendir(name); }

struct dirent *f2(DIR *dirp) { return readdir(dirp); }

int f3(DIR *dirp) { return closedir(dirp); }
