#![cfg(not(windows))]

extern crate path_dedot;

use std::path::Path;

use path_dedot::{CWD, ParseDot};

#[test]
fn dedot_lv0_1() {
    let p = Path::new("./path/to/123/456");

    assert_eq!(Path::join(&CWD, Path::new("path/to/123/456")).to_str().unwrap(), p.parse_dot().unwrap().to_str().unwrap());
}

#[test]
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
fn dedot_lv1() {
    let p = Path::new("/path/to/../123/456/./777");

    assert_eq!("/path/123/456/777", p.parse_dot().unwrap().to_str().unwrap());
}

#[test]
fn dedot_lv2() {
    let p = Path::new("/path/to/../123/456/./777/..");

    assert_eq!("/path/123/456", p.parse_dot().unwrap().to_str().unwrap());
}

#[test]
fn dedot_lv3() {
    let p = Path::new("path/to/../123/456/./777/..");

    assert_eq!("path/123/456", p.parse_dot().unwrap().to_str().unwrap());
}

#[test]
fn dedot_lv4() {
    let p = Path::new("path/to/../../../../123/456/./777/..");

    assert_eq!("123/456", p.parse_dot().unwrap().to_str().unwrap());
}

#[test]
fn dedot_lv5() {
    let p = Path::new("/path/to/../../../../123/456/./777/..");

    assert_eq!("/123/456", p.parse_dot().unwrap().to_str().unwrap());
}