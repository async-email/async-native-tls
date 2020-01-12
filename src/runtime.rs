#[cfg(feature = "runtime-async-std")]
pub(crate) use async_std::io::{Read as AsyncRead, Write as AsyncWrite};

#[cfg(feature = "runtime-async-std")]
#[allow(unused_imports)]
pub(crate) use async_std::io::prelude::{ReadExt as AsyncReadExt, WriteExt as AsyncWriteExt};

#[cfg(feature = "runtime-tokio")]
pub(crate) use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite};
