use std::{
    borrow::Cow,
    ffi::{OsStr, OsString},
    io,
    path::{Component, Path, PathBuf, PrefixComponent, MAIN_SEPARATOR_STR},
};

use crate::ParseDot;

impl ParseDot for Path {
    fn parse_dot(&self) -> io::Result<Cow<'_, Path>> {
        let mut iter = self.components();

        let needs_cwd = match iter.next() {
            Some(Component::CurDir | Component::ParentDir) => true,
            Some(first_component @ Component::Prefix(_)) => match iter.next() {
                Some(Component::CurDir | Component::ParentDir) => true,
                Some(Component::Normal(_)) => {
                    // a path like `C:.\path` does not produce a `CurDir` component, so the raw string needs to be checked
                    self.as_os_str().to_str().is_some_and(|path_str| {
                        path_str[first_component.as_os_str().len()..].starts_with(r".\")
                    })
                },
                _ => false,
            },
            _ => false,
        };

        if needs_cwd {
            let cwd = get_cwd!();

            Ok(self.parse_dot_from(cwd))
        } else {
            // the cwd is only used when the path starts with a single dot or double dots, so an empty path is enough here
            Ok(self.parse_dot_from(Path::new("")))
        }
    }

    fn parse_dot_from(&self, cwd: impl AsRef<Path>) -> Cow<'_, Path> {
        let main_separator = OsStr::new(MAIN_SEPARATOR_STR);

        let mut iter = self.components();

        let mut has_dots = false;

        if let Some(first_component) = iter.next() {
            let mut tokens = Vec::new();

            let (has_prefix, first_is_root) = match first_component {
                Component::Prefix(prefix) => {
                    tokens.push(prefix.as_os_str());

                    if let Some(second_component) = iter.next() {
                        match second_component {
                            Component::RootDir => {
                                tokens.push(main_separator);

                                (true, true)
                            },
                            Component::CurDir => {
                                // may be unreachable

                                has_dots = true;

                                let cwd = cwd.as_ref();

                                for token in cwd.iter().skip(if cwd.get_path_prefix().is_some() {
                                    1
                                } else {
                                    0
                                }) {
                                    tokens.push(token);
                                }

                                (true, tokens.len() > 1 && tokens[1] == main_separator)
                            },
                            Component::ParentDir => {
                                has_dots = true;

                                let cwd = cwd.as_ref();

                                match cwd.parent() {
                                    Some(cwd_parent) => {
                                        for token in cwd_parent.iter().skip(
                                            if cwd.get_path_prefix().is_some() { 1 } else { 0 },
                                        ) {
                                            tokens.push(token);
                                        }

                                        (true, tokens.len() > 1 && tokens[1] == main_separator)
                                    },
                                    None => {
                                        if cwd.get_path_prefix().is_some() {
                                            if cwd.is_absolute() {
                                                tokens.push(main_separator);

                                                (true, true)
                                            } else {
                                                (true, false)
                                            }
                                        } else {
                                            // don't care about `cwd` is "\\" or "\\\"
                                            if cwd == main_separator {
                                                tokens.push(main_separator);

                                                (true, true)
                                            } else {
                                                (true, false)
                                            }
                                        }
                                    },
                                }
                            },
                            _ => {
                                let path = self.as_os_str().as_encoded_bytes();

                                if path[first_component.as_os_str().len()..].starts_with(br".\") {
                                    has_dots = true;

                                    let out =
                                        {
                                            let cwd = cwd.as_ref();

                                            for token in cwd.iter().skip(
                                                if cwd.get_path_prefix().is_some() { 1 } else { 0 },
                                            ) {
                                                tokens.push(token);
                                            }

                                            (true, tokens.len() > 1 && tokens[1] == main_separator)
                                        };

                                    tokens.push(second_component.as_os_str());

                                    out
                                } else {
                                    tokens.push(second_component.as_os_str());

                                    (true, false)
                                }
                            },
                        }
                    } else {
                        (true, false)
                    }
                },
                Component::RootDir => {
                    tokens.push(main_separator);

                    (false, true)
                },
                Component::CurDir => {
                    has_dots = true;

                    let cwd = cwd.as_ref();

                    for token in cwd.iter() {
                        tokens.push(token);
                    }

                    if cwd.get_path_prefix().is_some() {
                        (true, tokens.len() > 1 && tokens[1] == main_separator)
                    } else {
                        (false, !tokens.is_empty() && tokens[0] == main_separator)
                    }
                },
                Component::ParentDir => {
                    has_dots = true;

                    let cwd = cwd.as_ref();

                    match cwd.parent() {
                        Some(cwd_parent) => {
                            for token in cwd_parent.iter() {
                                tokens.push(token);
                            }

                            if cwd.get_path_prefix().is_some() {
                                (true, tokens.len() > 1 && tokens[1] == main_separator)
                            } else {
                                (false, !tokens.is_empty() && tokens[0] == main_separator)
                            }
                        },
                        None => match cwd.get_path_prefix() {
                            Some(prefix) => {
                                tokens.push(prefix.as_os_str());

                                if cwd.is_absolute() {
                                    tokens.push(main_separator);

                                    (true, true)
                                } else {
                                    (true, false)
                                }
                            },
                            None => {
                                // don't care about `cwd` is "\\" or "\\\"
                                if cwd == main_separator {
                                    tokens.push(main_separator);

                                    (false, true)
                                } else {
                                    (false, false)
                                }
                            },
                        },
                    }
                },
                Component::Normal(token) => {
                    tokens.push(token);

                    (false, false)
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

                        if tokens_length > 0
                            && ((tokens_length != 1 || (!first_is_root && !has_prefix))
                                && (tokens_length != 2 || !(first_is_root && has_prefix)))
                        {
                            tokens.pop();
                        }

                        has_dots = true;
                    },
                    _ => {
                        tokens.push(component.as_os_str());
                    },
                }
            }

            // all tokens can be removed by double dots (e.g. `a\..\..`), and in this case the result is an empty path
            if tokens.is_empty() {
                return Cow::from(PathBuf::new());
            }

            let tokens_length = tokens.len();

            let mut size = tokens.iter().fold(tokens_length - 1, |acc, &x| acc + x.len());

            if has_prefix {
                if tokens_length > 1 {
                    size -= 1;

                    if first_is_root {
                        if tokens_length > 2 {
                            size -= 1;
                        } else if tokens[0].len() == self.as_os_str().len() {
                            // tokens_length == 2
                            // e.g.
                            // `\\server\share\` -> `\\server\share\`
                            // `\\server\share` -> `\\server\share\` should still be `\\server\share`
                            return Cow::from(self);
                        }
                    }
                }
            } else if first_is_root && tokens_length > 1 {
                size -= 1;
            }

            if has_dots || size != self.as_os_str().len() {
                let mut path_string = OsString::with_capacity(size);

                let mut iter = tokens.iter();

                path_string.push(iter.next().unwrap());

                if tokens_length > 1 {
                    if has_prefix {
                        if let Some(token) = iter.next() {
                            path_string.push(token);

                            if tokens_length > 2 {
                                if !first_is_root {
                                    path_string.push(main_separator);
                                }

                                for token in iter.take(tokens_length - 3) {
                                    path_string.push(token);

                                    path_string.push(main_separator);
                                }

                                path_string.push(tokens[tokens_length - 1]);
                            }
                        }
                    } else {
                        if !first_is_root {
                            path_string.push(main_separator);
                        }

                        for token in iter.take(tokens_length - 2) {
                            path_string.push(token);

                            path_string.push(main_separator);
                        }

                        path_string.push(tokens[tokens_length - 1]);
                    }
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

pub trait ParsePrefix {
    fn get_path_prefix(&self) -> Option<PrefixComponent<'_>>;
}

impl ParsePrefix for Path {
    #[inline]
    fn get_path_prefix(&self) -> Option<PrefixComponent<'_>> {
        match self.components().next() {
            Some(Component::Prefix(prefix_component)) => Some(prefix_component),
            _ => None,
        }
    }
}

impl ParsePrefix for PathBuf {
    #[inline]
    fn get_path_prefix(&self) -> Option<PrefixComponent<'_>> {
        self.as_path().get_path_prefix()
    }
}
