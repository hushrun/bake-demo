# bake-demo

This repository exercises `hushrun build bake`: its CI builds a small Rust program through hushrun,
bakes the compiled crates into a layered OCI image on GHCR, signs it keylessly with Sigstore, and
boots a container warm from it. See [hushrun/hushrun](https://github.com/hushrun/hushrun).
