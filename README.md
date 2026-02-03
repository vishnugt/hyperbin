# HyperBin

A lightweight alternative to [httpbin](https://github.com/postmanlabs/httpbin) (Python/Flask) for HTTP status code testing. Written in Rust using [Hyper](https://hyper.rs/), HyperBin delivers 25x the throughput with sub-2ms latency under identical resource constraints.

Drop-in replacement for httpbin status endpoints in CI pipelines, load tests, and service health checks where performance matters.

```
docker pull vishnugt/hyperbin
docker run -p 3001:80 vishnugt/hyperbin
```

## Endpoints

```
GET /      200 OK
GET /200   200 OK
GET /404   404 Not Found
GET /503   503 Service Unavailable
GET /{code}  any valid HTTP status code
```

## Performance

Benchmarked under identical container constraints (1 CPU, 256 MB RAM, 50 concurrent connections, 10s duration).

| Metric | HyperBin | httpbin |
|---|---|---|
| Requests/sec | ~24,000 | ~960 |
| Avg latency | ~2 ms | ~52 ms |
| p50 latency | ~1.7 ms | ~66 ms |
| p99 latency | ~7 ms | ~206 ms |

HyperBin handles roughly 25x the throughput of httpbin at 1/26th the latency under the same resource constraints.

## Quick comparison

Run both side by side and benchmark with [oha](https://github.com/hatoo/oha):

```sh
docker run -d --name httpbin  -p 3001:80 --cpus 1 --memory 256m kennethreitz/httpbin
docker run -d --name hyperbin -p 3002:80 --cpus 1 --memory 256m vishnugt/hyperbin

oha -z 10s -c 50 http://localhost:3001/status/200
oha -z 10s -c 50 http://localhost:3002/200

docker rm -f httpbin hyperbin
```
