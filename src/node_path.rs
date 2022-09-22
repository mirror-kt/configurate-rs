use std::ops::{Add, Index};
use thiserror::Error;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ConfigurationValueType {
    List,
    Map,
    Null,
    Scalar,
}

/// Create a node path reference.
pub fn of(path: impl IntoIterator<Item = ConfigurationValueType>) -> impl NodePath {
    internal::NodePathImpl::of(path)
}

/// Get an empty node path. This refers to the root node.
pub fn empty() -> impl NodePath {
    internal::NodePathImpl::empty()
}

/// Represents the path to a given node.
pub trait NodePath:
    IntoIterator<Item = ConfigurationValueType>
    + Index<usize, Output = ConfigurationValueType>
    + Clone
    + Sized
{
    type Iter<'a>: Iterator
    where
        Self: 'a;

    /// Gets a specific element from the path vec..
    fn get(&self, index: usize) -> Option<&ConfigurationValueType>;

    /// Gets the length of the path.
    fn size(&self) -> usize;

    /// Returns a copy of the original path vec.
    fn to_vec(&self) -> Vec<ConfigurationValueType>;

    /// Creates a new path with the provided element appended to the end.
    fn with_appended_child(&self, child_key: ConfigurationValueType) -> Self;

    /// Creates a new path with the value at index.
    fn with(&self, index: usize, value: ConfigurationValueType) -> Result<Self, NodePathError>;

    fn iter(&self) -> Self::Iter<'_>;
}

/// Errors related to NodePath operations.
#[derive(Debug, Error)]
pub enum NodePathError {
    #[error("Index must be in 0 < index < {size}, but {value}")]
    IndexOutOfBounds { value: usize, size: usize },
}

pub(crate) mod internal {
    use std::fmt::Debug;

    use super::*;

    #[derive(Clone, PartialEq, Eq, Hash)]
    pub(crate) struct NodePathImpl {
        vec: Vec<ConfigurationValueType>,
    }

    impl NodePathImpl {
        pub(super) fn empty() -> Self {
            Self { vec: vec![] }
        }

        pub fn of(path: impl IntoIterator<Item = super::ConfigurationValueType>) -> Self {
            Self {
                vec: path.into_iter().collect(),
            }
        }
    }

    impl NodePath for NodePathImpl {
        type Iter<'a> = std::slice::Iter<'a, ConfigurationValueType>;

        fn get(&self, index: usize) -> Option<&ConfigurationValueType> {
            self.vec.get(index)
        }

        fn size(&self) -> usize {
            self.vec.len()
        }

        fn to_vec(&self) -> Vec<ConfigurationValueType> {
            self.vec.clone()
        }

        fn with_appended_child(&self, child_key: ConfigurationValueType) -> Self {
            if self.vec.is_empty() {
                return NodePathImpl {
                    vec: vec![child_key],
                };
            }

            let mut child_path = self.vec.clone();
            child_path.push(child_key);
            NodePathImpl { vec: child_path }
        }

        fn with(&self, index: usize, value: ConfigurationValueType) -> Result<Self, NodePathError> {
            if index > self.vec.len() {
                return Err(NodePathError::IndexOutOfBounds {
                    value: index,
                    size: self.vec.len(),
                });
            }

            let mut new_path = self.vec.clone();
            new_path[index] = value;
            Ok(NodePathImpl { vec: new_path })
        }

        fn iter(&self) -> Self::Iter<'_> {
            self.vec.iter()
        }
    }

    impl Index<usize> for NodePathImpl {
        type Output = ConfigurationValueType;

        fn index(&self, index: usize) -> &Self::Output {
            &self.vec[index]
        }
    }

    impl<Rhs: NodePath> Add<Rhs> for NodePathImpl {
        type Output = Self;

        fn add(self, rhs: Rhs) -> Self::Output {
            if self.vec.is_empty() {
                return Self::Output { vec: rhs.to_vec() };
            }
            if rhs.size() == 0 {
                return self;
            }

            let other_vec = rhs.to_vec();
            let result = self.vec.iter().chain(other_vec.iter()).cloned().collect();

            Self { vec: result }
        }
    }

    impl IntoIterator for NodePathImpl {
        type Item = ConfigurationValueType;
        type IntoIter = std::vec::IntoIter<ConfigurationValueType>;

        fn into_iter(self) -> Self::IntoIter {
            self.vec.into_iter()
        }
    }

    impl Debug for NodePathImpl {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            Debug::fmt(&self.vec, f)
        }
    }
}
