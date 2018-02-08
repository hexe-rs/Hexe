extern crate version_check;

fn main() {
    if let Some(true) = version_check::supports_features() {
        println!("cargo:rustc-cfg=nightly");
    }
}
