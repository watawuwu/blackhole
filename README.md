# blackhole

blackhole is a server that responds to any request with http status code 200.  
For example, you can check what kind of request is notified by GitHub webhook from the access log.

[![Azure devops](https://img.shields.io/azure-devops/build/sabi-dev/tools/xxx.svg)](https://dev.azure.com/sabi-dev/blackhole/_build?definitionId=xxx)
[![Latest version](https://img.shields.io/crates/v/blackhole.svg)](https://crates.io/crates/blackhole)
[![Documentation](https://docs.rs/blackhole/badge.svg)](https://docs.rs/crate/blackhole)
[![Docker](https://img.shields.io/docker/build/watawuwu/blackhole.svg)](https://cloud.docker.com/repository/docker/watawuwu/blackhole/)
![License](https://img.shields.io/crates/l/blackhole.svg)


## Getting Started

- Usage

```
USAGE:
    blackhole [FLAGS] [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
    -v, --verbose    Verbose mode (-v, -vv, -vvv, etc.)

OPTIONS:
    -a, --addr <addr>              Listen address [default: 0.0.0.0]
    -m, --max_chars <max_chars>    Max display body chars [default: 1024]
    -p, --port <port>              Listen port [default: 3000]
```

- Launch server

```
# listen port is 3000
❯❯ blackhole

---

# Other terinal
❯❯ curl -v  http://127.0.0.1:3000/
> GET / HTTP/1.1
> Host: 127.0.0.1:3000
> User-Agent: curl/7.54.0
> Accept: */*
>
< HTTP/1.1 200 OK
< content-length: 0
< date: Sat, 17 Aug 2019 14:33:43 GMT
<

❯❯ curl -v -XPOST http://127.0.0.1:3000/
> POST / HTTP/1.1
> Host: 127.0.0.1:3000
> User-Agent: curl/7.54.0
> Accept: */*
>
< HTTP/1.1 200 OK
< content-length: 0
< date: Sat, 17 Aug 2019 14:34:01 GMT
<

❯❯ curl -d'param=aaa' -XPOST http://127.0.0.1:3000/xxx/yyy
*   Trying 127.0.0.1...
* TCP_NODELAY set
* Connected to 127.0.0.1 (127.0.0.1) port 3000 (#0)
> POST /xxx/yyy HTTP/1.1
> Host: 127.0.0.1:3000
> User-Agent: curl/7.54.0
> Accept: */*
> Content-Length: 9
> Content-Type: application/x-www-form-urlencoded
>
* upload completely sent off: 9 out of 9 bytes
< HTTP/1.1 200 OK
< content-length: 0
< date: Sat, 17 Aug 2019 14:33:18 GMT
<

---
# access log
{"method": "GET", "path": "/", "query": "", "headers": {"host": "127.0.0.1:3000", "user-agent": "curl/7.54.0", "accept": "*/*"}, "body": "", "ts": "2019-08-17T23:27:58+09:00"}
{"method": "POST", "path": "/", "query": "", "headers": {"host": "127.0.0.1:3000", "user-agent": "curl/7.54.0", "accept": "*/*"}, "body": "", "ts": "2019-08-17T23:29:21+09:00"}
{"method": "POST", "path": "/xxx/yyy", "query": "", "headers": {"host": "127.0.0.1:3000", "user-agent": "curl/7.54.0", "accept": "*/*", "content-length": "9", "content-type": "application/x-www-form-urlencoded"}, "body": "param=aaa", "ts": "2019-08-17T23:31:44+09:00"}
```

## Installing

- Install binary directly

```
❯❯ curl --tlsv1.2 -sSf https://raw.githubusercontent.com/watawuwu/blackhole/master/install.sh | sh
```

- Compile and install

```
❯❯ git clone https://github.com/watawuwu/blackhole.git && cd blackhole

❯❯ make install
```

- Install with cargo

```
❯❯ cargo install blackhole-bin
```

## Contributing

Please read [CONTRIBUTING.md](https://gist.github.com/PurpleBooth/b24679402957c63ec426) for details on our code of conduct, and the process for submitting pull requests to us.

## Versioningx

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
