FROM ubuntu:latest

RUN mkdir /opt/source
RUN mkdir /opt/workdir

RUN apt-get -y update
RUN apt-get -y install \
	clang++-22 \
	clang-22 \
	cmake \
	curl \
	libclang-22-dev \
	libcurl4-openssl-dev \
	libedit-dev \
	libzstd-dev \
	ninja-build \
	zlib1g-dev \
	rustup

RUN rustup toolchain install "1.96.1-x86_64-unknown-linux-gnu"
RUN rustup toolchain install "nightly-2026-06-30-x86_64-unknown-linux-gnu"
RUN rustup component add --toolchain "nightly-2026-06-30-x86_64-unknown-linux-gnu" rust-src rustc-dev llvm-tools-preview
RUN rustup component add --toolchain "1.96.1-x86_64-unknown-linux-gnu" rust-src rustc-dev llvm-tools-preview

ENV PATH="/root/.cargo/bin:${PATH}"
ENV RUST_BACKTRACE=full 

COPY cpp2rust /opt/source/cpp2rust
COPY libc-dep /opt/source/libc-dep
COPY libcc2rs /opt/source/libcc2rs
COPY libcc2rs-macros /opt/source/libcc2rs-macros
COPY cmake /opt/source/cmake
COPY tests /opt/source/tests

COPY CMakeLists.txt /opt/source/CMakeLists.txt

WORKDIR /opt/source

RUN cmake \
	-G Ninja \
	-B build \
	-D CMAKE_C_COMPILER="$(which clang-22)" \
	-D CMAKE_CXX_COMPILER="$(which clang++-22)" \
	.; \
	cmake --build build -j$(nproc)

WORKDIR /opt/workdir