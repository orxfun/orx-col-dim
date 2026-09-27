use core::fmt::Debug;

/// Describes the dimensionality and index types of a collection.
pub trait Dim: Clone + Copy + Debug + PartialEq + Eq + 'static {
    /// Number of dimensions represented by this type.
    const D: usize;

    /// Dimension of a child collection.
    type ChildDim: Dim;

    /// Index type for this dimension.
    type Idx;

    /// Index type for a child collection.
    type ChildIdx;
}
