#![doc = include_str!("../README.md")]
#![warn(
    missing_docs,
    clippy::unwrap_in_result,
    clippy::unwrap_used,
    clippy::panic,
    clippy::panic_in_result_fn,
    clippy::float_cmp,
    clippy::float_cmp_const,
    clippy::missing_panics_doc,
    clippy::todo
)]
#![no_std]

extern crate alloc;

#[cfg(test)]
extern crate std;

mod d1;
mod d2;
mod d3;
mod d4;
mod d_never;
mod dim;

pub use d_never::{DNever, IdxNever};
pub use d1::D1;
pub use d2::D2;
pub use d3::D3;
pub use d4::D4;
pub use dim::Dim;
