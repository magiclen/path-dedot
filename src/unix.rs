use std::{
    borrow::Cow,
    ffi::{OsStr, OsString},
    io,
    path::{Component, Path, PathBuf, MAIN_SEPARATOR_STR},
};

use crate::ParseDot;

impl ParseDot for Path {
    #[inline]
    fn parse_dot(&self) -> io::Result<Cow<'_, Path>> {
        match self.components().next() {
            Some(Component::CurDir | Component::ParentDir) => {
                let cwd = get_cwd!();

                Ok(self.parse_dot_from(cwd))
            },
            // the cwd is only used when the path starts with a single dot or double dots, so an empty path is enough here
            _ => Ok(self.parse_dot_from(Path::new(""))),
        }
    }

    fn parse_dot_from(&self, cwd: impl AsRef<Path>) -> Cow<'_, Path> {
        let main_separator = OsStr::new(MAIN_SEPARATOR_STR);

        let mut iter = self.components();

        let mut has_dots = false;

        if let Some(first_component) = iter.next() {
            let mut tokens = Vec::new();

            let first_is_root = match first_component {
                Component::RootDir => {
                    tokens.push(main_separator);

                    true
                },
                Component::CurDir => {
                    has_dots = true;

                    let cwd = cwd.as_ref();

                    for token in cwd.iter() {
                        tokens.push(token);
                    }

                    !tokens.is_empty() && tokens[0] == main_separator
                },
                Component::ParentDir => {
                    has_dots = true;

                    let cwd = cwd.as_ref();

                    match cwd.parent() {
                        Some(cwd_parent) => {
                            for token in cwd_parent.iter() {
                                tokens.push(token);
                            }

                            !tokens.is_empty() && tokens[0] == main_separator
                        },
                        None => {
                            // don't care about `cwd` is "//" or "///"
                            if cwd == main_separator {
                                tokens.push(main_separator);

                                true
                            } else {
                                false
                            }
                        },
                    }
                },
                _ => {
                    tokens.push(first_component.as_os_str());

                    false
                },
            };

            for component in iter {
                match component {
                    Component::CurDir => {
                        // may be unreachable
                        has_dots = true;
                    },
                    Component::ParentDir => {
                        let tokens_length = tokens.len();

                        if tokens_length > 0 && (tokens_length != 1 || !first_is_root) {
                            tokens.pop();
                        }

                        has_dots = true;
                    },
                    _ => {
                        tokens.push(component.as_os_str());
                    },
                }
            }

            // all tokens can be removed by double dots (e.g. `a/../..`), and in this case the result is an empty path
            if tokens.is_empty() {
                return Cow::from(PathBuf::new());
            }

            let tokens_length = tokens.len();

            let mut size = tokens.iter().fold(tokens_length - 1, |acc, &x| acc + x.len());

            if first_is_root && tokens_length > 1 {
                size -= 1;
            }

            if has_dots || size != self.as_os_str().len() {
                let mut path_string = OsString::with_capacity(size);

                let mut iter = tokens.iter();

                path_string.push(iter.next().unwrap());

                if tokens_length > 1 {
                    if !first_is_root {
                        path_string.push(main_separator);
                    }

                    for token in iter.take(tokens_length - 2) {
                        path_string.push(token);

                        path_string.push(main_separator);
                    }

                    path_string.push(tokens[tokens_length - 1]);
                }

                let path_buf = PathBuf::from(path_string);

                Cow::from(path_buf)
            } else {
                Cow::from(self)
            }
        } else {
            Cow::from(self)
        }
    }
}
