#![cfg(all(windows, feature = "unsafe_cache"))]

use std::env;
use std::path::Path;

use path_dedot::{update_cwd, ParseDot, ParsePrefix};

#[test]
fn dedot_after_updating_cwd() {
    unsafe {
        update_cwd();
    }

    let p = Path::new(r".\path\to\123\456");

    assert_eq!(
        Path::join(env::current_dir().unwrap().as_path(), Path::new(r"path\to\123\456"))
            .to_str()
            .unwrap(),
        p.parse_dot().unwrap().to_str().unwrap()
    );

    let cwd = env::current_dir().unwrap();

    let prefix = cwd.get_path_prefix().unwrap();

    env::set_current_dir(Path::new(prefix.as_os_str())).unwrap();

    unsafe {
        update_cwd();
    }

    assert_eq!(
        Path::join(env::current_dir().unwrap().as_path(), Path::new(r"path\to\123\456"))
            .to_str()
            .unwrap(),
        p.parse_dot().unwrap().to_str().unwrap()
    );
}
