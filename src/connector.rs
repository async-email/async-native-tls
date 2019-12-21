use std::fmt;
use std::marker::Unpin;

use async_std::io::{Read as AsyncRead, Write as AsyncWrite};
use native_tls::Error;

use crate::handshake::handshake;
use crate::TlsStream;

/// A wrapper around a `native_tls::TlsConnector`, providing an async `connect`
/// method.
#[derive(Clone)]
pub(crate) struct TlsConnector(native_tls::TlsConnector);

impl TlsConnector {
    /// Connects the provided stream with this connector, assuming the provided domain.
    pub(crate) async fn connect<S>(&self, domain: &str, stream: S) -> Result<TlsStream<S>, Error>
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
