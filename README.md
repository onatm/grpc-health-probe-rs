# grpc-health-probe-rs

rust implementation of the [`grpc-health-probe`](https://github.com/grpc-ecosystem/grpc-health-probe). It allows you to query health of gRPC services that expose their status through the [gRPC Health Checking Protocol](https://github.com/grpc/grpc/blob/master/doc/health-checking.md).

[![build](https://github.com/onatm/grpc-health-probe-rs/actions/workflows/build.yaml/badge.svg)](https://github.com/onatm/grpc-health-probe-rs/actions/workflows/build.yaml)

This command-line utility makes a RPC to `/grpc.health.v1.Health/Check`. If it responds with a `SERVING` status, the `grpc_health_probe` will exit with success, otherwise it will exit with a non-zero exit code (documented below).

This port does not provide a way to skip TLS verification, see [configuring TLS](#health-checking-tls-servers).

## Installation

### From binary

You can directly [download the grpc_health_probe executable](https://github.com/onatm/grpc-health-probe-rs/releases).

### Install from crates.io

```sh
cargo install grpc_health_probe
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

then put the resulting `target/release/grpc_health_probe` executable on your PATH.

## Examples

```sh
$ grpc_health_probe --addr=https://localhost:5000
status: Serving
```

```sh
$ grpc_health_probe --addr=https://localhost:9999 --connect-timeout=250 --rpc-timeout=100
error: health rpc failed: Timeout expired
```

## Example: gRPC health checking on Kubernetes

You can bundle the `grpc_health_probe` binary in your container image and define liveness and/or readiness checks for your gRPC server pods using `exec` probes.

```yaml
spec:
  containers:
  - name: server
    image: "[YOUR-DOCKER-IMAGE]"
    ports:
    - containerPort: 5000
    readinessProbe:
      exec:
        command: [
          "/bin/grpc_health_probe",
          "--addr=https://0.0.0.0:5000",
          "--service=queries.v1.QueriesAPI",
          "--tls",
          "--tls-server-name=deployment.namespace.svc.cluster.local"
        ]
      initialDelaySeconds: 5
    livenessProbe:
      exec:
        command: [
          "/bin/grpc_health_probe",
          "--addr=https://0.0.0.0:5000",
          "--service=queries.v1.QueriesAPI",
          "--tls",
          "--tls-server-name=deployment.namespace.svc.cluster.local"
        ]
      initialDelaySeconds: 10
```

## Health Checking TLS Servers

| Option                  | Description                                                                 |
| :---------------------- | --------------------------------------------------------------------------- |
| **`--tls`**             | use TLS (default: false)                                                    |
| **`--tls-ca-cert`**     | path to file containing CA certificates (to override system root CAs)       |
| **`--tls-server-name`** | override the hostname used to verify the server certificate                 |
| **`--tls-client-cert`** | path to file containing client certificate for authenticating to the server |
| **`--tls-client-key`**  | path to file containing client private key for authenticating to the server |

## Other Available Flags

| Option                  | Description                                                                        |
| :---------------------- | ---------------------------------------------------------------------------------- |
| **`--connect-timeout`** | timeout in milliseconds for establishing connection                                |
| **`--rpc-timeout`**     | timeout in milliseconds for health check rpc                                       |
| **`--user-agent`**      | user-agent header value of health check requests (default: grpc_health_probe_rs)   |
| **`--service`**         | service name to check (default: "") - empty string is convention for server health |

## Exit codes

It is not recommended to rely on specific exit statuses. Any failure will be
a non-zero exit code.

| Exit Code | Description                                                |
| :-------: | ---------------------------------------------------------- |
|   **0**   | success: rpc response is `SERVING`.                        |
|   **1**   | failure: invalid command-line arguments                    |
|   **2**   | failure: connection failed or timed out                    |
|   **3**   | failure: rpc failed or timed out                           |
|   **4**   | failure: rpc successful, but the response is not `SERVING` |
