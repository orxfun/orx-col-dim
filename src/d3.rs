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

// descendent

/// Index into a descendant dimension of a three-dimensional collection.
pub enum DescendentIdxD3 {
    /// Index for a zero-dimensional descendant.
    Child0([usize; 0]),
    /// Index for a one-dimensional descendant.
    Child1([usize; 1]),
    /// Index for a two-dimensional descendant.
    Child2([usize; 2]),
}

impl From<[usize; 0]> for DescendentIdxD3 {
    fn from(value: [usize; 0]) -> Self {
        Self::Child0(value)
    }
}

impl From<usize> for DescendentIdxD3 {
    fn from(value: usize) -> Self {
        Self::Child1([value])
    }
}

impl From<[usize; 1]> for DescendentIdxD3 {
    fn from(value: [usize; 1]) -> Self {
        Self::Child1(value)
    }
}

impl From<[usize; 2]> for DescendentIdxD3 {
    fn from(value: [usize; 2]) -> Self {
        Self::Child2(value)
    }
}
