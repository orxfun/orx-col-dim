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

    type DescendentIdx = DescendentIdxD4;
}

// descendent

/// Index into a descendant dimension of a four-dimensional collection.
pub enum DescendentIdxD4 {
    /// Index for a zero-dimensional descendant.
    Child0([usize; 0]),
    /// Index for a one-dimensional descendant.
    Child1([usize; 1]),
    /// Index for a two-dimensional descendant.
    Child2([usize; 2]),
    /// Index for a three-dimensional descendant.
    Child3([usize; 3]),
}

impl From<[usize; 0]> for DescendentIdxD4 {
    fn from(value: [usize; 0]) -> Self {
        Self::Child0(value)
    }
}

impl From<usize> for DescendentIdxD4 {
    fn from(value: usize) -> Self {
        Self::Child1([value])
    }
}

impl From<[usize; 1]> for DescendentIdxD4 {
    fn from(value: [usize; 1]) -> Self {
        Self::Child1(value)
    }
}

impl From<[usize; 2]> for DescendentIdxD4 {
    fn from(value: [usize; 2]) -> Self {
        Self::Child2(value)
    }
}

impl From<[usize; 3]> for DescendentIdxD4 {
    fn from(value: [usize; 3]) -> Self {
        Self::Child3(value)
    }
}
