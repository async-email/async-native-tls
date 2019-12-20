<h1 align="center">async-native-tls</h1>
<div align="center">
 <strong>
   Native TLS for async
 </strong>
</div>

<br />

<div align="center">
  <!-- Crates version -->
  <a href="https://crates.io/crates/async-native-tls">
    <img src="https://img.shields.io/crates/v/async-native-tls.svg?style=flat-square"
    alt="Crates.io version" />
  </a>
  <!-- Downloads -->
  <a href="https://crates.io/crates/async-native-tls">
    <img src="https://img.shields.io/crates/d/async-native-tls.svg?style=flat-square"
      alt="Download" />
  </a>
  <!-- docs.rs docs -->
  <a href="https://docs.rs/async-native-tls">
    <img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square"
      alt="docs.rs docs" />
  </a>
</div>

<div align="center">
  <h3>
    <a href="https://docs.rs/async-native-tls">
      API Docs
    </a>
    <span> | </span>
    <a href="https://github.com/async-email/async-native-tls/releases">
      Releases
    </a>
  </h3>
</div>

<br/>

> Native TLS for [futures](https://crates.io/crates/futures) and [async-std](https://crates.io/crates/async-std). Based on tokio-tls.

## Installation

```sh
$ cargo add async-native-tls
```

## Example

```rust
use async_std::prelude::*;
use async_std::net::TcpStream;

let stream = TcpStream::connect("google.com:443").await?;
let mut stream = async_native_tls::connect("google.com", stream).await?;
stream.write_all(b"GET / HTTP/1.0\r\n\r\n").await?;

let mut res = Vec::new();
stream.read_to_end(&mut res).await?;
println!("{}", String::from_utf8_lossy(&res));
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
