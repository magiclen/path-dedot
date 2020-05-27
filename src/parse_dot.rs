use std::io;
use std::path::PathBuf;

/// Let `Path` and `PathBuf` have `parse_dot` method.
pub trait ParseDot {
    /// Remove dots in the path and create a new `PathBuf` instance.
    ///
    /// Please read the following examples to know the parsing rules.
    ///
    /// # Examples
    ///
    /// If a path starts with a single dot, the dot means **current working directory**.
    ///
    /// ```
    /// extern crate path_dedot;
    ///
    /// use std::env;
    /// use std::path::Path;
    ///
    /// use path_dedot::*;
    ///
    /// if cfg!(not(windows)) {
    ///     let p = Path::new("./path/to/123/456");
    ///
    ///     assert_eq!(
    ///         Path::join(env::current_dir().unwrap().as_path(), Path::new("path/to/123/456"))
    ///             .to_str()
    ///             .unwrap(),
    ///         p.parse_dot().unwrap().to_str().unwrap()
    ///     );
    /// }
    /// ```
    ///
    /// If a path starts with a pair of dots, the dots means the parent of **current working directory**. If **current working directory** is **root**, the parent is still **root**.
    ///
    /// ```
    /// extern crate path_dedot;
    ///
    /// use std::env;
    /// use std::path::Path;
    ///
    /// use path_dedot::*;
    ///
    /// if cfg!(not(windows)) {
    ///     let p = Path::new("../path/to/123/456");
    ///
    ///     let cwd = env::current_dir().unwrap();
    ///
    ///     let cwd_parent = cwd.parent();
    ///
    ///     match cwd_parent {
    ///         Some(cwd_parent) => {
    ///             assert_eq!(
    ///                 Path::join(&cwd_parent, Path::new("path/to/123/456")).to_str().unwrap(),
    ///                 p.parse_dot().unwrap().to_str().unwrap()
    ///             );
    ///         }
    ///         None => {
    ///             assert_eq!(
    ///                 Path::join(Path::new("/"), Path::new("path/to/123/456")).to_str().unwrap(),
    ///                 p.parse_dot().unwrap().to_str().unwrap()
    ///             );
    ///         }
    ///     }
    /// }
    /// ```
    ///
    /// Besides starting with, the **Single Dot** and **Double Dots** can also be placed to other positions. **Single Dot** means noting and will be ignored. **Double Dots** means the parent.
    ///
    /// ```
    /// extern crate path_dedot;
    ///
    /// use std::path::Path;
    ///
    /// use path_dedot::*;
    ///
    /// if cfg!(not(windows)) {
    ///     let p = Path::new("/path/to/../123/456/./777");
    ///
    ///     assert_eq!("/path/123/456/777", p.parse_dot().unwrap().to_str().unwrap());
    /// }
    /// ```
    ///
    /// ```
    /// extern crate path_dedot;
    ///
    /// use std::path::Path;
    ///
    /// use path_dedot::*;
    ///
    /// if cfg!(not(windows)) {
    ///     let p = Path::new("/path/to/../123/456/./777/..");
    ///
    ///     assert_eq!("/path/123/456", p.parse_dot().unwrap().to_str().unwrap());
    /// }
    /// ```
    ///
    /// You should notice that `parse_dot` method does **not** aim to get an **absolute path**. A path which does not start with a `MAIN_SEPARATOR`, **Single Dot** and **Double Dots**, will not have each of them after the `parse_dot` method is used.
    ///
    /// ```
    /// extern crate path_dedot;
    ///
    /// use std::path::Path;
    ///
    /// use path_dedot::*;
    ///
    /// if cfg!(not(windows)) {
    ///     let p = Path::new("path/to/../123/456/./777/..");
    ///
    ///     assert_eq!("path/123/456", p.parse_dot().unwrap().to_str().unwrap());
    /// }
    /// ```
    ///
    /// **Double Dots** which is not placed at the start cannot get the parent beyond the original path. Why not? With this constraint, you can insert an absolute path to the start as a virtual root in order to protect your file system from being exposed.
    ///
    /// ```
    /// extern crate path_dedot;
    ///
    /// use std::path::Path;
    ///
    /// use path_dedot::*;
    ///
    /// if cfg!(not(windows)) {
    ///     let p = Path::new("path/to/../../../../123/456/./777/..");
    ///
    ///     assert_eq!("123/456", p.parse_dot().unwrap().to_str().unwrap());
    /// }
    /// ```
    ///
    /// ```
    /// extern crate path_dedot;
    ///
    /// use std::path::Path;
    ///
    /// use path_dedot::*;
    ///
    /// if cfg!(not(windows)) {
    ///     let p = Path::new("/path/to/../../../../123/456/./777/..");
    ///
    ///     assert_eq!("/123/456", p.parse_dot().unwrap().to_str().unwrap());
    /// }
    /// ```
    fn parse_dot(&self) -> io::Result<PathBuf>;
}
