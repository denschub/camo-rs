# Building `camo-rs` from source

To build `camo-rs` from source, start by cloning this repository. The `main` branch always includes the latest - but potentially unstable - sources. Check out the tags to get the stable releases.

When having a local checkout, run

```
cargo install --locked --path . --root ./out
```

and wait until that completes. After that, you will find the `camo` binary inside the `out` directory. Check the [documentation on using the pre-built binaries](/docs/binaries.md) for instructions on what to do with those binaries!
