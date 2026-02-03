# HyperBin

A lightweight alternative to [httpbin](https://github.com/postmanlabs/httpbin) (Python/Flask) for HTTP status code testing. Written in Rust using [Hyper](https://hyper.rs/), HyperBin delivers 15x the throughput with sub-1ms p50 latency under identical resource constraints.

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

| Metric    | hyperbin     | httpbin    | Improvement (times)    |
|-----------|--------------|------------|-----------------------|
| throughput| ~596,660     | ~40,090    | 14.88x more requests  |
| average   | 0.84 ms      | 12.48 ms   | 14.92x faster         |
| fastest   | 0.08 ms      | 0.52 ms    | 6.52x faster          |
| slowest   | 18.85 ms     | 48.89 ms   | 2.59x faster          |
| p50       | 0.80 ms      | 12.25 ms   | 15.22x faster         |
| p99       | 1.58 ms      | 17.35 ms   | 10.95x faster         |

HyperBin handles roughly 15x the throughput of httpbin at 1/15th of the p50 latency under the same resource constraints.

## Quick comparison

Run both side by side and benchmark with [oha](https://github.com/hatoo/oha):

```sh
docker run -d --name hyperbin -p 3001:80 --cpus 1 --memory 256m vishnugt/hyperbin
docker run -d --name httpbin  -p 3002:80 --cpus 1 --memory 256m kennethreitz/httpbin

oha -z 10s -c 50 http://localhost:3001/
oha -z 10s -c 50 http://localhost:3002/status/200
```
