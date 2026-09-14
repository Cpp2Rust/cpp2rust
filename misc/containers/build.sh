#!/usr/bin/env sh

# DO NOT RUN!
# This image is intended to be used from an OCI compose file.

set -eo pipefail

if [ -d "/opt/workdir/build" ]; then
	echo "Build directory already exists, moving it."
	if [ -d "/opt/workdir/build-backup" ]; then
		echo "Build backup directory already exists, exiting as error."
		exit 1
	else
		mv /opt/workdir/build /opt/workdir/build-backup
	fi
fi

cmake \
	-G Ninja \
	-B build \
	-D CMAKE_C_COMPILER="$(which clang-22)" \
	-D CMAKE_CXX_COMPILER="$(which clang++-22)" \
	.

cmake --build build "-j$(nproc)"
