//! Low-level types used throughout the Substrate stencil code.

#![warn(missing_docs)]
#![cfg_attr(not(feature = "std"), no_std)]

/// Type used for expressing timestamp.
pub type Moment = u64;

/// A hash of some data used by the chain.
pub type Hash = sp_core::H256;

/// An index to a block.
pub type BlockNumber = u32;
