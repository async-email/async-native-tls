#![warn(
    missing_debug_implementations,
    missing_docs,
    rust_2018_idioms,
    unreachable_pub
)]

//! Async TLS streams
//!
//! # Examples
//!
//! Create an HTTP client:
//!
//! ```no_run
//! # fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> { async_std::task::block_on(async {
//! #
//! use async_std::prelude::*;
//! use std::net::ToSocketAddrs;
//!
//! // First up, resolve google.com
//! let addr = "google.com:443".to_socket_addrs()?.next().unwrap();
//!
//! let socket = async_std::net::TcpStream::connect(&addr).await?;
//!
//! // Send off the request by first negotiating an SSL handshake, then writing
//! // of our request, then flushing, then finally read off the response.
//! let builder = native_tls::TlsConnector::builder();
//! let connector = builder.build()?;
//! let connector = async_native_tls::TlsConnector::from(connector);
//! let mut socket = connector.connect("google.com", socket).await?;
//! socket.write_all(b"GET / HTTP/1.0\r\n\r\n").await?;
//!
//! let mut data = Vec::new();
//! socket.read_to_end(&mut data).await?;
//!
//! // any response code is fine
//! assert!(data.starts_with(b"HTTP/1.0 "));
//!
//! let data = String::from_utf8_lossy(&data);
//! let data = data.trim_end();
//! assert!(data.ends_with("</html>") || data.ends_with("</HTML>"));
//! #
//! # Ok(()) }) }
//! ```

mod acceptor;
mod connector;
mod handshake;
mod std_adapter;
mod tls_stream;

pub use tls_stream::TlsStream;
pub use connector::TlsConnector;
pub use acceptor::TlsAcceptor;
