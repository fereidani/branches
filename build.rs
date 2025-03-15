use rustc_version::{version_meta, Channel};

fn main() {
    if let Ok(meta) = version_meta() {
        if meta.channel == Channel::Nightly {
            println!("cargo:rustc-cfg=unstable");
        }
    }
}
