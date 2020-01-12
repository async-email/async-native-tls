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
//! # #[cfg(feature = "runtime-async-std")]
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
//! # #[cfg(feature = "runtime-tokio")]
//! # fn main() {}
//! ```

#[cfg(not(any(feature = "runtime-tokio", feature = "runtime-async-std")))]
compile_error!("one of 'runtime-async-std' or 'runtime-tokio' features must be enabled");

#[cfg(all(feature = "runtime-tokio", feature = "runtime-async-std"))]
compile_error!("only one of 'runtime-async-std' or 'runtime-tokio' features must be enabled");

mod acceptor;
mod connector;
mod handshake;
mod runtime;
mod std_adapter;
mod tls_stream;

pub use accept::accept;
pub use acceptor::{Error as AcceptError, TlsAcceptor};
pub use connect::{connect, TlsConnector};
pub use tls_stream::TlsStream;

#[doc(inline)]
pub use native_tls::{Certificate, Error, Identity, Protocol, Result};

mod accept {
    use crate::runtime::{AsyncRead, AsyncWrite};

    use crate::TlsStream;

    /// One of accept of an incoming connection.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # #[cfg(feature = "runtime-async-std")]
    /// # fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> { async_std::task::block_on(async {
    /// #
    /// use async_std::prelude::*;
    /// use async_std::net::TcpListener;
    /// use async_std::fs::File;
    ///
    /// let listener = TcpListener::bind("0.0.0.0:8443").await?;
    /// let (stream, _addr) = listener.accept().await?;
    ///
    /// let key = File::open("identity.pfx").await?;
    /// let stream = async_native_tls::accept(key, "<password>", stream).await?;
    /// // handle stream here
    /// #
    /// # Ok(()) }) }
    /// # #[cfg(feature = "runtime-tokio")]
    /// # fn main() {}
    /// ```
    pub async fn accept<R, S, T>(
        file: R,
        password: S,
        stream: T,
    ) -> Result<TlsStream<T>, crate::AcceptError>
    where
        R: AsyncRead + Unpin,
        S: AsRef<str>,
        T: AsyncRead + AsyncWrite + Unpin,
    {
        let acceptor = crate::TlsAcceptor::new(file, password).await?;
        let stream = acceptor.accept(stream).await?;

        Ok(stream)
    }
}

mod connect {
    use std::fmt::{self, Debug};

    use crate::runtime::{AsyncRead, AsyncWrite};
    use crate::TlsStream;
    use crate::{Certificate, Identity, Protocol};

    /// Connect a client to a remote server.
    ///
    /// # Examples
    ///
    /// ```
    /// # #[cfg(feature = "runtime-async-std")]
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
    /// # #[cfg(feature = "runtime-tokio")]
    /// # fn main() {}
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
    /// ```
    /// # #[cfg(feature = "runtime-async-std")]
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
    /// # #[cfg(feature = "runtime-tokio")]
    /// # fn main() {}
    /// ```
    pub struct TlsConnector {
        builder: native_tls::TlsConnectorBuilder,
    }

    impl Default for TlsConnector {
        fn default() -> Self {
            TlsConnector::new()
        }
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
        pub fn min_protocol_version(mut self, protocol: Option<Protocol>) -> Self {
            self.builder.min_protocol_version(protocol);
            self
        }

        /// Sets the maximum supported protocol version.
        ///
        /// A value of `None` enables support for the newest protocols supported by the
        /// implementation. Defaults to `None`.
        pub fn max_protocol_version(mut self, protocol: Option<Protocol>) -> Self {
            self.builder.max_protocol_version(protocol);
            self
        }

        /// Adds a certificate to the set of roots that the connector will trust.
        ///
        /// The connector will use the system's trust root by default. This method can be used to
        /// add to that set when communicating with servers not trusted by the system. Defaults to
        /// an empty set.
        pub fn add_root_certificate(mut self, cert: Certificate) -> Self {
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
        pub fn danger_accept_invalid_certs(mut self, accept_invalid_certs: bool) -> Self {
            self.builder
                .danger_accept_invalid_certs(accept_invalid_certs);
            self
        }

        /// Controls the use of Server Name Indication (SNI).
        ///
        /// Defaults to `true`.
        pub fn use_sni(mut self, use_sni: bool) -> Self {
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
        pub fn danger_accept_invalid_hostnames(mut self, accept_invalid_hostnames: bool) -> Self {
            self.builder
                .danger_accept_invalid_hostnames(accept_invalid_hostnames);
            self
        }

        /// Connect to a remote server.
        ///
        /// # Examples
        ///
        /// ```
        /// # #[cfg(feature = "runtime-async-std")]
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
        /// # #[cfg(feature = "runtime-tokio")]
        /// # fn main() {}
        /// ```
        pub async fn connect<S>(&self, domain: &str, stream: S) -> native_tls::Result<TlsStream<S>>
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

    impl From<native_tls::TlsConnectorBuilder> for TlsConnector {
        fn from(builder: native_tls::TlsConnectorBuilder) -> Self {
            Self { builder }
        }
    }
}
