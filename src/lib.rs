use std::env;
use std::fs;
use std::path::PathBuf;

pub fn fltk_out_dir() -> Option<PathBuf> {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let build_dir = out_dir.join("../../");
    for subdir in fs::read_dir(build_dir.clone()).unwrap() {
        let subdir = subdir
            .unwrap()
            .path()
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned();
        if subdir.contains("fltk-sys") {
            let temp = build_dir.join(subdir).join("out");
            if temp.exists() {
                return Some(temp);
            }
        }
    }
    None
}

pub fn link_fltk() {
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    let fltk_out_dir = fltk_out_dir().unwrap();
    println!(
        "cargo:rustc-link-search=native={}",
        fltk_out_dir.join("lib").display()
    );

    for lib in fs::read_dir(fltk_out_dir.join("lib")).unwrap() {
        let lib = lib
            .unwrap()
            .path()
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned();
        if lib.starts_with("lib") {
            println!(
                "cargo:rustc-link-lib=static={}",
                lib.strip_prefix("lib").unwrap()
            );
        } else {
            println!("cargo:rustc-link-lib=static={}", lib);
        }
    }
    println!("cargo:rustc-link-lib=static=cfltk");
    println!("cargo:rustc-link-lib=static=fltk");
    match target_os.as_str() {
        "macos" => {
            println!("cargo:rustc-link-lib=framework=Carbon");
            println!("cargo:rustc-link-lib=framework=Cocoa");
            println!("cargo:rustc-link-lib=framework=ApplicationServices");
        }
        "windows" => {
            println!("cargo:rustc-link-lib=dylib=ws2_32");
            println!("cargo:rustc-link-lib=dylib=comctl32");
            println!("cargo:rustc-link-lib=dylib=gdi32");
            println!("cargo:rustc-link-lib=dylib=oleaut32");
            println!("cargo:rustc-link-lib=dylib=ole32");
            println!("cargo:rustc-link-lib=dylib=uuid");
            println!("cargo:rustc-link-lib=dylib=shell32");
            println!("cargo:rustc-link-lib=dylib=advapi32");
            println!("cargo:rustc-link-lib=dylib=comdlg32");
            println!("cargo:rustc-link-lib=dylib=winspool");
            println!("cargo:rustc-link-lib=dylib=user32");
            println!("cargo:rustc-link-lib=dylib=kernel32");
            println!("cargo:rustc-link-lib=dylib=odbc32");
            if !cfg!(feature = "no-gdiplus") {
                println!("cargo:rustc-link-lib=dylib=gdiplus");
            }
        }
        "android" => {
            println!("cargo:rustc-link-lib=log");
            println!("cargo:rustc-link-lib=android");
            println!("cargo:rustc-link-lib=c++_shared");
        }
        _ => {
            println!("cargo:rustc-link-lib=dylib=pthread");
            println!("cargo:rustc-link-lib=dylib=X11");
            println!("cargo:rustc-link-lib=dylib=Xext");
            println!("cargo:rustc-link-lib=dylib=Xinerama");
            println!("cargo:rustc-link-lib=dylib=Xcursor");
            println!("cargo:rustc-link-lib=dylib=Xrender");
            println!("cargo:rustc-link-lib=dylib=Xfixes");
            println!("cargo:rustc-link-lib=dylib=Xft");
            println!("cargo:rustc-link-lib=dylib=fontconfig");
            if !cfg!(feature = "no-pango") {
                println!("cargo:rustc-link-lib=dylib=pango-1.0");
                println!("cargo:rustc-link-lib=dylib=pangoxft-1.0");
                println!("cargo:rustc-link-lib=dylib=gobject-2.0");
                println!("cargo:rustc-link-lib=dylib=cairo");
                println!("cargo:rustc-link-lib=dylib=pangocairo-1.0");
            }
        }
    }
}
