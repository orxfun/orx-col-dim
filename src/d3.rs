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

    type DescendentIdx = DescendentIdxD3;
}

pub enum DescendentIdxD3 {
    Child0([usize; 0]),
    Child1([usize; 1]),
    Child2([usize; 2]),
}

impl From<usize> for DescendentIdxD3 {
    fn from(value: usize) -> Self {
        Self::Child1([value])
    }
}
