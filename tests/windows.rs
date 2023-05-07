#![cfg(all(windows, not(feature = "unsafe_cache")))]

use std::{env, path::Path};

use path_dedot::{ParseDot, ParsePrefix};

#[test]
fn dedot_lv0_1() {
    let p = Path::new(r".\path\to\123\456");

    assert_eq!(
        Path::join(env::current_dir().unwrap().as_path(), Path::new(r"path\to\123\456"))
            .to_str()
            .unwrap(),
        p.parse_dot().unwrap().to_str().unwrap()
    );
}

#[test]
fn dedot_lv0_2() {
    let p = Path::new(r"..\path\to\123\456");

    let cwd = env::current_dir().unwrap();

    let cwd_parent = cwd.parent();

    match cwd_parent {
        Some(cwd_parent) => {
            assert_eq!(
                Path::join(&cwd_parent, Path::new(r"path\to\123\456")).to_str().unwrap(),
                p.parse_dot().unwrap().to_str().unwrap()
            );
        },
        None => {
            assert_eq!(
                Path::join(
                    Path::new(cwd.get_path_prefix().unwrap().as_os_str()),
                    Path::new(r"\path\to\123\456"),
                )
                .to_str()
                .unwrap(),
                p.parse_dot().unwrap().to_str().unwrap()
            );
        },
    }
}

#[test]
fn dedot_lv0_3() {
    let cwd = env::current_dir().unwrap();

    let prefix = cwd.get_path_prefix().unwrap();

    let p = Path::join(Path::new(prefix.as_os_str()), Path::new(r".\path\to\123\456"));

    assert_eq!(
        Path::join(&cwd, Path::new(r"path\to\123\456")).to_str().unwrap(),
        p.parse_dot().unwrap().to_str().unwrap()
    );
}

#[test]
fn dedot_lv0_4() {
    let cwd = env::current_dir().unwrap();

    let prefix = cwd.get_path_prefix().unwrap();

    let p = Path::join(Path::new(prefix.as_os_str()), Path::new(r"..\path\to\123\456"));

    let cwd_parent = cwd.parent();

    match cwd_parent {
        Some(cwd_parent) => {
            assert_eq!(
                Path::join(&cwd_parent, Path::new(r"path\to\123\456")).to_str().unwrap(),
                p.parse_dot().unwrap().to_str().unwrap()
            );
        },
        None => {
            assert_eq!(
                Path::join(
                    Path::new(cwd.get_path_prefix().unwrap().as_os_str()),
                    Path::new(r"\path\to\123\456"),
                )
                .to_str()
                .unwrap(),
                p.parse_dot().unwrap().to_str().unwrap()
            );
        },
    }
}

#[test]
fn dedot_lv1_1() {
    let p = Path::new(r"\path\to\123\456\.\777");

    assert_eq!(r"\path\to\123\456\777", p.parse_dot().unwrap().to_str().unwrap());
}

#[test]
fn dedot_lv1_2() {
    let p = Path::new(r"C:\path\to\123\456\.\777");

    assert_eq!(r"C:\path\to\123\456\777", p.parse_dot().unwrap().to_str().unwrap());
}

#[test]
fn dedot_lv2_1() {
    let p = Path::new(r"\path\to\123\456\..\777");

    assert_eq!(r"\path\to\123\777", p.parse_dot().unwrap().to_str().unwrap());
}

#[test]
fn dedot_lv2_2() {
    let p = Path::new(r"C:\path\to\123\456\..\777");

    assert_eq!(r"C:\path\to\123\777", p.parse_dot().unwrap().to_str().unwrap());
}

#[test]
fn dedot_lv3_1() {
    let p = Path::new(r"\path\to\..\123\456\.\777");

    assert_eq!(r"\path\123\456\777", p.parse_dot().unwrap().to_str().unwrap());
}

#[test]
fn dedot_lv3_2() {
    let p = Path::new(r"C:\path\to\..\123\456\.\777");

    assert_eq!(r"C:\path\123\456\777", p.parse_dot().unwrap().to_str().unwrap());
}

#[test]
fn dedot_lv4_1() {
    let p = Path::new(r"\path\to\..\123\456\.\777\..");

    assert_eq!(r"\path\123\456", p.parse_dot().unwrap().to_str().unwrap());
}

#[test]
fn dedot_lv4_2() {
    let p = Path::new(r"C:\path\to\..\123\456\.\777\..");

    assert_eq!(r"C:\path\123\456", p.parse_dot().unwrap().to_str().unwrap());
}

#[test]
fn dedot_lv5_1() {
    let p = Path::new(r"path\to\..\123\456\.\777\..");

    assert_eq!(r"path\123\456", p.parse_dot().unwrap().to_str().unwrap());
}

#[test]
fn dedot_lv5_2() {
    let p = Path::new(r"C:path\to\..\123\456\.\777\..");

    assert_eq!(r"C:path\123\456", p.parse_dot().unwrap().to_str().unwrap());
}

#[test]
fn dedot_lv6_1() {
    let p = Path::new(r"path\to\..\..\..\..\123\456\.\777\..");

    assert_eq!(r"123\456", p.parse_dot().unwrap().to_str().unwrap());
}

#[test]
fn dedot_lv6_2() {
    let p = Path::new(r"C:path\to\..\..\..\..\123\456\.\777\..");

    assert_eq!(r"C:123\456", p.parse_dot().unwrap().to_str().unwrap());
}

#[test]
fn dedot_lv7_1() {
    let p = Path::new(r"\path\to\..\..\..\..\123\456\.\777\..");

    assert_eq!(r"\123\456", p.parse_dot().unwrap().to_str().unwrap());
}

#[test]
fn dedot_lv7_2() {
    let p = Path::new(r"C:\path\to\..\..\..\..\123\456\.\777\..");

    assert_eq!(r"C:\123\456", p.parse_dot().unwrap().to_str().unwrap());
}

#[test]
fn dedot_lv8_1() {
    let p = Path::new(r"C:\");

    assert_eq!(r"C:\", p.parse_dot_from(r"\foo\bar\baz").unwrap().to_str().unwrap());
    assert_eq!(r"C:\", p.parse_dot_from(r"foo\bar\baz").unwrap().to_str().unwrap());
}

#[test]
fn dedot_lv8_2() {
    let p = Path::new(r"C:");

    assert_eq!(r"C:", p.parse_dot_from(r"\foo\bar\baz").unwrap().to_str().unwrap());
    assert_eq!(r"C:", p.parse_dot_from(r"foo\bar\baz").unwrap().to_str().unwrap());
}

#[test]
fn dedot_lv8_3() {
    let p = Path::new(r"");

    assert_eq!(r"", p.parse_dot_from(r"\foo\bar\baz").unwrap().to_str().unwrap());
    assert_eq!(r"", p.parse_dot_from(r"foo\bar\baz").unwrap().to_str().unwrap());
}

#[test]
fn dedot_lv8_4() {
    let p = Path::new(r"abc");

    assert_eq!(r"abc", p.parse_dot_from(r"\foo\bar\baz").unwrap().to_str().unwrap());
    assert_eq!(r"abc", p.parse_dot_from(r"foo\bar\baz").unwrap().to_str().unwrap());
}

#[test]
fn dedot_lv9_1() {
    let p = Path::new(r".\abc");

    assert_eq!(r"\abc", p.parse_dot_from(r"\").unwrap().to_str().unwrap());
    assert_eq!("abc", p.parse_dot_from("").unwrap().to_str().unwrap());

    assert_eq!(r"C:\abc", p.parse_dot_from(r"C:\").unwrap().to_str().unwrap());
    assert_eq!("C:abc", p.parse_dot_from("C:").unwrap().to_str().unwrap());
}

#[test]
fn dedot_lv9_2() {
    let p = Path::new(r"..\abc");

    assert_eq!(r"\abc", p.parse_dot_from(r"\").unwrap().to_str().unwrap());
    assert_eq!("abc", p.parse_dot_from("").unwrap().to_str().unwrap());

    assert_eq!(r"C:\abc", p.parse_dot_from(r"C:\").unwrap().to_str().unwrap());
    assert_eq!("C:abc", p.parse_dot_from("C:").unwrap().to_str().unwrap());
}

#[test]
fn dedot_lv9_3() {
    let p = Path::new(r".\abc");

    assert_eq!(r"\foo\bar\baz\abc", p.parse_dot_from(r"\foo\bar\baz").unwrap().to_str().unwrap());
    assert_eq!(r"foo\bar\baz\abc", p.parse_dot_from(r"foo\bar\baz").unwrap().to_str().unwrap());

    assert_eq!(
        r"C:\foo\bar\baz\abc",
        p.parse_dot_from(r"C:\foo\bar\baz").unwrap().to_str().unwrap()
    );
    assert_eq!(r"C:foo\bar\baz\abc", p.parse_dot_from(r"C:foo\bar\baz").unwrap().to_str().unwrap());
}

#[test]
fn dedot_lv9_4() {
    let p = Path::new(r"..\abc");

    assert_eq!(r"\foo\bar\abc", p.parse_dot_from(r"\foo\bar\baz").unwrap().to_str().unwrap());
    assert_eq!(r"foo\bar\abc", p.parse_dot_from(r"foo\bar\baz").unwrap().to_str().unwrap());

    assert_eq!(r"C:\foo\bar\abc", p.parse_dot_from(r"C:\foo\bar\baz").unwrap().to_str().unwrap());
    assert_eq!(r"C:foo\bar\abc", p.parse_dot_from(r"C:foo\bar\baz").unwrap().to_str().unwrap());
}

#[test]
fn dedot_lv9_5() {
    let p = Path::new(r"C:.\abc");

    assert_eq!(r"C:\abc", p.parse_dot_from(r"\").unwrap().to_str().unwrap());
    assert_eq!("C:abc", p.parse_dot_from("").unwrap().to_str().unwrap());

    assert_eq!(r"C:\abc", p.parse_dot_from(r"C:\").unwrap().to_str().unwrap());
    assert_eq!("C:abc", p.parse_dot_from("C:").unwrap().to_str().unwrap());
}

#[test]
fn dedot_lv9_6() {
    let p = Path::new(r"C:..\abc");

    assert_eq!(r"C:\abc", p.parse_dot_from(r"\").unwrap().to_str().unwrap());
    assert_eq!("C:abc", p.parse_dot_from("").unwrap().to_str().unwrap());

    assert_eq!(r"C:\abc", p.parse_dot_from(r"C:\").unwrap().to_str().unwrap());
    assert_eq!("C:abc", p.parse_dot_from("C:").unwrap().to_str().unwrap());
}

#[test]
fn dedot_lv9_7() {
    let p = Path::new(r"C:.\abc");

    assert_eq!(r"C:\foo\bar\baz\abc", p.parse_dot_from(r"\foo\bar\baz").unwrap().to_str().unwrap());
    assert_eq!(r"C:foo\bar\baz\abc", p.parse_dot_from(r"foo\bar\baz").unwrap().to_str().unwrap());

    assert_eq!(
        r"C:\foo\bar\baz\abc",
        p.parse_dot_from(r"C:\foo\bar\baz").unwrap().to_str().unwrap()
    );
    assert_eq!(r"C:foo\bar\baz\abc", p.parse_dot_from(r"C:foo\bar\baz").unwrap().to_str().unwrap());
}

#[test]
fn dedot_lv9_8() {
    let p = Path::new(r"C:..\abc");

    assert_eq!(r"C:\foo\bar\abc", p.parse_dot_from(r"\foo\bar\baz").unwrap().to_str().unwrap());
    assert_eq!(r"C:foo\bar\abc", p.parse_dot_from(r"foo\bar\baz").unwrap().to_str().unwrap());

    assert_eq!(r"C:\foo\bar\abc", p.parse_dot_from(r"C:\foo\bar\baz").unwrap().to_str().unwrap());
    assert_eq!(r"C:foo\bar\abc", p.parse_dot_from(r"C:foo\bar\baz").unwrap().to_str().unwrap());
}

#[test]
fn prefix_1() {
    let p = Path::new(r"C:\");

    assert_eq!(r"C:\", p.parse_dot().unwrap().to_str().unwrap());
}

#[test]
fn prefix_2() {
    let p = Path::new(r"C:");

    assert_eq!(r"C:", p.parse_dot().unwrap().to_str().unwrap());
}

#[test]
fn prefix_3() {
    let p = Path::new(r"\\VBOXSRV\test");

    assert_eq!(r"\\VBOXSRV\test", p.parse_dot().unwrap().to_str().unwrap());
}

#[test]
fn prefix_4() {
    let p = Path::new(r"\\VBOXSRV\test\");

    assert_eq!(r"\\VBOXSRV\test\", p.parse_dot().unwrap().to_str().unwrap());
}
