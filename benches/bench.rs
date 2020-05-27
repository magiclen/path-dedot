extern crate path_dedot;

#[macro_use]
extern crate bencher;

use path_dedot::ParseDot;

use std::path::Path;

use bencher::Bencher;

fn no_dots(bencher: &mut Bencher) {
    #[cfg(feature = "unsafe_cache")]
    unsafe {
        path_dedot::update_cwd()
    };

    let path = Path::new("path/to/123/456");

    bencher.iter(|| path.parse_dot());
}

fn starts_with_a_single_dot(bencher: &mut Bencher) {
    #[cfg(feature = "unsafe_cache")]
    unsafe {
        path_dedot::update_cwd()
    };

    let path = Path::new("./path/to/123/456");

    bencher.iter(|| path.parse_dot());
}

fn starts_with_double_dots(bencher: &mut Bencher) {
    #[cfg(feature = "unsafe_cache")]
    unsafe {
        path_dedot::update_cwd()
    };

    let path = Path::new("../path/to/123/456");

    bencher.iter(|| path.parse_dot());
}

fn mix(bencher: &mut Bencher) {
    #[cfg(feature = "unsafe_cache")]
    unsafe {
        path_dedot::update_cwd()
    };

    let path = Path::new("./path/to/123/../456");

    bencher.iter(|| path.parse_dot());
}

benchmark_group!(bench_group, no_dots, starts_with_a_single_dot, starts_with_double_dots, mix);
benchmark_main!(bench_group);
