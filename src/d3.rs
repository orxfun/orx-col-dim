use crate::d2::D2;
use crate::dim::Dim;

/// Marker type for three-dimensional collections.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct D3;

impl Dim for D3 {
    const D: usize = 3;

    type ChildDim = D2;

    type Idx = [usize; Self::D];

    type ChildIdx = usize;
}
