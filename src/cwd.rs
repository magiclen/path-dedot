use std::env;
use std::ops::Deref;
use std::path::{Path, PathBuf};

/// Current working directory.
#[doc(hidden)]
pub struct CWD {
    path: Option<PathBuf>,
}

impl CWD {
    #[inline]
    pub(crate) const fn new() -> CWD {
        CWD {
            path: None,
        }
    }

    #[inline]
    pub(crate) fn update(&mut self) {
        let cwd = env::current_dir().unwrap();

        self.path.replace(cwd);
    }

    #[inline]
    #[doc(hidden)]
    pub fn initial(&mut self) {
        if self.path.is_none() {
            self.update();
        }
    }
}

impl Deref for CWD {
    type Target = Path;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.path.as_ref().unwrap().as_path()
    }
}
