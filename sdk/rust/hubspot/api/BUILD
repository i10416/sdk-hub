
load("@rules_rust//crate_universe:defs.bzl", "crates_vendor")
crates_vendor(
    name = "crates_vendor",
    mode = "local",
    cargo_lockfile = ":Cargo.lock",
    manifests = [":Cargo.toml"],
)
load("//sdk/rust/hubspot/api/crates:defs.bzl", "all_crate_deps")
load("@rules_rust//cargo:defs.bzl", "cargo_build_script")

cargo_build_script(
    name = "build",
    srcs = [
        "build.rs",
    ],
    data = [
        "codegen.toml", # Replace it with output file
        "simple.json",
        "//spec/hubspot:build"
    ],
    deps = all_crate_deps(
        normal = True,
        build = True,
    ) + [
    ],
    crate_root = "build.rs",
)
