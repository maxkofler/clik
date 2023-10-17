//! Errors available from the `clik` crate
use std::error::Error;

#[derive(Debug)]
/// Describes an error where there is no argument at an expected position
pub struct MissingArgumentError {
    /// The name of the argument
    pub name: String,
    /// The position of the argument
    pub position: usize,
    /// The type of the argument in string form
    pub ty: String,
}
impl std::fmt::Display for MissingArgumentError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Argument '{}' at position #{} of type '{}' not found",
            self.name, self.position, self.ty
        )
    }
}
impl std::error::Error for MissingArgumentError {}

#[derive(Debug)]
/// Describes an error where there is an argument, but it can't be parsed to the
/// desired type
pub struct WrongArgumentError {
    /// The name of the argument
    pub name: String,
    /// The position of the argument
    pub position: usize,
    /// The type of the argument in string form
    pub ty: String,
    /// The inner error describing what exactly went wrong
    pub inner: Box<dyn Error>,
}
impl std::fmt::Display for WrongArgumentError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Failed to parse argument '{}' at position #{} of type '{}': {}",
            self.name, self.position, self.ty, self.inner
        )
    }
}
impl std::error::Error for WrongArgumentError {}
