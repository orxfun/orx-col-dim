use crate::d3::D3;
use crate::dim::Dim;

/// Marker type for four-dimensional collections.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct D4;

impl Dim for D4 {
    const D: usize = 4;

    type ChildDim = D3;

    type Idx = [usize; Self::D];

    type ChildIdx = usize;
}

pub enum ChildIdxD4 {
    Child0([usize; 0]),
    Child1([usize; 1]),
    Child2([usize; 2]),
    Child3([usize; 3]),
}
