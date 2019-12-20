use std::fmt;
use std::marker::Unpin;

use futures_io::{AsyncRead, AsyncWrite};
use native_tls::Error;

use crate::handshake::handshake;
use crate::TlsStream;

/// A wrapper around a `native_tls::TlsConnector`, providing an async `connect`
/// method.
#[derive(Clone)]
pub struct TlsConnector(native_tls::TlsConnector);

impl TlsConnector {
    /// Connects the provided stream with this connector, assuming the provided
    /// domain.
    ///
    /// This function will internally call `TlsConnector::connect` to connect
    /// the stream and returns a future representing the resolution of the
    /// connection operation. The returned future will resolve to either
    /// `TlsStream<S>` or `Error` depending if it's successful or not.
    ///
    /// This is typically used for clients who have already established, for
    /// example, a TCP connection to a remote server. That stream is then
    /// provided here to perform the client half of a connection to a
    /// TLS-powered server.
    ///
    /// # Example
    ///
    /// ```rust
    /// # fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> { async_std::task::block_on(async {
    /// #
    /// use async_std::prelude::*;
    /// use std::net::ToSocketAddrs;
    ///
    /// let socket = async_std::net::TcpStream::connect("google.com:443").await?;
    ///
    /// // Configure using the regular native_tls::TlsConnector.
    /// let builder = native_tls::TlsConnector::builder();
    ///
    /// let connector: async_native_tls::TlsConnector = builder.build()?.into();
    ///
    /// let mut socket = connector.connect("google.com", socket).await?;
    /// socket.write_all(b"GET / HTTP/1.0\r\n\r\n").await?;
    ///
    /// let mut data = Vec::new();
    /// socket.read_to_end(&mut data).await?;
    ///
    /// // any response code is fine
    /// assert!(data.starts_with(b"HTTP/1.0 "));
    ///
    /// let data = String::from_utf8_lossy(&data);
    /// let data = data.trim_end();
    /// assert!(data.ends_with("</html>") || data.ends_with("</HTML>"));
    /// #
    /// # Ok(()) }) }
    pub async fn connect<S>(&self, domain: &str, stream: S) -> Result<TlsStream<S>, Error>
    where
        S: AsyncRead + AsyncWrite + Unpin,
    {
        handshake(move |s| self.0.connect(domain, s), stream).await
    }
}

impl fmt::Debug for TlsConnector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TlsConnector").finish()
    }
}

impl From<native_tls::TlsConnector> for TlsConnector {
    fn from(inner: native_tls::TlsConnector) -> TlsConnector {
        TlsConnector(inner)
    }
}
