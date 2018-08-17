use std::path::{self, Path, PathBuf};
use std::io;
use std::env;
use std::ffi::OsString;

#[macro_use]
extern crate lazy_static;

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
    /// let p = Path::new("./path/to/123/456");
    ///
    /// assert_eq!(Path::join(&CWD, Path::new("path/to/123/456")).to_str().unwrap(), p.parse_dot().unwrap().to_str().unwrap());
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
    /// let p = Path::new("../path/to/123/456");
    ///
    /// let cwd_parent = CWD.parent();
    ///
    /// match cwd_parent {
    ///    Some(cwd_parent) => {
    ///       assert_eq!(Path::join(&cwd_parent, Path::new("path/to/123/456")).to_str().unwrap(), p.parse_dot().unwrap().to_str().unwrap());
    ///    }
    ///    None => {
    ///       assert_eq!(Path::join(Path::new("/"), Path::new("path/to/123/456")).to_str().unwrap(), p.parse_dot().unwrap().to_str().unwrap());
    ///    }
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
    /// let p = Path::new("/path/to/../123/456/./777");
    ///
    /// assert_eq!("/path/123/456/777", p.parse_dot().unwrap().to_str().unwrap());
    /// ```
    ///
    /// ```
    /// extern crate path_dedot;
    ///
    /// use std::path::Path;
    ///
    /// use path_dedot::*;
    ///
    /// let p = Path::new("/path/to/../123/456/./777/..");
    ///
    /// assert_eq!("/path/123/456", p.parse_dot().unwrap().to_str().unwrap());
    /// ```
    ///
    /// You should notice that `parse_dot` method does **not** aim to get an **absolute path**. For those paths which do not start with **/**, **Single Dot** and **Double Dots** are still do not have each of them after the `parse_dot` method is used.
    ///
    /// ```
    /// extern crate path_dedot;
    ///
    /// use std::path::Path;
    ///
    /// use path_dedot::*;
    ///
    /// let p = Path::new("path/to/../123/456/./777/..");
    ///
    /// assert_eq!("path/123/456", p.parse_dot().unwrap().to_str().unwrap());
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
    /// let p = Path::new("path/to/../../../../123/456/./777/..");
    ///
    /// assert_eq!("123/456", p.parse_dot().unwrap().to_str().unwrap());
    /// ```
    ///
    /// ```
    /// extern crate path_dedot;
    ///
    /// use std::path::Path;
    ///
    /// use path_dedot::*;
    ///
    /// let p = Path::new("/path/to/../../../../123/456/./777/..");
    ///
    /// assert_eq!("/123/456", p.parse_dot().unwrap().to_str().unwrap());
    /// ```
    fn parse_dot(&self) -> io::Result<PathBuf>;
}

impl ParseDot for Path {
    fn parse_dot(&self) -> io::Result<PathBuf> {
        let mut tokens = Vec::new();

        for (index, token) in self.iter().enumerate() {
            if token.eq(".") {
                if index == 0 {
                    for token in CWD.iter() {
                        tokens.push(token);
                    }
                }
            } else if token.eq("..") {
                let len = tokens.len();
                if index == 0 {
                    let cwd_parent = CWD.parent();

                    match cwd_parent {
                        Some(cwd_parent) => {
                            for token in cwd_parent.iter() {
                                tokens.push(token);
                            }
                        }
                        None => {
                            tokens.push(MAIN_SEPARATOR.as_os_str());
                        }
                    }
                } else if len > 0 && (len != 1 || tokens[0].ne("/")) {
                    tokens.remove(len - 1);
                }
            } else {
                tokens.push(token);
            }
        }

        let mut path = OsString::new();

        let len = tokens.len();

        if len > 0 {
            let first_token = tokens[0];
            path.push(first_token);

            if len > 1 {
                if !first_token.eq("/") {
                    path.push(MAIN_SEPARATOR.as_os_str());
                }

                for &token in tokens.iter().skip(1).take(len - 2) {
                    path.push(token);

                    path.push(MAIN_SEPARATOR.as_os_str());
                }

                path.push(tokens[len - 1]);
            }
        }

        let path_buf = PathBuf::from(&path);

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
    #[cfg(not(windows))]
    fn dedot_lv1() {
        let p = Path::new("/path/to/../123/456/./777");

        assert_eq!("/path/123/456/777", p.parse_dot().unwrap().to_str().unwrap());
    }

    #[test]
    #[cfg(not(windows))]
    fn dedot_lv2() {
        let p = Path::new("/path/to/../123/456/./777/..");

        assert_eq!("/path/123/456", p.parse_dot().unwrap().to_str().unwrap());
    }

    #[test]
    #[cfg(not(windows))]
    fn dedot_lv3() {
        let p = Path::new("path/to/../123/456/./777/..");

        assert_eq!("path/123/456", p.parse_dot().unwrap().to_str().unwrap());
    }

    #[test]
    #[cfg(not(windows))]
    fn dedot_lv4() {
        let p = Path::new("path/to/../../../../123/456/./777/..");

        assert_eq!("123/456", p.parse_dot().unwrap().to_str().unwrap());
    }

    #[test]
    #[cfg(not(windows))]
    fn dedot_lv5() {
        let p = Path::new("/path/to/../../../../123/456/./777/..");

        assert_eq!("/123/456", p.parse_dot().unwrap().to_str().unwrap());
    }
}
