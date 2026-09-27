use crate::dim::Dim;

/// Uninhabited index type used by the zero-dimensional sentinel.
pub enum IdxNever {}

/// Sentinel dimension below the one-dimensional base case.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DNever {}

impl Dim for DNever {
    const D: usize = 0;

    type ChildDim = Self;

    type Idx = IdxNever;

    type ChildIdx = IdxNever;
}
