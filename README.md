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

Benchmarked under identical container constraints (1 CPU, 256 MB RAM, 50 concurrent connections, 10s duration).

| Metric     | hyperbin   | httpbin    | Speedup |
|------------|------------|------------|---------|
| total reqs | ~596,660   | ~40,090    | 14.88x  |
| average    | 0.84 ms    | 12.48 ms   | 14.92x  |
| fastest    | 0.08 ms    | 0.52 ms    | 6.52x   |
| slowest    | 18.85 ms   | 48.89 ms   | 2.59x   |
| p50        | 0.80 ms    | 12.25 ms   | 15.22x  |
| p99        | 1.58 ms    | 17.35 ms   | 10.95x  |

## Reproduce

Run both side by side and benchmark with [oha](https://github.com/hatoo/oha):

```sh
docker run -d --name hyperbin -p 3001:80 --cpus 1 --memory 256m vishnugt/hyperbin
docker run -d --name httpbin  -p 3002:80 --cpus 1 --memory 256m kennethreitz/httpbin

oha -z 10s -c 50 http://localhost:3001/
oha -z 10s -c 50 http://localhost:3002/status/200
```

## Build

```sh
cargo build --release
./target/release/hyperbin
```

Listens on port 80.
