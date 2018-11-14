#![cfg(windows)]

extern crate path_dedot;

use std::path::Path;

use path_dedot::{CWD, ParseDot};

#[test]
fn dedot_lv0_1() {
    let p = Path::new(r".\path\to\123\456");

    assert_eq!(Path::join(&CWD, Path::new(r"path\to\123\456")).to_str().unwrap(), p.parse_dot().unwrap().to_str().unwrap());
}

#[test]
fn dedot_lv0_2() {
    let p = Path::new(r"..\path\to\123\456");

    let cwd_parent = CWD.parent();

    match cwd_parent {
        Some(cwd_parent) => {
            assert_eq!(Path::join(&cwd_parent, Path::new(r"path\to\123\456")).to_str().unwrap(), p.parse_dot().unwrap().to_str().unwrap());
        }
        None => {
            assert_eq!(Path::join(Path::new(CWD.get_path_prefix().unwrap().as_os_str()), Path::new(r"\path\to\123\456")).to_str().unwrap(), p.parse_dot().unwrap().to_str().unwrap());
        }
    }
}

#[test]
#[ignore]
// Ignored because it may not be standard
fn dedot_lv0_3() {
    let prefix = CWD.get_path_prefix().unwrap();

    let p = Path::join(Path::new(prefix.as_os_str()), Path::new(r".\path\to\123\456"));

    assert_eq!(Path::join(&CWD, Path::new(r"path\to\123\456")).to_str().unwrap(), p.parse_dot().unwrap().to_str().unwrap());
}

#[test]
#[ignore]
// Ignored because it may not be standard
fn dedot_lv0_4() {
    let prefix = CWD.get_path_prefix().unwrap();

    let p = Path::join(Path::new(prefix.as_os_str()), Path::new(r"..\path\to\123\456"));

    let cwd_parent = CWD.parent();

    match cwd_parent {
        Some(cwd_parent) => {
            assert_eq!(Path::join(&cwd_parent, Path::new(r"path\to\123\456")).to_str().unwrap(), p.parse_dot().unwrap().to_str().unwrap());
        }
        None => {
            assert_eq!(Path::join(Path::new(CWD.get_path_prefix().unwrap().as_os_str()), Path::new(r"\path\to\123\456")).to_str().unwrap(), p.parse_dot().unwrap().to_str().unwrap());
        }
    }
}

#[test]
fn dedot_lv1_1() {
    let p = Path::new(r"\path\to\..\123\456\.\777");

    assert_eq!(r"\path\123\456\777", p.parse_dot().unwrap().to_str().unwrap());
}

#[test]
fn dedot_lv1_2() {
    let p = Path::new(r"C:\path\to\..\123\456\.\777");

    assert_eq!(r"C:\path\123\456\777", p.parse_dot().unwrap().to_str().unwrap());
}

#[test]
fn dedot_lv2_1() {
    let p = Path::new(r"\path\to\..\123\456\.\777\..");

    assert_eq!(r"\path\123\456", p.parse_dot().unwrap().to_str().unwrap());
}

#[test]
fn dedot_lv2_2() {
    let p = Path::new(r"C:\path\to\..\123\456\.\777\..");

    assert_eq!(r"C:\path\123\456", p.parse_dot().unwrap().to_str().unwrap());
}

#[test]
fn dedot_lv3_1() {
    let p = Path::new(r"path\to\..\123\456\.\777\..");

    assert_eq!(r"path\123\456", p.parse_dot().unwrap().to_str().unwrap());
}

#[test]
fn dedot_lv3_2() {
    let p = Path::new(r"C:path\to\..\123\456\.\777\..");

    assert_eq!(r"C:path\123\456", p.parse_dot().unwrap().to_str().unwrap());
}

#[test]
fn dedot_lv4_1() {
    let p = Path::new(r"path\to\..\..\..\..\123\456\.\777\..");

    assert_eq!(r"123\456", p.parse_dot().unwrap().to_str().unwrap());
}

#[test]
fn dedot_lv4_2() {
    let p = Path::new(r"C:path\to\..\..\..\..\123\456\.\777\..");

    assert_eq!(r"C:123\456", p.parse_dot().unwrap().to_str().unwrap());
}

#[test]
fn dedot_lv5_1() {
    let p = Path::new(r"\path\to\..\..\..\..\123\456\.\777\..");

    assert_eq!(r"\123\456", p.parse_dot().unwrap().to_str().unwrap());
}

#[test]
fn dedot_lv5_2() {
    let p = Path::new(r"C:\path\to\..\..\..\..\123\456\.\777\..");

    assert_eq!(r"C:\123\456", p.parse_dot().unwrap().to_str().unwrap());
}