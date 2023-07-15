# blackhole

blackhole is a server that responds to any request with http status code 200.
For example, you can check what kind of request is notified by GitHub webhook from the access log.

![Github Action](https://github.com/watawuwu/blackhole/workflows/Test/badge.svg?branch=main)
[![Latest version](https://img.shields.io/crates/v/blackhole-bin.svg)](https://crates.io/crates/blackhole-bin)
[![Documentation](https://docs.rs/blackhole-bin/badge.svg)](https://docs.rs/crate/blackhole-bin)
![Docker Pulls](https://img.shields.io/docker/pulls/watawuwu/blackhole)
![License](https://img.shields.io/crates/l/blackhole-bin.svg)


## Getting Started

- Usage

```
Usage: blackhole [OPTIONS]

Options:
  -c, --no-color
          Color mode off
  -a, --address <ADDRESS>
          Listen address [default: 127.0.0.1]
  -p, --port <PORT>
          Listen port [env: PORT=] [default: 8080]
  -v, --verbose...
          More output per occurrence
  -q, --quiet...
          Less output per occurrence
  -h, --help
          Print help
  -V, --version
          Print version
```

- Launch server

```
# listen port is 8080
$ blackhole
Start server. addr: 127.0.0.1:8080

---

# Other terinal
$ curl -v http://127.0.0.1:8080/
$ curl -v -XPOST http://127.0.0.1:8080/
$ curl -d'param=aaa' -XPOST http://127.0.0.1:8080/xxx/yyy
$ curl -v -d '{"test": 1}' -H 'application/json' -XPOST http://127.0.0.1:8080/json

---
# access log
{"headers":{"accept":"*/*","user-agent":"curl/7.77.0"},"host":"127.0.0.1:8080","method":"GET","path":"/","query":"","req":{"size":0},"scheme":"http","timestamp":"2023-07-15T05:23:50.356541Z"}
{"headers":{"accept":"*/*","user-agent":"curl/7.77.0"},"host":"127.0.0.1:8080","method":"POST","path":"/","query":"","req":{"size":0},"scheme":"http","timestamp":"2023-07-15T05:23:53.42902Z"}
{"headers":{"accept":"*/*","content-length":"9","content-type":"application/x-www-form-urlencoded","user-agent":"curl/7.77.0"},"host":"127.0.0.1:8080","method":"POST","path":"/xxx/yyy","query":"","req":{"body":"param=aaa","size":9},"scheme":"http","timestamp":"2023-07-15T05:23:56.055892Z"}
{"headers":{"accept":"*/*","content-length":"11","content-type":"application/x-www-form-urlencoded","user-agent":"curl/7.77.0"},"host":"127.0.0.1:8080","method":"POST","path":"/json","query":"","req":{"body":{"test":1},"size":11},"scheme":"http","timestamp":"2023-07-15T05:24:00.304255Z"}
```

## Installing

- Install binary directly

```
❯❯ curl --tlsv1.2 -sSf https://raw.githubusercontent.com/watawuwu/blackhole/main/install.sh | sh
```

- Install with cargo

```
❯❯ cargo install blackhole-bin
```

## Contributing

Please read [CONTRIBUTING.md](https://gist.github.com/PurpleBooth/b24679402957c63ec426) for details on our code of conduct, and the process for submitting pull requests to us.

## Versioning

We use [SemVer](http://semver.org/) for versioning.

## License
This project is licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

## Authors

* [Wataru Matsui](watawuwu@3bi.tech)
