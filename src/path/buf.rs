use std::fmt::Debug;
use std::ops::Deref;

use super::NodePath;

/// An owned, mutable node path (akin to [`String`]).
///
/// This type provides methods like [`push`] that mutate the
/// path in place. It also implements [`Deref`] to [`NodePath`],
/// meaning that all methods on [`NodePath`] slices are available
/// on `NodePathBuf` values as well.
///
/// More details about the overall approach can be found in
/// the [module documentation](self).
#[derive(Default)]
pub struct NodePathBuf<'s> {
    paths: Vec<&'s str>,
}

impl<'s> NodePathBuf<'s> {
    /// Allocates an empty `NodePathBuf`.
    ///
    /// # Examples
    /// ```
    /// use configurate::path::NodePathBuf;
    ///
    /// let path = NodePathBuf::new();
    /// ```
    #[inline]
    pub fn new() -> Self {
        Self { paths: Vec::new() }
    }

    /// Creates anew `NodePathBuf` with a given slice.
    ///
    /// # Examples
    /// ```
    /// use configurate::path::NodePathBuf;
    ///
    /// let mut from_slice = NodePathBuf::from_slice(&["foo", "bar"]);
    /// assert_eq!(from_slice.as_slice(), &["foo", "bar"]);
    /// ```
    pub fn from_slice<S: AsRef<[&'s str]>>(s: S) -> Self {
        Self {
            paths: s.as_ref().to_vec(),
        }
    }

    /// Creates a new `NodePathBuf` with a given capacity used to create the
    /// internal [`Vec`].
    ///
    /// See [`with_capacity`] defined on [`Vec`].
    ///
    /// # Examples
    /// ```
    /// use configurate::path::{NodePath, NodePathBuf};
    ///
    /// let mut buf = NodePathBuf::with_capacity(10);
    /// let capacity = buf.capacity();
    ///
    /// // This push is done without reallocating
    /// buf.push(NodePath::from_slice(&["foo", "bar"]));
    ///
    /// assert_eq!(capacity, buf.capacity());
    /// ```
    /// [`with_capacity`]: Vec::with_capacity
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            paths: Vec::with_capacity(capacity),
        }
    }

    /// Extends `self` with `node_path`.
    ///
    /// # Examples
    /// ```
    /// use configurate::path::{NodePath, NodePathBuf};
    ///
    /// let mut buf = NodePathBuf::new();
    /// buf.push(NodePath::from_slice(&["foo", "bar"]));
    ///
    /// assert_eq!(buf.as_slice(), &["foo", "bar"]);
    /// ```
    pub fn push<P: AsRef<NodePath<'s>>>(&mut self, node_path: P) {
        node_path
            .as_ref()
            .paths
            .iter()
            .for_each(|p| self.paths.push(p));
    }

    /// Invokes [`capacity`] on the underlying instance of [`Vec`].
    ///
    /// [`capacity`]: Vec::capacity
    pub fn capacity(&self) -> usize {
        self.paths.capacity()
    }
}

impl<'s> Deref for NodePathBuf<'s> {
    type Target = NodePath<'s>;

    fn deref(&self) -> &Self::Target {
        NodePath::from_slice(&self.paths)
    }
}

impl<'s> AsRef<NodePath<'s>> for NodePathBuf<'s> {
    fn as_ref(&self) -> &NodePath<'s> {
        self
    }
}

impl<'s> Debug for NodePathBuf<'s> {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.paths.fmt(f)
    }
}
