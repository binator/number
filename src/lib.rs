#![doc = include_str!("../readme.md")]
#![cfg_attr(not(test), no_std)]
#![feature(trait_alias)]
#![warn(missing_docs)]
#![deny(clippy::default_numeric_fallback)]

mod float;
use binator_base::{
  digit,
  is,
  BaseAtom,
};
use binator_core::{
  Contexting,
  CoreAtom,
  Parse,
  Parsed,
  Streaming,
};
use binator_utils::{
  Utils,
  UtilsAtom,
};
pub use float::*;

#[cfg(feature = "radix")]
mod radix;
#[cfg(feature = "radix")]
pub use radix::*;

mod primitive;
pub use primitive::*;

/// Parse character digit and return it in integer format
#[cfg_attr(
  feature = "tracing",
  tracing::instrument(level = "trace", skip_all, ret(Display))
)]
pub fn to_digit<Stream, Context>(stream: Stream) -> Parsed<u8, Stream, Context>
where
  Stream: Streaming,
  Stream::Item: Into<u8>,
  Context: Contexting<CoreAtom<Stream>>,
  Context: Contexting<BaseAtom<u8>>,
  Context: Contexting<UtilsAtom<Stream>>,
{
  digit.map(|d| u8::from(d) - b'0').parse(stream)
}

/// Enum that hold Sign value
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Sign {
  /// When sign is positive
  Pos,
  /// When sign is negative
  Neg,
}

/// Sign   ::= [+-]
#[cfg_attr(
  feature = "tracing",
  tracing::instrument(level = "trace", skip_all, ret(Display))
)]
pub fn sign<Stream, Context>(stream: Stream) -> Parsed<Sign, Stream, Context>
where
  Stream: Streaming,
  <Stream as Streaming>::Item: Into<u8>,
  Context: Contexting<UtilsAtom<Stream>>,
  Context: Contexting<BaseAtom<u8>>,
  Context: Contexting<CoreAtom<Stream>>,
{
  is(b'-')
    .map(|_| Sign::Neg)
    .or(is(b'+').map(|_| Sign::Pos))
    .parse(stream)
}
