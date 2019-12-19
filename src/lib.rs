#![warn(
    missing_debug_implementations,
    missing_docs,
    rust_2018_idioms,
    unreachable_pub
)]

//! Async TLS streams

mod acceptor;
mod connector;
mod handshake;
mod std_adapter;
mod tls_stream;

pub use tls_stream::TlsStream;
pub use connector::TlsConnector;
pub use acceptor::TlsAcceptor;
