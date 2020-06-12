use std::borrow::Cow;
use std::io;
use std::path::Path;

/// Let `Path` and `PathBuf` have `parse_dot` method.
pub trait ParseDot {
    /// Remove dots in the path and create a new `PathBuf` instance on demand.
    fn parse_dot(&self) -> io::Result<Cow<Path>>;
}
