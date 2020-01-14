#[cfg(feature = "runtime-async-std")]
pub(crate) use async_std::io::{Read as AsyncRead, Write as AsyncWrite};

#[cfg(feature = "runtime-async-std")]
#[allow(unused_imports)]
pub(crate) use async_std::io::prelude::{ReadExt as AsyncReadExt, WriteExt as AsyncWriteExt};

#[cfg(feature = "runtime-futures")]
pub(crate) use futures_util::io::{AsyncRead, AsyncWrite};

#[cfg(feature = "runtime-futures")]
#[allow(unused_imports)]
pub(crate) use futures_util::io::{AsyncReadExt, AsyncWriteExt};

#[cfg(feature = "runtime-tokio")]
pub(crate) use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite};
