use thiserror::Error;

/// Errors that can occur during constraint tree operations.
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum ConstraintError {
    #[error("Cannot add child to a leaf node")]
    CannotAddChildToLeaf,

    #[error("Cannot add child to a NOT node")]
    CannotAddChildToNot,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cannot_add_child_to_leaf_display() {
        let e = ConstraintError::CannotAddChildToLeaf;
        assert_eq!(e.to_string(), "Cannot add child to a leaf node");
    }

    #[test]
    fn cannot_add_child_to_not_display() {
        let e = ConstraintError::CannotAddChildToNot;
        assert_eq!(e.to_string(), "Cannot add child to a NOT node");
    }

    #[test]
    fn error_equality() {
        assert_eq!(
            ConstraintError::CannotAddChildToLeaf,
            ConstraintError::CannotAddChildToLeaf
        );
        assert_ne!(
            ConstraintError::CannotAddChildToLeaf,
            ConstraintError::CannotAddChildToNot
        );
    }
}
