FROM rust:1.47-buster AS builder
RUN apt-get -y update && \
	apt-get upgrade -y && \
	apt-get install -y --no-install-recommends \
	curl make cmake file ca-certificates cargo \
	g++ gcc-aarch64-linux-gnu g++-aarch64-linux-gnu \
	libc6-dev-arm64-cross binutils-aarch64-linux-gnu \
	&& \
	apt-get clean

# show backtraces
ENV RUST_BACKTRACE 1

WORKDIR /openethereum
COPY . /openethereum

RUN cargo build --features final --target x86_64-unknown-linux-gnu --verbose

FROM ubuntu:20.04

# show backtraces
ENV RUST_BACKTRACE 1

# curl and jq are installed to help create health and readiness checks on Kubernetes
RUN apt-get -y update && \
  apt-get upgrade -y && \
  apt-get install -y --no-install-recommends \
  g++ gcc-aarch64-linux-gnu g++-aarch64-linux-gnu \
  libc6-dev-arm64-cross binutils-aarch64-linux-gnu \
  curl jq && \
  apt-get clean
  

EXPOSE 8080 8545 8180

WORKDIR /home/openethereum

RUN mkdir -p /home/openethereum/.local/share/io.parity.ethereum/
COPY --from=builder /openethereum/target/x86_64-unknown-linux-gnu/debug/openethereum ./

ENTRYPOINT ["/home/openethereum/openethereum"]
