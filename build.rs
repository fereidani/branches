use rustc_version::{version_meta, Channel, Version};

fn main() {
    match version_meta() {
        Ok(meta) => {
            if meta.channel == Channel::Nightly {
                println!("cargo:rustc-cfg=branches_nightly");
            } else {
                println!("cargo:rustc-cfg=branches_stable");
            }
            if meta.semver >= Version::parse("1.81.0").unwrap() {
                println!("cargo:rustc-cfg=rustc_ge_1_81_0");
            }
            if meta.semver >= Version::parse("1.95.0").unwrap() {
                println!("cargo:rustc-cfg=rustc_ge_1_95_0");
            }
        }
        // If the compiler version cannot be detected, emitting no cfg at all
        // would make the crate fail to compile with confusing type errors.
        // Fall back to the most conservative stable code path instead.
        Err(_) => println!("cargo:rustc-cfg=branches_stable"),
    }
}
