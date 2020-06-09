# blackhole

blackhole is a server that responds to any request with http status code 200.
For example, you can check what kind of request is notified by GitHub webhook from the access log.

![Github Action](https://github.com/watawuwu/blackhole/workflows/Test/badge.svg?branch=master)
[![Latest version](https://img.shields.io/crates/v/blackhole-bin.svg)](https://crates.io/crates/blackhole-bin)
[![Documentation](https://docs.rs/blackhole-bin/badge.svg)](https://docs.rs/crate/blackhole-bin)
![Docker Pulls](https://img.shields.io/docker/pulls/watawuwu/blackhole)
![License](https://img.shields.io/crates/l/blackhole-bin.svg)


## Getting Started

- Usage

```
blackhole-bin 0.4.0
USAGE:
    blackhole [FLAGS] [OPTIONS]

FLAGS:
    -h, --help
            Prints help information

        --log-all
            Enable log output from dependencies

    -P, --pretty
            Enable pretty printing

    -q, --quiet
            Suppress all log output

    -V, --version
            Prints version information

    -v, --verbosity
            Print more log output


OPTIONS:
    -a, --address <address>
            Network address [default: 127.0.0.1]

    -p, --port <port>
            Insecure HTTP port [env: PORT=]  [default: 80]

```

- Launch server

```
# listen port is 3000
❯❯ blackhole --port 3000

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

❯❯ curl -v -d '{"test": 1}' -H 'application/json' -XPOST http://127.0.0.1:3000/json
Note: Unnecessary use of -X or --request, POST is already inferred.
*   Trying 127.0.0.1...
* TCP_NODELAY set
* Connected to 127.0.0.1 (127.0.0.1) port 3000 (#0)
> POST /json HTTP/1.1
> Host: 127.0.0.1:3000
> User-Agent: curl/7.64.1
> Accept: */*
> Content-Length: 11
> Content-Type: application/x-www-form-urlencoded
>
* upload completely sent off: 11 out of 11 bytes
< HTTP/1.1 200 OK
< content-length: 0
< date: Fri, 29 May 2020 07:54:16 GMT
<
* Connection #0 to host 127.0.0.1 left intact
* Closing connection 0

---
# access log
{"path":"/","query":{},"addr":"127.0.0.1:3000","headers":{"accept":"*/*","user-agent":"curl/7.64.1","host":"127.0.0.1:3000"},"method":"GET","ts":"2020-05-29T16:52:11.600380+09:00"}
{"path":"/","query":{},"addr":"127.0.0.1:3000","headers":{"accept":"*/*","host":"127.0.0.1:3000","user-agent":"curl/7.64.1"},"method":"POST","ts":"2020-05-29T16:52:24.620505+09:00"}
{"path":"/xxx/yyy","query":{},"addr":"127.0.0.1:3000","body":"param=aaa","headers":{"user-agent":"curl/7.64.1","accept":"*/*","host":"127.0.0.1:3000","content-type":"application/x-www-form-urlencoded","content-length":"9"},"method":"POST","ts":"2020-05-29T16:52:52.644463+09:00"}
{"path":"/json","query":{},"addr":"127.0.0.1:3000","body":{"test":1},"headers":{"content-type":"application/x-www-form-urlencoded","content-length":"11","user-agent":"curl/7.64.1","host":"127.0.0.1:3000","accept":"*/*"},"method":"POST","ts":"2020-05-29T16:53:34.934432+09:00"}
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
