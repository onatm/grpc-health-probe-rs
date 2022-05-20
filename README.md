# grpc-health-probe-rs

rust implementation of the `grpc-health-probe`. It allows you to query health of gRPC services that expose their status through the [gRPC Health Checking Protocol](https://github.com/grpc/grpc/blob/master/doc/health-checking.md).

## Installation

### From binary

You can directly [download the grpc-health-probe executable](https://github.com/onatm/grpc-health-probe/releases).

### Install from crates.io

```sh
cargo install grpc-health-probe
```

### Build Manually

Clone the repo and run:

```sh
cargo install --path .
```

Alternatively, run:

```sh
cargo build --release
```

then put the resulting `target/release/grpc-health-probe` executable on your PATH.
