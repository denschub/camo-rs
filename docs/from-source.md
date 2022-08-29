# Building `camo-rs` from source

To build `camo-rs` from source, start by cloning this repository. The `develop` branch, which is the default, always includes the latest - but potentially unstable - sources. The `release` branch contains the code of the latest stable version.

When having a local checkout, run

```
cargo install --locked --path . --root ./out
```

and wait until that completes. After that, you will find the `camo` binary inside the `out` directory. Check the [documentation on using the pre-built binaries](/docs/binaries.md) for instructions on what to do with those binaries!
