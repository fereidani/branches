use rustc_version::{version_meta, Channel, Version};

fn main() {
    if let Ok(meta) = version_meta() {
        if meta.channel == Channel::Nightly {
            println!("cargo:rustc-cfg=branches_nightly");
        } else {
            println!("cargo:rustc-cfg=branches_stable");
        }
        if meta.semver >= Version::parse("1.81.0").unwrap() {
            println!("cargo:rustc-cfg=rustc_ge_1_81_0");
        }
    }
}
