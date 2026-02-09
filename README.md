# HyperBin

A lightweight [httpbin](https://github.com/postmanlabs/httpbin) alternative for HTTP status code testing. Written in Rust using [Hyper](https://hyper.rs/).

## Quick Start

```
docker pull vishnugt/hyperbin
docker run -p 3001:80 vishnugt/hyperbin
```

## Endpoints

```
GET /{code}   returns the given HTTP status code
GET /         200 OK
GET /404      404 Not Found
GET /503      503 Service Unavailable
```

Any valid HTTP status code works. Invalid paths return `400 Bad Request`.

## Performance

Benchmarked under identical container constraints (2 CPUs, 256 MB RAM, 500 concurrent connections, 10s duration).

| Metric     | hyperbin   | httpbin    | Speedup |
|------------|------------|------------|---------|
| total reqs | 681,294    | 35,352     | 19.27x  |
| average    | 7.33 ms    | 140.54 ms  | 19.17x  |
| fastest    | 0.66 ms    | 0.50 ms    | 0.76x   |
| slowest    | 86.84 ms   | 807.54 ms  | 9.30x   |
| p50        | 7.04 ms    | 125.42 ms  | 17.82x  |
| p99        | 12.21 ms   | 577.30 ms  | 47.27x  |
| req/sec    | 68,135     | 3,584      | 19.01x  |

## Reproduce

Run both side by side and benchmark with [oha](https://github.com/hatoo/oha):

```sh
docker run -d --name hyperbin -p 3001:80 --cpus 2 --memory 256m vishnugt/hyperbin
docker run -d --name httpbin  -p 3002:80 --cpus 2 --memory 256m kennethreitz/httpbin

oha -z 10s -c 500 http://localhost:3001/
oha -z 10s -c 500 http://localhost:3002/status/200
```

## Build

```sh
cargo build --release
./target/release/hyperbin
```

Listens on port 80.
