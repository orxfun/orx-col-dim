use crate::d_never::{DNever, IdxNever};
use crate::dim::Dim;

/// Marker type for one-dimensional collections.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct D1;

impl Dim for D1 {
    const D: usize = 1;

    type ChildDim = DNever;

    type Idx = usize;

    type ChildIdx = IdxNever;
}
