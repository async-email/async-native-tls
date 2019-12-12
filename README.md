# async-native-tls

> TLS implementation for asnync. Based on tokio-tls.

## Installation

```sh
$ cargo add async-native-tls
```

## Example

```rust
// First up, resolve google.com
let addr = "google.com:443".to_socket_addrs().unwrap().next().unwrap();

let socket = TcpStream::connect(&addr).await.unwrap();

// Send off the request by first negotiating an SSL handshake, then writing
// of our request, then flushing, then finally read off the response.
let builder = TlsConnector::builder();
let connector = builder.build().unwrap();
let connector = async_native_tls::TlsConnector::from(connector);
let mut socket = connector.connect("google.com", socket).await;
socket.write_all(b"GET / HTTP/1.0\r\n\r\n").await.unwrap();

let mut data = Vec::new();
socket.read_to_end(&mut data).await.unwrap();

// any response code is fine
assert!(data.starts_with(b"HTTP/1.0 "));

let data = String::from_utf8_lossy(&data);
let data = data.trim_end();
assert!(data.ends_with("</html>") || data.ends_with("</HTML>"));
```

## Contributing
Want to join us? Check out our ["Contributing" guide][contributing] and take a
look at some of these issues:

- [Issues labeled "good first issue"][good-first-issue]
- [Issues labeled "help wanted"][help-wanted]

[contributing]: https://github.com/dignifiedquire/semver2/blob/master.github/CONTRIBUTING.md
[good-first-issue]: https://github.com/dignifiedquire/semver2/labels/good%20first%20issue
[help-wanted]: https://github.com/dignifiedquire/semver2/labels/help%20wanted

## License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br/>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>
