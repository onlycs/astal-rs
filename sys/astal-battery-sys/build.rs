// Generated by gir (https://github.com/gtk-rs/gir @ 26e721588f2f)
// from ../../gir/gir-files (@ e02639f55a90)
// DO NOT EDIT

#[cfg(not(docsrs))]
use std::process;

#[cfg(docsrs)]
fn main() {} // prevent linking libraries to avoid documentation failure

#[cfg(not(docsrs))]
fn main() {
    if let Err(s) = system_deps::Config::new().probe() {
        println!("cargo:warning={s}");
        process::exit(1);
    }
}
