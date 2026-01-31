# RK3506 M0 Peripheral Access Crate

[![Build and Test](https://github.com/agrif/rk3506-m0-pac/actions/workflows/build-test.yaml/badge.svg)](https://github.com/agrif/rk3506-m0-pac/actions/workflows/build-test.yaml)

Rust PAC for the RK3506 M0 MCU. Work in progress.

 * [Crate Documentation](https://agrif.github.io/rk3506-m0-pac/rk3506_m0_pac/)
 * [Memory Map (html)](https://agrif.github.io/rk3506-m0-pac/memorymap/RK3506%20M0.html)
 * [Memory Map (txt)](https://agrif.github.io/rk3506-m0-pac/memorymap.txt)
 * [SVD File](https://agrif.github.io/rk3506-m0-pac/rk3506-m0.svd)

## Install and Build

~~~~
# install tools and toolchain
cargo install cargo-make@^0.37.24 form@^0.13.0 svdtools@^0.5.0
cargo install --git https://github.com/embassy-rs/chiptool --locked --rev 6a8c2aa
rustup target add thumbv6m-none-eabi

# generate svd, build, and document
cargo make
~~~~

## License

Licensed under the [MIT License](LICENSE). Unless stated otherwise,
any contributions to this work will also be licensed this way, with no
additional terms or conditions.
