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

pub enum DescendentIdxD2 {
    Child0([usize; 0]),
    Child1([usize; 1]),
}

impl From<usize> for DescendentIdxD2 {
    fn from(value: usize) -> Self {
        Self::Child1([value])
    }
}
