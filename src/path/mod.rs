mod buf;

pub use buf::*;

use std::fmt::Debug;

/// A slice of a node path (akin to [`str`]).
///
/// This type supports a number of operations for inspecting a node path, including
/// breaking the node path into its components (e.x. separated by indents on YAML),
/// and so on.
///
/// This is an *unsized* type, meaning that it must always used behind a
/// pointer like `&` or [`Box`]. For an owned version of this type,
/// see [`NodePathBuf`].
///
/// More details about the overall approach can be found in
/// the [module documentation](self).
#[derive(PartialEq, Eq)]
#[repr(transparent)]
pub struct NodePath<'s> {
    paths: [&'s str],
}

impl<'s> NodePath<'s> {
    /// Directly wraps a string slice as a `NodePath` slice.
    ///
    /// This is a cost-free conversion.
    ///
    /// # Examples
    /// ```
    /// use configurate::path::NodePath;
    ///
    /// let path = NodePath::from_slice(&["foo", "bar"]);
    /// assert_eq!(path.as_slice(), &["foo", "bar"]);
    /// ```
    ///
    /// You can create `NodePath`s from `String`s, or even other `NodePath`s:
    /// ```
    /// use configurate::path::NodePath;
    ///
    /// let string = String::from("foo");
    /// let from_string = NodePath::from_slice(&[&string]);
    /// let from_node_path = NodePath::from_slice(from_string.as_ref());
    /// assert_eq!(from_string, from_node_path);
    /// ```
    pub fn from_slice(s: &[&'s str]) -> &'s Self {
        // Safety: `NodePath` and [&str] has same memory layout
        unsafe { &*(s as *const [&'s str] as *const Self) }
    }

    /// Converts a `NodePath` to an owned [`NodePathBuf`].
    ///
    /// # Examples
    /// ```
    /// use configurate::path::{NodePath, NodePathBuf};
    ///
    /// let path = NodePath::from_slice(&["foo", "bar"]);
    /// let buf = NodePathBuf::from_slice(&["foo", "bar"]);
    ///
    /// assert_eq!(path.as_slice(), buf.as_slice());
    /// ```
    pub fn to_node_path_buf(&self) -> NodePathBuf {
        NodePathBuf::from_slice(&self.paths)
    }

    /// Yields the underlying [`str`] slices.
    ///
    /// # Examples
    /// ```
    /// use configurate::path::NodePath;
    ///
    /// let path = NodePath::from_slice(&["foo", "bar"]);
    /// assert_eq!(path.as_slice(), &["foo", "bar"]);
    /// ```
    #[inline]
    pub fn as_slice(&'s self) -> &'s [&'s str] {
        &self.paths
    }
}

impl<'s> AsRef<[&'s str]> for &NodePath<'s> {
    #[inline]
    fn as_ref(&self) -> &[&'s str] {
        &self.paths
    }
}

impl <'s> AsRef<NodePath<'s>> for &NodePath<'s> {
    fn as_ref(&self) -> &NodePath<'s> {
        self
    }
}

impl<'s> From<&[&'s str]> for &NodePath<'s> {
    fn from(s: &[&'s str]) -> Self {
        NodePath::from_slice(s)
    }
}

impl Debug for NodePath<'_> {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.paths.fmt(f)
    }
}
