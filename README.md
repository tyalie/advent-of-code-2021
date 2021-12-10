# Advent of Code 2021 

This will be developed in Rust on a [Teensy 4.1](https://www.pjrc.com/store/teensy41.html) 
with an ARM Cortex-M7.

Solutions are stored in `src/bin`.

## Setup
It is really recommended to go through the Dependencies section of the [`teensy4-rs`
crate](https://github.com/mciantyre/teensy4-rs/blob/master/README.md#dependencies).

The following commands require the `cargo-binutils` package with llvm installed
```bash
# In order to build the code and debug compile time errors one can use:
cargo build --release --bin test 

# To build & translate the output to a hex file use
# Note: The hex will be generated relative to the location of invocation
cargo objcopy --release --bin test -- -O ihex out.hex

# Upload the hex using (requires putting the Teesny into boot mode)
teensy_loader_cli --mcu=TEENSY41 -w out.hex
```

