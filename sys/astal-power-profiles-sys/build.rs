// Generated by gir (https://github.com/gtk-rs/gir @ 74c0d5542b5c)
// from ../../gir/gir-files (@ 1783d05ebac3+)
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
