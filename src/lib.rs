#![deny(missing_docs)]
#![forbid(unsafe_code)]
//! Offers two structs:
//! * [`RangedReader`], that implements [`std::io::Read`] and [`std::io::Seek`] from a function
//!   returning a [`Vec<u8>`] from a ranged request `(start, length)`.
//! * [`RangedAsyncReader`] that implements [`futures::io::AsyncRead`] and [`futures::io::AsyncRead`]
//!   from a asynchronous function returning (a future of) [`Vec<u8>`] from a ranged request `(start, length)`.
//!
//! A common use use-case for this crate is to perform ranged queries from remote blob storages
//! such as AWS s3, Azure blob, and Google's cloud storage.

#[cfg(feature = "sync")]
#[cfg_attr(docsrs, doc(cfg(feature = "sync")))]
mod sync;
#[cfg(feature = "sync")]
#[cfg_attr(docsrs, doc(cfg(feature = "sync")))]
pub use sync::*;
#[cfg(feature = "async")]
#[cfg_attr(docsrs, doc(cfg(feature = "async")))]
mod stream;
#[cfg(feature = "async")]
#[cfg_attr(docsrs, doc(cfg(feature = "async")))]
pub use stream::*;
