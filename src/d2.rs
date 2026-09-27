use crate::d1::D1;
use crate::dim::Dim;

/// Marker type for two-dimensional collections.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct D2;

impl Dim for D2 {
    const D: usize = 2;

    type ChildDim = D1;

    type Idx = [usize; Self::D];

    type ChildIdx = usize;
}
