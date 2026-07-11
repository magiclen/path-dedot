Path Dedot
====================

[![CI](https://github.com/magiclen/path-dedot/actions/workflows/ci.yml/badge.svg)](https://github.com/magiclen/path-dedot/actions/workflows/ci.yml)

This is a library for extending `Path` and `PathBuf` in order to parse the path which contains dots.

Please read the following examples to know the parsing rules.

## Examples

If a path starts with a single dot, the dot means your program's **current working directory** (CWD).

```rust
use std::path::Path;
use std::env;

use path_dedot::*;

let p = Path::new("./path/to/123/456");

assert_eq!(Path::join(env::current_dir().unwrap().as_path(), Path::new("path/to/123/456")).to_str().unwrap(), p.parse_dot().unwrap().to_str().unwrap());
```

If a path starts with a pair of dots, the dots means the parent of the CWD. If the CWD is **root**, the parent is still **root**.

```rust
use std::path::Path;
use std::env;

use path_dedot::*;

let p = Path::new("../path/to/123/456");

let cwd = env::current_dir().unwrap();

let cwd_parent = cwd.parent();

match cwd_parent {
   Some(cwd_parent) => {
      assert_eq!(Path::join(&cwd_parent, Path::new("path/to/123/456")).to_str().unwrap(), p.parse_dot().unwrap().to_str().unwrap());
   }
   None => {
      assert_eq!(Path::join(Path::new("/"), Path::new("path/to/123/456")).to_str().unwrap(), p.parse_dot().unwrap().to_str().unwrap());
   }
}
```

In addition to starting with, the **Single Dot** and **Double Dots** can also be placed to other positions. **Single Dot** means noting and will be ignored. **Double Dots** means the parent.

```rust
use std::path::Path;

use path_dedot::*;

let p = Path::new("/path/to/../123/456/./777");

assert_eq!("/path/123/456/777", p.parse_dot().unwrap().to_str().unwrap());
```

```rust
use std::path::Path;

use path_dedot::*;

let p = Path::new("/path/to/../123/456/./777/..");

assert_eq!("/path/123/456", p.parse_dot().unwrap().to_str().unwrap());
```

You should notice that `parse_dot` method does **not** aim to get an **absolute path**. A path which does not start with a `MAIN_SEPARATOR`, **Single Dot** and **Double Dots**, will not have each of them after the `parse_dot` method is used.

```rust
use std::path::Path;

use path_dedot::*;

let p = Path::new("path/to/../123/456/./777/..");

assert_eq!("path/123/456", p.parse_dot().unwrap().to_str().unwrap());
```

**Double Dots** which is not placed at the start cannot get the parent beyond the original path. Why not? With this constraint, you can insert an absolute path to the start as a virtual root in order to protect your file system from being exposed.

```rust
use std::path::Path;

use path_dedot::*;

let p = Path::new("path/to/../../../../123/456/./777/..");

assert_eq!("123/456", p.parse_dot().unwrap().to_str().unwrap());
```

```rust
use std::path::Path;

use path_dedot::*;

let p = Path::new("/path/to/../../../../123/456/./777/..");

assert_eq!("/123/456", p.parse_dot().unwrap().to_str().unwrap());
```

If a path is entirely consumed by its **Double Dots**, an empty path will be returned.

```rust
use std::path::Path;

use path_dedot::*;

let p = Path::new("path/to/../../..");

assert_eq!("", p.parse_dot().unwrap().to_str().unwrap());
```

### Starting from a given current working directory

With the `parse_dot_from` function, you can provide the current working directory that the relative paths should be resolved from.

```rust
use std::env;
use std::path::Path;

use path_dedot::*;

let p = Path::new("../path/to/123/456");
let cwd = env::current_dir().unwrap();

println!("{}", p.parse_dot_from(cwd).to_str().unwrap());
```

## Caching

The `parse_dot` method requires the CWD only when the input path starts with a **Single Dot** or **Double Dots**, and by default, it creates a new `PathBuf` instance of the CWD every time in its operation. The overhead is obvious. Although it allows us to safely change the CWD at runtime by the program itself (e.g. using the `std::env::set_current_dir` function) or outside controls (e.g. using gdb to call `chdir`), we don't need that in most cases.

In order to parse paths with better performance, the `fixed_workdir` feature can be enabled.

```toml
[dependencies.path-dedot]
version = "*"
features = ["fixed_workdir"]
```

## Benchmark

#### No-cache

```bash
cargo bench
```

#### fixed_workdir

```bash
cargo bench --features fixed_workdir
```

## Crates.io

https://crates.io/crates/path-dedot

## Documentation

https://docs.rs/path-dedot

## License

[MIT](LICENSE)