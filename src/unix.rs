use std::borrow::Cow;
use std::ffi::OsString;
use std::io;
use std::path::{Component, Path, PathBuf};

use crate::{ParseDot, MAIN_SEPARATOR};

impl ParseDot for Path {
    fn parse_dot(&self) -> io::Result<Cow<Path>> {
        let mut size = self.as_os_str().len();

        let mut iter = self.components();

        let mut has_dots = false;

        if let Some(first_component) = iter.next() {
            let cwd = get_cwd!();

            let mut tokens = Vec::new();

            let first_is_root = match first_component {
                Component::RootDir => {
                    tokens.push(MAIN_SEPARATOR.as_os_str());

                    true
                }
                Component::CurDir => {
                    for token in cwd.iter() {
                        tokens.push(token);
                    }

                    size += cwd.as_os_str().len() - 1;

                    has_dots = true;

                    true
                }
                Component::ParentDir => {
                    match cwd.parent() {
                        Some(cwd_parent) => {
                            for token in cwd_parent.iter() {
                                tokens.push(token);
                            }

                            size += cwd_parent.as_os_str().len();
                            size -= 2;
                        }
                        None => {
                            tokens.push(MAIN_SEPARATOR.as_os_str());
                            size -= 1;
                        }
                    }

                    has_dots = true;

                    true
                }
                _ => {
                    tokens.push(first_component.as_os_str());

                    false
                }
            };

            for component in iter {
                match component {
                    Component::CurDir => {
                        // may be unreachable
                        size -= 2;

                        has_dots = true;
                    }
                    Component::ParentDir => {
                        let tokens_length = tokens.len();

                        if tokens_length > 0 && (tokens_length != 1 || !first_is_root) {
                            let removed = tokens.remove(tokens_length - 1);
                            size -= removed.len() + 4; // xxx/../
                        } else {
                            size -= 3; // ../
                        }

                        has_dots = true;
                    }
                    _ => {
                        tokens.push(component.as_os_str());
                    }
                }
            }

            debug_assert!(!tokens.is_empty());

            if has_dots {
                let mut path_string = OsString::with_capacity(size);

                let mut iter = tokens.iter();

                path_string.push(iter.next().unwrap());

                let tokens_length = tokens.len();

                if tokens_length > 1 {
                    if !first_is_root {
                        path_string.push(MAIN_SEPARATOR.as_os_str());
                    }

                    for token in iter.take(tokens_length - 2) {
                        path_string.push(token);

                        path_string.push(MAIN_SEPARATOR.as_os_str());
                    }

                    path_string.push(tokens[tokens_length - 1]);
                }

                debug_assert!(size >= path_string.len());

                let path_buf = PathBuf::from(path_string);

                Ok(Cow::from(path_buf))
            } else {
                Ok(Cow::from(self))
            }
        } else {
            Ok(Cow::from(self))
        }
    }
}
