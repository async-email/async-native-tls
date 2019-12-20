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
pub use connect::{connect, TlsConnector};
pub use tls_stream::TlsStream;

#[doc(inline)]
use native_tls::{Certificate, Identity, Protocol};

mod connect {
    use futures_io::{AsyncRead, AsyncWrite};
    use std::fmt::{self, Debug};

    use crate::TlsStream;
    use crate::{Certificate, Identity, Protocol};

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
        let stream = TlsConnector::new().connect(domain, stream).await?;
        Ok(stream)
    }

    /// Connect a client to a remote server.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> { async_std::task::block_on(async {
    /// #
    /// use async_std::prelude::*;
    /// use async_std::net::TcpStream;
    /// use async_native_tls::TlsConnector;
    ///
    /// let stream = TcpStream::connect("google.com:443").await?;
    /// let mut stream = TlsConnector::new()
    ///     .use_sni(true)
    ///     .connect("google.com", stream)
    ///     .await?;
    /// stream.write_all(b"GET / HTTP/1.0\r\n\r\n").await?;
    ///
    /// let mut res = Vec::new();
    /// stream.read_to_end(&mut res).await?;
    /// println!("{}", String::from_utf8_lossy(&res));
    /// #
    /// # Ok(()) }) }
    /// ```
    pub struct TlsConnector {
        builder: native_tls::TlsConnectorBuilder,
    }

    impl TlsConnector {
        /// Create a new instance.
        pub fn new() -> Self {
            Self {
                builder: native_tls::TlsConnector::builder(),
            }
        }

        /// Sets the identity to be used for client certificate authentication.
        pub fn identity(mut self, identity: Identity) -> Self {
            self.builder.identity(identity);
            self
        }

        /// Sets the minimum supported protocol version.
        ///
        /// A value of `None` enables support for the oldest protocols supported by the
        /// implementation. Defaults to `Some(Protocol::Tlsv10)`.
        pub fn min_protocol_version(&mut self, protocol: Option<Protocol>) -> &mut Self {
            self.builder.min_protocol_version(protocol);
            self
        }

        /// Sets the maximum supported protocol version.
        ///
        /// A value of `None` enables support for the newest protocols supported by the
        /// implementation. Defaults to `None`.
        pub fn max_protocol_version(&mut self, protocol: Option<Protocol>) -> &mut Self {
            self.builder.max_protocol_version(protocol);
            self
        }

        /// Adds a certificate to the set of roots that the connector will trust.
        ///
        /// The connector will use the system's trust root by default. This method can be used to
        /// add to that set when communicating with servers not trusted by the system. Defaults to
        /// an empty set.
        pub fn add_root_certificate(&mut self, cert: Certificate) -> &mut Self {
            self.builder.add_root_certificate(cert);
            self
        }

        /// Controls the use of certificate validation.
        ///
        /// Defaults to false.
        ///
        /// # Warning
        ///
        /// You should think very carefully before using this method. If invalid certificates are
        /// trusted, any certificate for any site will be trusted for use. This includes expired
        /// certificates. This introduces significant vulnerabilities, and should only be used as a
        /// last resort.
        pub fn danger_accept_invalid_certs(&mut self, accept_invalid_certs: bool) -> &mut Self {
            self.builder
                .danger_accept_invalid_certs(accept_invalid_certs);
            self
        }

        /// Controls the use of Server Name Indication (SNI).
        ///
        /// Defaults to `true`.
        pub fn use_sni(&mut self, use_sni: bool) -> &mut Self {
            self.builder.use_sni(use_sni);
            self
        }

        /// Controls the use of hostname verification.
        ///
        /// Defaults to `false`.
        ///
        /// # Warning
        ///
        /// You should think very carefully before using this method. If invalid hostnames are
        /// trusted, any valid certificate for any site will be trusted for use. This introduces
        /// significant vulnerabilities, and should only be used as a last resort.
        pub fn danger_accept_invalid_hostnames(
            &mut self,
            accept_invalid_hostnames: bool,
        ) -> &mut Self {
            self.builder
                .danger_accept_invalid_hostnames(accept_invalid_hostnames);
            self
        }

        /// Connect to a remote server.
        ///
        /// # Examples
        ///
        /// ```no_run
        /// # fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> { async_std::task::block_on(async {
        /// #
        /// use async_std::prelude::*;
        /// use async_std::net::TcpStream;
        /// use async_native_tls::TlsConnector;
        ///
        /// let stream = TcpStream::connect("google.com:443").await?;
        /// let mut stream = TlsConnector::new()
        ///     .use_sni(true)
        ///     .connect("google.com", stream)
        ///     .await?;
        /// stream.write_all(b"GET / HTTP/1.0\r\n\r\n").await?;
        ///
        /// let mut res = Vec::new();
        /// stream.read_to_end(&mut res).await?;
        /// println!("{}", String::from_utf8_lossy(&res));
        /// #
        /// # Ok(()) }) }
        /// ```
        pub async fn connect<S>(self, domain: &str, stream: S) -> native_tls::Result<TlsStream<S>>
        where
            S: AsyncRead + AsyncWrite + Unpin,
        {
            let connector = self.builder.build()?;
            let connector = crate::connector::TlsConnector::from(connector);
            let stream = connector.connect(domain, stream).await?;
            Ok(stream)
        }
    }

    impl Debug for TlsConnector {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("TlsConnector").finish()
        }
    }
}
