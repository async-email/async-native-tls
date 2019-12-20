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
//! To connect as a client to a remote server:
//!
//! ```rust
//! # fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> { async_std::task::block_on(async {
//! #
//! use async_std::prelude::*;
//! use async_std::net::TcpStream;
//!
//! let stream = TcpStream::connect("google.com:443").await?;
//! let mut stream = async_native_tls::connect("google.com", stream).await?;
//! stream.write_all(b"GET / HTTP/1.0\r\n\r\n").await?;
//!
//! let mut res = Vec::new();
//! stream.read_to_end(&mut res).await?;
//! println!("{}", String::from_utf8_lossy(&res));
//! #
//! # Ok(()) }) }
//! ```

mod acceptor;
mod connector;
mod handshake;
mod std_adapter;
mod tls_stream;

pub use acceptor::TlsAcceptor;
pub use connect::connect;
pub use connector::TlsConnector;
pub use tls_stream::TlsStream;

mod connect {
    use crate::TlsConnector;
    use crate::TlsStream;
    use futures_io::{AsyncRead, AsyncWrite};

    /// Connect a client to a remote server.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> { async_std::task::block_on(async {
    /// #
    /// use async_std::prelude::*;
    /// use async_std::net::TcpStream;
    ///
    /// let stream = TcpStream::connect("google.com:443").await?;
    /// let mut stream = async_native_tls::connect("google.com", stream).await?;
    /// stream.write_all(b"GET / HTTP/1.0\r\n\r\n").await?;
    ///
    /// let mut res = Vec::new();
    /// stream.read_to_end(&mut res).await?;
    /// println!("{}", String::from_utf8_lossy(&res));
    /// #
    /// # Ok(()) }) }
    /// ```
    pub async fn connect<S>(domain: &str, stream: S) -> native_tls::Result<TlsStream<S>>
    where
        S: AsyncRead + AsyncWrite + Unpin,
    {
        let builder = native_tls::TlsConnector::builder();
        let connector = builder.build()?;
        let connector = TlsConnector::from(connector);
        let stream = connector.connect(domain, stream).await?;
        Ok(stream)
    }
}
