#[macro_use]
extern crate lazy_static;

use std::path::{self, Path, PathBuf};
use std::io;
use std::env;
use std::ffi::OsString;

#[cfg(windows)]
use std::path::{Component, PrefixComponent};

lazy_static! {
    /// Current working directory.
    pub static ref CWD: PathBuf = {
        env::current_dir().unwrap()
    };

    /// The main separator for the target OS.
    pub static ref MAIN_SEPARATOR: OsString = {
        OsString::from(path::MAIN_SEPARATOR.to_string())
    };
}

#[cfg(windows)]
#[doc(hidden)]
pub trait ParsePrefix {
    #[cfg(windows)]
    fn get_path_prefix(&self) -> Option<PrefixComponent>;
}

#[cfg(windows)]
impl ParsePrefix for Path {
    fn get_path_prefix(&self) -> Option<PrefixComponent> {
        let first_component = self.components().next();

        match first_component.unwrap() {
            Component::Prefix(prefix_component) => Some(prefix_component),
            _ => None,
        }
    }
}

#[cfg(windows)]
impl ParsePrefix for PathBuf {
    fn get_path_prefix(&self) -> Option<PrefixComponent> {
        let first_component = self.components().next();

        match first_component.unwrap() {
            Component::Prefix(prefix_component) => Some(prefix_component),
            _ => None,
        }
    }
}

/// Make `Path` and `PathBuf` have `parse_dot` method.
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
    /// use std::path::Path;
    ///
    /// use path_dedot::*;
    ///
    /// if cfg!(not(windows)) {
    ///
    ///     let p = Path::new("./path/to/123/456");
    ///
    ///     assert_eq!(Path::join(&CWD, Path::new("path/to/123/456")).to_str().unwrap(), p.parse_dot().unwrap().to_str().unwrap());
    ///
    /// }
    /// ```
    ///
    /// If a path starts with a pair of dots, the dots means the parent of **current working directory**. If **current working directory** is **root**, the parent is still **root**.
    ///
    /// ```
    /// extern crate path_dedot;
    ///
    /// use std::path::Path;
    ///
    /// use path_dedot::*;
    ///
    /// if cfg!(not(windows)) {
    ///
    ///     let p = Path::new("../path/to/123/456");
    ///
    ///     let cwd_parent = CWD.parent();
    ///
    ///     match cwd_parent {
    ///        Some(cwd_parent) => {
    ///       assert_eq!(Path::join(&cwd_parent, Path::new("path/to/123/456")).to_str().unwrap(), p.parse_dot().unwrap().to_str().unwrap());
    ///        }
    ///        None => {
    ///           assert_eq!(Path::join(Path::new("/"), Path::new("path/to/123/456")).to_str().unwrap(), p.parse_dot().unwrap().to_str().unwrap());
    ///        }
    ///     }
    ///
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
    ///
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
    ///
    ///     let p = Path::new("/path/to/../123/456/./777/..");
    ///
    ///     assert_eq!("/path/123/456", p.parse_dot().unwrap().to_str().unwrap());
    ///
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
    ///
    ///     let p = Path::new("path/to/../123/456/./777/..");
    ///
    ///     assert_eq!("path/123/456", p.parse_dot().unwrap().to_str().unwrap());
    ///
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
    ///
    ///     let p = Path::new("path/to/../../../../123/456/./777/..");
    ///
    ///     assert_eq!("123/456", p.parse_dot().unwrap().to_str().unwrap());
    ///
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
    ///
    ///     let p = Path::new("/path/to/../../../../123/456/./777/..");
    ///
    ///     assert_eq!("/123/456", p.parse_dot().unwrap().to_str().unwrap());
    ///
    /// }
    /// ```
    fn parse_dot(&self) -> io::Result<PathBuf>;
}

impl ParseDot for Path {
    #[cfg(not(windows))]
    fn parse_dot(&self) -> io::Result<PathBuf> {
        let mut size = self.as_os_str().len();

        let mut tokens = Vec::new();

        let mut iter = self.iter();

        if let Some(first_token) = iter.next() {
            if first_token.eq(".") {
                for token in CWD.iter() {
                    tokens.push(token);
                }
                size += CWD.as_os_str().len() - 1;
            } else if first_token.eq("..") {
                let cwd_parent = CWD.parent();

                match cwd_parent {
                    Some(cwd_parent) => {
                        for token in cwd_parent.iter() {
                            tokens.push(token);
                        }
                        size += cwd_parent.as_os_str().len() - 2;
                    }
                    None => {
                        tokens.push(MAIN_SEPARATOR.as_os_str());
                        size -= 2;
                    }
                }
            } else {
                tokens.push(first_token);
            }

            for token in iter {
//                if token.eq(".") {
//                    size -= 2;
//                    continue;
//                } else
                // Don't need to check single dot. It is already filtered.
                if token.eq("..") {
                    let len = tokens.len();
                    if len > 0 && (len != 1 || tokens[0].ne(MAIN_SEPARATOR.as_os_str())) {
                        let removed = tokens.remove(len - 1);
                        size -= removed.len() + 4;
                    } else {
                        size -= 3;
                    }
                } else {
                    tokens.push(token);
                }
            }
        }

        let mut path = OsString::with_capacity(size);

        let len = tokens.len();

        if len > 0 {
            let mut iter = tokens.iter();

            if let Some(first_token) = iter.next() {
                path.push(first_token);

                if len > 1 {
                    if !first_token.eq(&MAIN_SEPARATOR.as_os_str()) {
                        path.push(MAIN_SEPARATOR.as_os_str());
                    }

                    for &token in iter.take(len - 2) {
                        path.push(token);

                        path.push(MAIN_SEPARATOR.as_os_str());
                    }

                    path.push(tokens[len - 1]);
                }
            }
        }

        let path_buf = PathBuf::from(path);

        Ok(path_buf)
    }

    #[cfg(windows)]
    fn parse_dot(&self) -> io::Result<PathBuf> {
        let mut size = self.as_os_str().len();

        let mut tokens = Vec::new();

        let mut iter = self.iter();

        let mut prefix = self.get_path_prefix();

        if let Some(first_token) = iter.next() {
            if first_token.eq(".") {
                prefix = CWD.get_path_prefix();

                for token in CWD.iter() {
                    tokens.push(token);
                }
                size += CWD.as_os_str().len() - 1;
            } else if first_token.eq("..") {
                prefix = CWD.get_path_prefix();

                let cwd_parent = CWD.parent();

                match cwd_parent {
                    Some(cwd_parent) => {
                        for token in cwd_parent.iter() {
                            tokens.push(token);
                        }
                        size += cwd_parent.as_os_str().len() - 1;
                    }
                    None => {
                        let prefix = prefix.unwrap().as_os_str();
                        tokens.push(prefix);
                        tokens.push(MAIN_SEPARATOR.as_os_str());
                        size += prefix.len() - 2;
                    }
                }
            } else {
                tokens.push(first_token);

                if let Some(prefix) = prefix {
                    // single dot is filtered by the iterator
                    let path = self.as_os_str().to_str().unwrap();

                    let prefix_len = prefix.as_os_str().len();
                    let path_len = path.len();

                    if (&path[prefix_len..prefix_len + 1]).eq(".") && (prefix_len + 1 == path_len || (&path[prefix_len + 1..prefix_len + 2]).eq(r"\")) {
                        for token in CWD.iter().skip(1) {
                            tokens.push(token);
                        }
                        size += CWD.as_os_str().len() - 1;
                        size -= CWD.get_path_prefix().unwrap().as_os_str().len();
                    } else if let Some(second_token) = iter.next() {
                        if second_token.eq("..") {
                            let cwd_parent = CWD.parent();

                            match cwd_parent {
                                Some(cwd_parent) => {
                                    for token in cwd_parent.iter().skip(1) {
                                        tokens.push(token);
                                    }
                                    size += cwd_parent.as_os_str().len() - 1;
                                    size -= CWD.get_path_prefix().unwrap().as_os_str().len();
                                }
                                None => {
                                    tokens.push(MAIN_SEPARATOR.as_os_str());
                                    size -= 2;
                                }
                            }
                        } else {
                            tokens.push(second_token);
                        }
                    }
                }
            }

            if prefix.is_some() {
                for token in iter {
//                  if token.eq(".") {
//                      size -= 2;
//                      continue;
//                  } else
                    // Don't need to check single dot. It is already filtered.
                    if token.eq("..") {
                        let len = tokens.len();

                        if len > 1 && (len != 2 || tokens[1].ne(MAIN_SEPARATOR.as_os_str())) {
                            let removed = tokens.remove(len - 1);
                            size -= removed.len() + 4;
                        } else {
                            size -= 3;
                        }
                    } else {
                        tokens.push(token);
                    }
                }
            } else {
                for token in iter {
//                  if token.eq(".") {
//                      size -= 2;
//                      continue;
//                  } else
                    // Don't need to check single dot. It is already filtered.
                    if token.eq("..") {
                        let len = tokens.len();

                        if len > 0 && (len != 1 || tokens[0].ne(MAIN_SEPARATOR.as_os_str())) {
                            let removed = tokens.remove(len - 1);
                            size -= removed.len() + 4;
                        } else {
                            size -= 3;
                        }
                    } else {
                        tokens.push(token);
                    }
                }
            }
        }

        let mut path = OsString::with_capacity(size);

        let len = tokens.len();

        if len > 0 {
            let mut iter = tokens.iter();

            if let Some(first_token) = iter.next() {
                path.push(first_token);

                if len > 1 {
                    if prefix.is_some() {
                        if let Some(second_token) = iter.next() {
                            path.push(second_token);

                            if !second_token.eq(&MAIN_SEPARATOR.as_os_str()) {
                                path.push(MAIN_SEPARATOR.as_os_str());
                            }

                            for &token in iter.take(len - 3) {
                                path.push(token);

                                path.push(MAIN_SEPARATOR.as_os_str());
                            }
                        }
                    } else {
                        if !first_token.eq(&MAIN_SEPARATOR.as_os_str()) {
                            path.push(MAIN_SEPARATOR.as_os_str());
                        }

                        for &token in iter.take(len - 2) {
                            path.push(token);

                            path.push(MAIN_SEPARATOR.as_os_str());
                        }
                    }

                    path.push(tokens[len - 1]);
                }
            }
        }

        let path_buf = PathBuf::from(path);

        Ok(path_buf)
    }
}

impl ParseDot for PathBuf {
    fn parse_dot(&self) -> io::Result<PathBuf> {
        let path = Path::new(&self);

        path.parse_dot()
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::*;

    #[test]
    #[cfg(not(windows))]
    fn dedot_lv0_1() {
        let p = Path::new("./path/to/123/456");

        assert_eq!(Path::join(&CWD, Path::new("path/to/123/456")).to_str().unwrap(), p.parse_dot().unwrap().to_str().unwrap());
    }

    #[test]
    #[cfg(windows)]
    fn dedot_lv0_1() {
        let p = Path::new(r".\path\to\123\456");

        assert_eq!(Path::join(&CWD, Path::new(r"path\to\123\456")).to_str().unwrap(), p.parse_dot().unwrap().to_str().unwrap());
    }

    #[test]
    #[cfg(not(windows))]
    fn dedot_lv0_2() {
        let p = Path::new("../path/to/123/456");

        let cwd_parent = CWD.parent();

        match cwd_parent {
            Some(cwd_parent) => {
                assert_eq!(Path::join(&cwd_parent, Path::new("path/to/123/456")).to_str().unwrap(), p.parse_dot().unwrap().to_str().unwrap());
            }
            None => {
                assert_eq!(Path::join(Path::new("/"), Path::new("path/to/123/456")).to_str().unwrap(), p.parse_dot().unwrap().to_str().unwrap());
            }
        }
    }

    #[test]
    #[cfg(windows)]
    fn dedot_lv0_2() {
        let p = Path::new(r"..\path\to\123\456");

        let cwd_parent = CWD.parent();

        match cwd_parent {
            Some(cwd_parent) => {
                assert_eq!(Path::join(&cwd_parent, Path::new(r"path\to\123\456")).to_str().unwrap(), p.parse_dot().unwrap().to_str().unwrap());
            }
            None => {
                assert_eq!(Path::join(Path::new(CWD.get_path_prefix().unwrap().as_os_str()), Path::new(r"\path\to\123\456")).to_str().unwrap(), p.parse_dot().unwrap().to_str().unwrap());
            }
        }
    }

    #[test]
    #[cfg(windows)]
    #[ignore]
    // Ignored because the test needs to be run under the drive C
    fn dedot_lv0_3() {
        let p = Path::new(r"C:.\path\to\123\456");

        assert_eq!(Path::join(&CWD, Path::new(r"path\to\123\456")).to_str().unwrap(), p.parse_dot().unwrap().to_str().unwrap());
    }

    #[test]
    #[cfg(windows)]
    #[ignore]
    // Ignored because the test needs to be run under the drive C
    fn dedot_lv0_4() {
        let p = Path::new(r"C:..\path\to\123\456");

        let cwd_parent = CWD.parent();

        match cwd_parent {
            Some(cwd_parent) => {
                assert_eq!(Path::join(&cwd_parent, Path::new(r"path\to\123\456")).to_str().unwrap(), p.parse_dot().unwrap().to_str().unwrap());
            }
            None => {
                assert_eq!(Path::join(Path::new(CWD.get_path_prefix().unwrap().as_os_str()), Path::new(r"\path\to\123\456")).to_str().unwrap(), p.parse_dot().unwrap().to_str().unwrap());
            }
        }
    }

    #[test]
    #[cfg(not(windows))]
    fn dedot_lv1() {
        let p = Path::new("/path/to/../123/456/./777");

        assert_eq!("/path/123/456/777", p.parse_dot().unwrap().to_str().unwrap());
    }

    #[test]
    #[cfg(windows)]
    fn dedot_lv1_1() {
        let p = Path::new(r"\path\to\..\123\456\.\777");

        assert_eq!(r"\path\123\456\777", p.parse_dot().unwrap().to_str().unwrap());
    }

    #[test]
    #[cfg(windows)]
    fn dedot_lv1_2() {
        let p = Path::new(r"C:\path\to\..\123\456\.\777");

        assert_eq!(r"C:\path\123\456\777", p.parse_dot().unwrap().to_str().unwrap());
    }

    #[test]
    #[cfg(not(windows))]
    fn dedot_lv2() {
        let p = Path::new("/path/to/../123/456/./777/..");

        assert_eq!("/path/123/456", p.parse_dot().unwrap().to_str().unwrap());
    }

    #[test]
    #[cfg(windows)]
    fn dedot_lv2_1() {
        let p = Path::new(r"\path\to\..\123\456\.\777\..");

        assert_eq!(r"\path\123\456", p.parse_dot().unwrap().to_str().unwrap());
    }

    #[test]
    #[cfg(windows)]
    fn dedot_lv2_2() {
        let p = Path::new(r"C:\path\to\..\123\456\.\777\..");

        assert_eq!(r"C:\path\123\456", p.parse_dot().unwrap().to_str().unwrap());
    }

    #[test]
    #[cfg(not(windows))]
    fn dedot_lv3() {
        let p = Path::new("path/to/../123/456/./777/..");

        assert_eq!("path/123/456", p.parse_dot().unwrap().to_str().unwrap());
    }

    #[test]
    #[cfg(windows)]
    fn dedot_lv3_1() {
        let p = Path::new(r"path\to\..\123\456\.\777\..");

        assert_eq!(r"path\123\456", p.parse_dot().unwrap().to_str().unwrap());
    }

    #[test]
    #[cfg(windows)]
    fn dedot_lv3_2() {
        let p = Path::new(r"C:path\to\..\123\456\.\777\..");

        assert_eq!(r"C:path\123\456", p.parse_dot().unwrap().to_str().unwrap());
    }

    #[test]
    #[cfg(not(windows))]
    fn dedot_lv4() {
        let p = Path::new("path/to/../../../../123/456/./777/..");

        assert_eq!("123/456", p.parse_dot().unwrap().to_str().unwrap());
    }

    #[test]
    #[cfg(windows)]
    fn dedot_lv4_1() {
        let p = Path::new(r"path\to\..\..\..\..\123\456\.\777\..");

        assert_eq!(r"123\456", p.parse_dot().unwrap().to_str().unwrap());
    }

    #[test]
    #[cfg(windows)]
    fn dedot_lv4_2() {
        let p = Path::new(r"C:path\to\..\..\..\..\123\456\.\777\..");

        assert_eq!(r"C:123\456", p.parse_dot().unwrap().to_str().unwrap());
    }

    #[test]
    #[cfg(not(windows))]
    fn dedot_lv5() {
        let p = Path::new("/path/to/../../../../123/456/./777/..");

        assert_eq!("/123/456", p.parse_dot().unwrap().to_str().unwrap());
    }

    #[test]
    #[cfg(windows)]
    fn dedot_lv5_1() {
        let p = Path::new(r"\path\to\..\..\..\..\123\456\.\777\..");

        assert_eq!(r"\123\456", p.parse_dot().unwrap().to_str().unwrap());
    }

    #[test]
    #[cfg(windows)]
    fn dedot_lv5_2() {
        let p = Path::new(r"C:\path\to\..\..\..\..\123\456\.\777\..");

        assert_eq!(r"C:\123\456", p.parse_dot().unwrap().to_str().unwrap());
    }
}
