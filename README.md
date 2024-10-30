# NDS RS

Rust support for the Nintendo DS console family

## Project structure

### [nds-sys](nds-sys)

[libnds](https://github.com/blocksds/libnds) library bindings.

### [picolibc](picolibc)

wf-toolchain uses a custom libc implementation as such we can't rely on the libc crate.

Here we include our own bindings to the [wf-picolibc](https://github.com/WonderfulToolchain/wf-picolibc)

### [nds-proc-macros](nds-proc-macros)

Helper crate that hosts procedural macros

### [bitfield-tools](bitfield-tools)

Functions related to bit manipulation
