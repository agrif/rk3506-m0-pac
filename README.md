RK3506 M0 Peripheral Access Crate
=================================

Rust PAC for the RK3506 M0 MCU. Work in progress.

~~~~
# install tools and toolchain
cargo install cargo-make@^0.37.24 form@^0.13.0 svdtools@^0.5.0
cargo install --git https://github.com/embassy-rs/chiptool --locked --rev 6a8c2aa
rustup target add thumbv6m-none-eabi

# generate svd, build, and document
cargo make
~~~~
