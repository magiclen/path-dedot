use std::ffi::OsString;
use std::io::{self, ErrorKind};
use std::path::{Component, Path, PathBuf, PrefixComponent};

use crate::{ParseDot, MAIN_SEPARATOR};

impl ParseDot for Path {
    #[allow(clippy::let_unit_value)]
    fn parse_dot(&self) -> io::Result<PathBuf> {
        let mut size = self.as_os_str().len();

        let mut iter = self.components();

        if let Some(first_component) = iter.next() {
            let cwd = get_cwd!();

            let mut tokens = Vec::new();

            let (has_prefix, first_is_root) = match first_component {
                Component::Prefix(prefix) => {
                    tokens.push(prefix.as_os_str());

                    if let Some(second_component) = iter.next() {
                        match second_component {
                            Component::RootDir => {
                                tokens.push(MAIN_SEPARATOR.as_os_str());

                                (true, true)
                            }
                            Component::CurDir => {
                                // may be unreachable

                                for token in cwd.iter().skip(1) {
                                    tokens.push(token);
                                    size += token.len();
                                }

                                size -= 1;

                                (true, false)
                            }
                            Component::ParentDir => {
                                match cwd.parent() {
                                    Some(cwd_parent) => {
                                        for token in cwd_parent.iter().skip(1) {
                                            tokens.push(token);
                                            size += token.len();
                                        }
                                    }
                                    None => {
                                        tokens.push(MAIN_SEPARATOR.as_os_str());
                                    }
                                }

                                size -= 2;

                                (true, true)
                            }
                            _ => {
                                let path_str = self.as_os_str().to_str().ok_or_else(|| {
                                    io::Error::new(ErrorKind::Other, "The path is not valid UTF-8.")
                                })?;

                                if path_str[first_component.as_os_str().len()..].starts_with('.') {
                                    for token in cwd.iter().skip(1) {
                                        tokens.push(token);
                                        size += token.len();
                                    }

                                    size -= 1;

                                    tokens.push(second_component.as_os_str());

                                    (true, true)
                                } else {
                                    tokens.push(second_component.as_os_str());

                                    (true, false)
                                }
                            }
                        }
                    } else {
                        (true, false)
                    }
                }
                Component::RootDir => {
                    tokens.push(MAIN_SEPARATOR.as_os_str());

                    (false, true)
                }
                Component::CurDir => {
                    for token in cwd.iter() {
                        tokens.push(token);
                    }

                    size += cwd.as_os_str().len() - 1;

                    (true, true)
                }
                Component::ParentDir => {
                    match cwd.parent() {
                        Some(cwd_parent) => {
                            for token in cwd_parent.iter() {
                                tokens.push(token);
                            }

                            size += cwd_parent.as_os_str().len();
                        }
                        None => {
                            let prefix = cwd.get_path_prefix().unwrap().as_os_str();
                            tokens.push(prefix);
                            size += prefix.len();

                            tokens.push(MAIN_SEPARATOR.as_os_str());
                        }
                    }

                    size -= 2;

                    (true, true)
                }
                Component::Normal(token) => {
                    tokens.push(token);

                    (false, false)
                }
            };

            for component in iter {
                match component {
                    Component::CurDir => {
                        // may be unreachable

                        size -= 2;
                    }
                    Component::ParentDir => {
                        let tokens_length = tokens.len();

                        if tokens_length > 0
                            && ((tokens_length != 1 || (!first_is_root && !has_prefix))
                                && (tokens_length != 2 || !(first_is_root && has_prefix)))
                        {
                            let removed = tokens.remove(tokens_length - 1);
                            size -= removed.len() + 4; // xxx\..\
                        } else {
                            size -= 3; // ..\
                        }
                    }
                    _ => {
                        tokens.push(component.as_os_str());
                    }
                }
            }

            debug_assert!(!tokens.is_empty());

            let mut path_string = OsString::with_capacity(size);

            let mut iter = tokens.iter();

            path_string.push(iter.next().unwrap());

            let tokens_length = tokens.len();

            if tokens_length > 1 {
                if has_prefix {
                    if let Some(token) = iter.next() {
                        path_string.push(token);

                        if tokens_length > 2 {
                            if !first_is_root {
                                path_string.push(MAIN_SEPARATOR.as_os_str());
                            }

                            for &token in iter.take(tokens_length - 3) {
                                path_string.push(token);

                                path_string.push(MAIN_SEPARATOR.as_os_str());
                            }

                            path_string.push(tokens[tokens_length - 1]);
                        }
                    }
                } else {
                    if !first_is_root {
                        path_string.push(MAIN_SEPARATOR.as_os_str());
                    }

                    for &token in iter.take(tokens_length - 2) {
                        path_string.push(token);

                        path_string.push(MAIN_SEPARATOR.as_os_str());
                    }

                    path_string.push(tokens[tokens_length - 1]);
                }
            }

            debug_assert!(size + 1 >= path_string.len()); // + 1 is for `\\server\share` -> `\\server\share\`

            let path_buf = PathBuf::from(path_string);

            Ok(path_buf)
        } else {
            Ok(PathBuf::new())
        }
    }
}

pub trait ParsePrefix {
    fn get_path_prefix(&self) -> Option<PrefixComponent>;
}

impl ParsePrefix for Path {
    #[inline]
    fn get_path_prefix(&self) -> Option<PrefixComponent> {
        let first_component = self.components().next();

        match first_component.unwrap() {
            Component::Prefix(prefix_component) => Some(prefix_component),
            _ => None,
        }
    }
}

impl ParsePrefix for PathBuf {
    #[inline]
    fn get_path_prefix(&self) -> Option<PrefixComponent> {
        let first_component = self.components().next();

        match first_component.unwrap() {
            Component::Prefix(prefix_component) => Some(prefix_component),
            _ => None,
        }
    }
}
