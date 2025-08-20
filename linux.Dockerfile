FROM ubuntu:20.04

ENV DEBIAN_FRONTEND=noninteractive

RUN apt update && apt install -y \
  build-essential \
  curl \
  libx11-dev \
  libxrandr-dev \
  libxi-dev \
  libxcb1-dev \
  libgl1-mesa-dev \
  libwayland-dev \
  libudev-dev \
  pkg-config \
  git \
  cmake \
  libssl-dev

# Install Rust
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

WORKDIR /app
COPY . .

RUN cargo build --release

CMD ["./target/release/EliteAssist"]
