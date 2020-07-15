# `f3` Madgwick Example Visualization

This is the visualization application for `../examples/madgwick.rs` to run on
your host.


## Building

The `f3` crate already configures its build target to `thumbv7em-none-eabihf`
and so we have to explicitly specify it when building and running the
visualization in place.

The most convenient way is likely to edit `./.cargo/config` to select your host
platform.

Alternatively you can explicitly specify it when building and running the
application. For example for Linux x86\_64:
```
$ cargo build --target x86_64-unknown-linux-gnu
$ cargo run --target x86_64-unknown-linux-gnu
```
