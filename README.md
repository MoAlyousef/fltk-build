# fltk-build


Allows creating native C/C++ fltk/cfltk modules to be used from Rust. This is done by exposing the include paths and lib paths from the built fltk-sys crate on your system.

## Usage
```toml
[dependencies] # or [dev-dependencies]
fltk = "1.2.17" # this won't work with fltk-bundled
# You might need fltk-sys and paste if you use fltk's trait macros

[build-dependencies]
fltk-build = "0.1"
fltk-sys = "1.2.17"
cc = "1" # or cmake = "0.1"
```

## Example
build.rs (using `cc`):
```rust
use fltk_build::fltk_out_dir;

fn main() {
    let fltk_out_dir = fltk_out_dir().unwrap();

    cc::Build::new()
        .file("src/my_wid.cpp")
        .cpp(true)
        .flag_if_supported("-w")
        .flag_if_supported("-fno-rtti")
        .include(&fltk_out_dir.join("include"))
        .compile("my_wid");
}
```

build.rs (using `cmake-rs`):
```rust
use std::env;
use std::path::PathBuf;
use fltk_build::fltk_out_dir;

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    let fltk_out_dir = fltk_out_dir().unwrap();

    cmake::Config::new("src")
    .define(
        "CMAKE_CXX_STANDARD_INCLUDE_DIRECTORIES",
        fltk_out_dir.join("include"),
    )
    .build();

    println!(
        "cargo:rustc-link-search=native={}",
        out_dir.display()
    );
    println!("cargo:rustc-link-lib=static=my_wid");

    match target_os.as_str() {
        "windows" => (), // doesn't need explicit linking to the C++ stdlib
        "macos" => println!("cargo:rustc-link-lib=c++"),
        _ => println!("cargo:rustc-link-lib=stdc++"),
    }
}
```

In your C/C++ files, you can directly access the FLTK and cfltk headers:
```c++
#include <cfl/cfl_window.h>
#include <FL/Fl_Window.H>
```

If you're using CMake, a minimal CMakeLists.txt example:
```cmake
cmake_minimum_required(VERSION 3.0)
project(wid)

if (NOT MSVC)
    set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -fno-rtti -fno-exceptions")
endif()

add_library(my_wid my_wid.cpp)

install(TARGETS my_wid DESTINATION ${CMAKE_INSTALL_PREFIX})
```

Example crate (not published on crates.io) using fltk-build:
https://github.com/fltk-rs/fltk-flow


