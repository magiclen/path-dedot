#![cfg(all(unix, not(feature = "unsafe_cache")))]

use std::{env, path::Path};

use path_dedot::ParseDot;

#[test]
fn dedot_lv0_1() {
    let p = Path::new("./path/to/123/456");

    assert_eq!(
        Path::join(env::current_dir().unwrap().as_path(), Path::new("path/to/123/456"))
            .to_str()
            .unwrap(),
        p.parse_dot().unwrap().to_str().unwrap()
    );
}

#[test]
fn dedot_lv0_2() {
    let p = Path::new("../path/to/123/456");

    let cwd = env::current_dir().unwrap();

    let cwd_parent = cwd.parent();

    match cwd_parent {
        Some(cwd_parent) => {
            assert_eq!(
                cwd_parent.join("path/to/123/456").to_str().unwrap(),
                p.parse_dot().unwrap().to_str().unwrap()
            );
        },
        None => {
            assert_eq!(
                Path::join(Path::new("/"), Path::new("path/to/123/456")).to_str().unwrap(),
                p.parse_dot().unwrap().to_str().unwrap()
            );
        },
    }
}

#[test]
fn dedot_lv1() {
    let p = Path::new("/path/to/123/456/./777");

    assert_eq!("/path/to/123/456/777", p.parse_dot().unwrap().to_str().unwrap());
}

#[test]
fn dedot_lv2() {
    let p = Path::new("/path/to/123/456/../777");

    assert_eq!("/path/to/123/777", p.parse_dot().unwrap().to_str().unwrap());
}

#[test]
fn dedot_lv3() {
    let p = Path::new("/path/to/../123/456/./777");

    assert_eq!("/path/123/456/777", p.parse_dot().unwrap().to_str().unwrap());
}

#[test]
fn dedot_lv4() {
    let p = Path::new("/path/to/../123/456/./777/..");

    assert_eq!("/path/123/456", p.parse_dot().unwrap().to_str().unwrap());
}

#[test]
fn dedot_lv5() {
    let p = Path::new("path/to/../123/456/./777/..");

    assert_eq!("path/123/456", p.parse_dot().unwrap().to_str().unwrap());
}

#[test]
fn dedot_lv6() {
    let p = Path::new("path/to/../../../../123/456/./777/..");

    assert_eq!("123/456", p.parse_dot().unwrap().to_str().unwrap());
}

#[test]
fn dedot_lv7() {
    let p = Path::new("/path/to/../../../../123/456/./777/..");

    assert_eq!("/123/456", p.parse_dot().unwrap().to_str().unwrap());
}
