use std::io;
use std::path::PathBuf;

/// Let `Path` and `PathBuf` have `parse_dot` method.
pub trait ParseDot {
    /// Remove dots in the path and create a new `PathBuf` instance.
    fn parse_dot(&self) -> io::Result<PathBuf>;
}
