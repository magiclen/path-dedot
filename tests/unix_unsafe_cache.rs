#![cfg(all(unix, feature = "unsafe_cache"))]

use std::{env, path::Path};

use path_dedot::{update_cwd, ParseDot};

#[test]
fn dedot_after_updating_cwd() {
    unsafe {
        update_cwd();
    }

    let p = Path::new("./path/to/123/456");

    assert_eq!(
        Path::join(env::current_dir().unwrap().as_path(), Path::new("path/to/123/456"))
            .to_str()
            .unwrap(),
        p.parse_dot().unwrap().to_str().unwrap()
    );

    env::set_current_dir("/").unwrap();

    unsafe {
        update_cwd();
    }

    assert_eq!(
        Path::join(env::current_dir().unwrap().as_path(), Path::new("path/to/123/456"))
            .to_str()
            .unwrap(),
        p.parse_dot().unwrap().to_str().unwrap()
    );
}
