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

    type DescendentIdx = DescendentIdxD2;
}

// descendent

/// Index into a descendant dimension of a two-dimensional collection.
pub enum DescendentIdxD2 {
    /// Index for a zero-dimensional descendant.
    Child0([usize; 0]),
    /// Index for a one-dimensional descendant.
    Child1([usize; 1]),
}

impl From<[usize; 0]> for DescendentIdxD2 {
    fn from(value: [usize; 0]) -> Self {
        Self::Child0(value)
    }
}

impl From<usize> for DescendentIdxD2 {
    fn from(value: usize) -> Self {
        Self::Child1([value])
    }
}

impl From<[usize; 1]> for DescendentIdxD2 {
    fn from(value: [usize; 1]) -> Self {
        Self::Child1(value)
    }
}
