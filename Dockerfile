FROM ubuntu:latest

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

WORKDIR /opt/workdir

CMD cmake \
	-G Ninja \
	-B build \
	-D CMAKE_C_COMPILER="$(which clang-22)" \
	-D CMAKE_CXX_COMPILER="$(which clang++-22)" \
	.; \
	cmake --build build -j$(nproc)
