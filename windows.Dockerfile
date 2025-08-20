FROM ubuntu:20.04

ENV DEBIAN_FRONTEND=noninteractive

# Base build tools and dependencies
RUN apt-get update && apt-get install -y \
  build-essential \
  curl \
  ca-certificates \
  pkg-config \
  git \
  cmake \
  libssl-dev \
  mingw-w64 \
  gcc-mingw-w64 \
  g++-mingw-w64 && rm -rf /var/lib/apt/lists/*

# Install Rust and the Windows GNU target
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"
RUN rustup target add x86_64-pc-windows-gnu

# Ensure the correct linker is used for the Windows GNU target
ENV CARGO_TARGET_X86_64_PC_WINDOWS_GNU_LINKER=x86_64-w64-mingw32-gcc
ENV CC_x86_64_pc_windows_gnu=x86_64-w64-mingw32-gcc
ENV CXX_x86_64_pc_windows_gnu=x86_64-w64-mingw32-g++

WORKDIR /app
COPY . .

# Build Windows artifact
RUN cargo build --release --target x86_64-pc-windows-gnu

# Default command is irrelevant for CI; keep a noop
CMD ["/bin/bash", "-lc", "echo build complete"]
